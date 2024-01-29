use reqwest::blocking::Client;
use reqwest::Url;
use scraper::{Html, Selector};
use thiserror::Error;

#[derive(Error, Debug)]
enum Error {
    #[error("request error: {0}")]
    ReqwestError(#[from] reqwest::Error),
    #[error("bad http response: {0}")]
    BadResponse(String),
}

#[derive(Debug)]
struct CrawlCommand {
    url: Url,
    extract_links: bool,
}

fn visit_page(client: &Client, command: &CrawlCommand) -> Result<Vec<Url>, Error> {
    println!("Checking {:#}", command.url);
    let response = client.get(command.url.clone()).send()?;
    if !response.status().is_success() {
        return Err(Error::BadResponse(response.status().to_string()));
    }

    let mut link_urls = Vec::new();
    if !command.extract_links {
        return Ok(link_urls);
    }

    let base_url = response.url().to_owned();
    let body_text = response.text()?;
    let document = Html::parse_document(&body_text);

    let selector = Selector::parse("a").unwrap();
    let href_values = document
        .select(&selector)
        .filter_map(|element| element.value().attr("href"));
    for href in href_values {
        match base_url.join(href) {
            Ok(link_url) => {
                link_urls.push(link_url);
            }
            Err(err) => {
                println!("On {base_url:#}: ignored unparsable {href:?}: {err}");
            }
        }
    }
    Ok(link_urls)
}

static MAX_REQUESTS: usize = 5;

fn main() -> Result<(), Error> {
    let client = Client::new();
    let start_url = Url::parse("https://www.google.at").unwrap();
    let crawl_command = CrawlCommand {
        url: start_url,
        extract_links: true,
    };

    let mut visited_urls = Vec::with_capacity(MAX_REQUESTS);
    let mut to_visit_urls = vec![crawl_command.url.clone()];

    while let Some(url) = to_visit_urls.pop() {
        if visited_urls.len() >= visited_urls.capacity() {
            println!("Max requests reached, stopping.");
            break;
        }
        visited_urls.push(url.clone());
        // send the URLs to be checked to a channel and let a few threads check the URLs in parallel.
        let links = visit_page(&client, &CrawlCommand {
            url: url.clone(),
            extract_links: true,
        })?;
        to_visit_urls.extend(links.into_iter().filter(|link| !visited_urls.contains(link)));
    }
    Ok(())
}
