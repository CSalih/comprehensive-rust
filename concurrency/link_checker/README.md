# Multi-threaded Link Checker

Let us use our new knowledge to create a multi-threaded link checker. It should
start at a webpage and check that links on the page are valid. It should
recursively check other pages on the same domain and keep doing this until all
pages have been validated.

For this, you will need an HTTP client such as [`reqwest`][1]. Create a new
Cargo project and `reqwest` it as a dependency with:

```shell
cargo new link-checker
cd link-checker
cargo add --features blocking,rustls-tls reqwest
```

> If `cargo add` fails with `error: no such subcommand`, then please edit the
> `Cargo.toml` file by hand. Add the dependencies listed below.

You will also need a way to find links. We can use [`scraper`][2] for that:

```shell
cargo add scraper
```

Finally, we'll need some way of handling errors. We use [`thiserror`][3] for
that:

```shell
cargo add thiserror
```

The `cargo add` calls will update the `Cargo.toml` file to look like this:

<!-- File Cargo.toml -->

```toml
[package]
name = "link-checker"
version = "0.1.0"
edition = "2021"
publish = false

[dependencies]
reqwest = { version = "0.11.12", features = ["blocking", "rustls-tls"] }
scraper = "0.13.0"
thiserror = "1.0.37"
```

You can now download the start page. Try with a small site such as
`https://www.google.org/`.

Your `src/main.rs` file should look something like this:

<!-- File src/main.rs -->

```rust
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

fn main() {
    let client = Client::new();
    let start_url = Url::parse("https://www.google.org").unwrap();
    let crawl_command = CrawlCommand{ url: start_url, extract_links: true };
    match visit_page(&client, &crawl_command) {
        Ok(links) => println!("Links: {links:#?}"),
        Err(err) => println!("Could not extract links: {err:#}"),
    }
}
```

Run the code in `src/main.rs` with

```shell
cargo run
```

## Tasks

- Use threads to check the links in parallel: send the URLs to be checked to a
  channel and let a few threads check the URLs in parallel.
- Extend this to recursively extract links from all pages on the
  `www.google.org` domain. Put an upper limit of 100 pages or so so that you
  don't end up being blocked by the site.

[1]: https://docs.rs/reqwest/
[2]: https://docs.rs/scraper/
[3]: https://docs.rs/thiserror/
