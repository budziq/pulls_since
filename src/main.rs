#[macro_use]
extern crate error_chain;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate clap;
extern crate dotenv;
extern crate reqwest;
extern crate chrono;
extern crate hyper;

use std::{env, fmt};
use std::collections::BTreeSet;
use chrono::{DateTime, Datelike, NaiveDate, Local};
use hyper::header::{Authorization, Link, RelationType};
use clap::{App, Arg, ArgMatches};
use dotenv::dotenv;
use serde::de::DeserializeOwned;

const EXCLUDE_LOGIN_ARG: &str = "login";
const SINCE_ARG: &str = "since";
const UNTIL_ARG: &str = "until";
const REPOS_ARG: &str = "repo";
const OWNER_ARG: &str = "owner";

#[derive(Deserialize, Debug)]
struct User {
    login: String,
    id: u32,
    // remaining fields not deserialized for brevity
}

#[derive(Deserialize, Debug)]
struct Pull {
    html_url: String,
    title: String,
    user: User,
    closed_at: DateTime<chrono::offset::Utc>,
    // remaining fields not deserialized for brevity
}

#[derive(Deserialize, Debug)]
struct Repo {
    full_name: String,
    // remaining fields not deserialized for brevity
}

impl fmt::Display for Pull {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "- @{} [{}]({})",
            self.user.login,
            self.title,
            self.html_url
        )
    }
}

error_chain! {
    foreign_links {
        Reqwest(reqwest::Error);
        Chrono(chrono::ParseError);
    }
}

struct PaginatedIter<T> {
    items: <Vec<T> as IntoIterator>::IntoIter,
    next_link: Option<String>,
    client: reqwest::Client,
    github_token: Option<String>,
}

impl<T> PaginatedIter<T>
where
    T: DeserializeOwned,
{
    fn for_addr(url: &str) -> Result<Self> {
        Ok(PaginatedIter {
            items: Vec::new().into_iter(),
            next_link: Some(url.to_owned()),
            client: reqwest::Client::new(),
            github_token: env::var("GITHUB_TOKEN").ok(),
        })
    }

    fn try_next(&mut self) -> Result<Option<T>> {
        if let Some(pull) = self.items.next() {
            return Ok(Some(pull));
        }

        if self.next_link.is_none() {
            return Ok(None);
        }

        let url = self.next_link.take().unwrap();
        let mut req = self.client.get(&url);

        if let Some(ref token) = self.github_token {
            req.header(Authorization(format!("token {}", token)));
        }

        let mut response = req.send()?;
        if !response.status().is_success() {
            bail!("Server error: {:?}", response.status());
        }

        self.items = response.json::<Vec<T>>()?.into_iter();

        if let Some(header) = response.headers().get::<Link>() {
            for val in header.values() {
                if val.rel()
                    .map(|rel| rel.contains(&RelationType::Next))
                    .unwrap_or(false)
                {
                    self.next_link = Some(val.link().to_owned());
                    break;
                }
            }
        }

        Ok(self.items.next())
    }
}

impl<T> Iterator for PaginatedIter<T>
where
    T: DeserializeOwned,
{
    type Item = Result<T>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.try_next() {
            Ok(Some(val)) => Some(Ok(val)),
            Ok(None) => None,
            Err(err) => Some(Err(err)),
        }
    }
}

struct Predicate {
    since: Option<NaiveDate>,
    until: Option<NaiveDate>,
    exclude_login: Option<String>,
}

impl Predicate {
    fn from_args<'a>(args: &ArgMatches<'a>) -> Result<Predicate> {
        let exclude_login = args.value_of(EXCLUDE_LOGIN_ARG).map(String::from);

        Ok(Predicate {
            since: date_arg(args, SINCE_ARG)?,
            until: date_arg(args, UNTIL_ARG)?,
            exclude_login: exclude_login,
        })
    }

    fn test(&self, pull: &Pull) -> bool {
        let pull_closed = pull.closed_at.date().naive_utc();
        self.since.map(|v| pull_closed > v).unwrap_or(true) &&
            self.until.map(|v| pull_closed < v).unwrap_or(true) &&
            self.exclude_login
                .as_ref()
                .map(|ex| *ex != pull.user.login)
                .unwrap_or(true)
    }
}

