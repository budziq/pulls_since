#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate serde_derive;
extern crate reqwest;
extern crate chrono;
extern crate hyper;

use std::fmt;
use chrono::DateTime;
use hyper::header::{Link, RelationType};

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
            client: reqwest::Client::new()?,
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
        let mut response = self.client.get(&url)?.send()?;
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

fn run() -> Result<()> {

    let since = chrono::NaiveDate::parse_from_str("2017/07/14", "%Y/%m/%d").unwrap();

    let request_url = format!(
        "https://api.github.com/repos/{owner}/{repo}/pulls?state=closed",
        owner = "rust-lang-nursery",
        repo = "rust-cookbook"
    );
    println!("{}", request_url);

    for pull in PullList::for_addr(&request_url)? {
        //.filter(|pull| pull.closed_at.date() > since ) {
        let pull = pull?;
        if pull.closed_at.date().naive_utc() > since {
            println!("{}", pull);
        }
    }

    Ok(())
}

quick_main!(run);
