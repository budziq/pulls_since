#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate clap;
extern crate reqwest;
extern crate chrono;
extern crate hyper;

use std::fmt;
use chrono::{DateTime, Datelike, NaiveDate, Local};
use hyper::header::{Link, RelationType};
use clap::{App, Arg, ArgMatches};

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

struct PullList {
    pulls: <Vec<Pull> as IntoIterator>::IntoIter,
    next_link: Option<String>,
    client: reqwest::Client,
}

impl PullList {
    fn for_addr(url: &str) -> Result<Self> {
        Ok(PullList {
            pulls: Vec::new().into_iter(),
            next_link: Some(url.to_owned()),
            client: reqwest::Client::new(),
        })
    }

    fn try_next(&mut self) -> Result<Option<Pull>> {
        if let Some(pull) = self.pulls.next() {
            return Ok(Some(pull));
        }

        if self.next_link.is_none() {
            return Ok(None);
        }

        let url = self.next_link.take().unwrap();
        let mut response = self.client.get(&url).send()?;
        self.pulls = response.json::<Vec<Pull>>()?.into_iter();

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

        Ok(self.pulls.next())
    }
}

impl Iterator for PullList {
    type Item = Result<Pull>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.try_next() {
            Ok(Some(val)) => Some(Ok(val)),
            Ok(None) => None,
            Err(err) => Some(Err(err)),
        }
    }
}

fn app<'a, 'b>() -> App<'a, 'b> {
    let args = vec![
        Arg::with_name("since")
            .short("s")
            .long("since")
            .takes_value(true)
            .required(true)
            .help("date argument dd.mm.yyyy"),
        Arg::with_name("until")
            .short("u")
            .long("until")
            .takes_value(true)
            .help("end date argument dd.mm.yyyy"),
        Arg::with_name("repo")
            .short("r")
            .long("repo")
            .takes_value(true)
            .required(true)
            .help("owner/repo"),
    ];

    App::new("pulls_since")
        .version(crate_version!())
        .about(
            "Print Markdown formatted list of pull requests closed since given date",
        )
        .args(&args)
}

fn parse_date(date: &str) -> Result<NaiveDate> {
    Ok(NaiveDate::parse_from_str(&date, "%Y/%m/%d")
        .or_else(|_| NaiveDate::parse_from_str(&date, "%d.%m.%Y"))
        .or_else(|_| {
            NaiveDate::parse_from_str(&format!("{}.{}", date, Local::now().year()), "%d.%m.%Y")
        })?)
}

fn since<'a>(args: &ArgMatches<'a>) -> Result<NaiveDate> {
    parse_date(args.value_of("since").ok_or("missing `since` argument")?)
}

fn until<'a>(args: &ArgMatches<'a>) -> Result<Option<NaiveDate>> {
    if let Some(until_s) = args.value_of("until") {
        parse_date(until_s).map(Some)
    } else {
        Ok(None)
    }
}

fn url<'a>(args: &ArgMatches<'a>) -> Result<String> {
    let repo = args.value_of("repo").ok_or("missing `repo` argument")?;

    Ok(format!(
        "https://api.github.com/repos/{}/pulls?state=closed",
        repo
    ))
}

fn run() -> Result<()> {
    let args = app().get_matches();

    let since = since(&args)?;
    let until = until(&args)?;
    let url = url(&args)?;

    println!("{}", url);

    for pull in PullList::for_addr(&url)? {
        let pull = pull?;
        let pull_closed = pull.closed_at.date().naive_utc();
        if pull_closed > since && (until.is_none() || pull_closed < *until.as_ref().unwrap()) {
            println!("{}", pull);
        }
    }

    Ok(())
}

quick_main!(run);
