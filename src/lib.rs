#[macro_use] extern crate failure;
extern crate reqwest;
extern crate scraper;

use failure::Error;
use reqwest::Client;
use scraper::{Html, Selector};

pub trait Reset {
    fn send_email(&self, client: &Client) -> Result<(), Error>;
}

pub struct HackerNews<'a>(pub &'a str);

impl<'a> Reset for HackerNews<'a> {
    fn send_email(&self, client: &Client) -> Result<(), Error> {
        let html = client.get("https://news.ycombinator.com/forgot")
            .send()?
            .error_for_status()?
            .text()?;
        let doc = Html::parse_document(&html);
        let sel = Selector::parse(r#"input[name="fnid"]"#).unwrap();
        let fnid = doc.select(&sel)
            .next()
            .ok_or(format_err!("no fnid input"))?
            .value()
            .attr("value")
            .ok_or(format_err!("no fnid value"))?;

        let form = [
            ("fnop", "forgot-password"),
            ("fnid", fnid),
            ("s", &self.0),
        ];
        client.post("https://news.ycombinator.com/x")
            .form(&form)
            .send()?
            .error_for_status()?;
        Ok(())
    }
}

struct A {
    reset: Box<Reset>,
}