fn app<'a, 'b>() -> App<'a, 'b> {
    let args = vec![
        Arg::with_name(SINCE_ARG)
            .short("s")
            .long("since")
            .takes_value(true)
            .help("start date argument dd.mm.yyyy"),
        Arg::with_name(UNTIL_ARG)
            .short("u")
            .long("until")
            .takes_value(true)
            .help("end date argument dd.mm.yyyy"),
        Arg::with_name(OWNER_ARG)
            .short("o")
            .long("owners")
            .multiple(true)
            .takes_value(true)
            .required_unless(REPOS_ARG)
            .help("space separated list of owners or org names"),
        Arg::with_name(EXCLUDE_LOGIN_ARG)
            .short("e")
            .long("exclude-login")
            .takes_value(true)
            .help("ommit PR's by given login (bots etc.)"),
        Arg::with_name(REPOS_ARG)
            .short("r")
            .long("repos")
            .multiple(true)
            .takes_value(true)
            .required_unless(OWNER_ARG)
            .help("space separated list of 'owner/repo'"),
    ];

    App::new("pulls_since")
        .version(crate_version!())
        .about(
            "Print Markdown formatted list of pull requests closed since given date",
        )
        .args(&args)
}

fn parse_date(date: &str) -> Result<NaiveDate> {
    Ok(NaiveDate::parse_from_str(date, "%Y/%m/%d")
        .or_else(|_| NaiveDate::parse_from_str(date, "%d.%m.%Y"))
        .or_else(|_| {
            NaiveDate::parse_from_str(&format!("{}.{}", date, Local::now().year()), "%d.%m.%Y")
        })?)
}


fn date_arg<'a>(args: &ArgMatches<'a>, key: &str) -> Result<Option<NaiveDate>> {
    match args.value_of(key) {
        Some(key_s) => parse_date(key_s).map(Some),
        None => Ok(None),
    }
}

fn print_pulls_for_repo(repo: &str, pred: &Predicate) -> Result<()> {
    let url = format!("https://api.github.com/repos/{}/pulls?state=closed", repo);

    let mut pulls = PaginatedIter::for_addr(&url)?
        .filter_map(Result::ok)
        .filter(|pull| pred.test(pull))
        .peekable();

    if pulls.peek().is_none() {
        return Ok(());
    }

    println!("\n#### {}\n", repo);

    for pull in pulls {
        println!("{}", pull);
    }
    Ok(())
}

fn repos_for_owner(org: &str) -> Result<Vec<Repo>> {
    let url = format!("https://api.github.com/orgs/{}/repos", org);

    PaginatedIter::for_addr(&url)?
        .collect::<Result<_>>()
        .or_else(|_| {
            let url = format!("https://api.github.com/users/{}/repos", org);
            PaginatedIter::for_addr(&url)?.collect()
        })
}

fn run() -> Result<()> {
    dotenv().ok();

    let args = app().get_matches();
    let pred = Predicate::from_args(&args)?;

    let mut repos = args.values_of(REPOS_ARG)
        .map(|names| names.into_iter().map(String::from).collect())
        .unwrap_or_else(BTreeSet::new);

    if let Some(owners) = args.values_of(OWNER_ARG) {
        repos.extend(
            owners
                .flat_map(|owner| repos_for_owner(owner).into_iter())
                .flat_map(|repos| repos.into_iter().map(|repo| repo.full_name)),
        );
    }

    for repo in repos {
        print_pulls_for_repo(&repo, &pred)?;
    }

    Ok(())
}

quick_main!(run);
