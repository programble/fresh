use failure::Error;
use reqwest::Client;
use scraper::{Html, Selector};
use mailparse::ParsedMail;

fn _object_safe(_: &Reset) { }
pub trait Reset {
    fn send_mail(&self, client: &Client) -> Result<(), Error>;
    fn match_mail(&self, mail: &ParsedMail) -> bool;
    fn set_password(
        &self, client: &Client, mail: &ParsedMail, password: &str
    ) -> Result<(), Error>;
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct HackerNews<'a> {
    pub username: &'a str,
}

impl<'a> Reset for HackerNews<'a> {
    fn send_mail(&self, client: &Client) -> Result<(), Error> {
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
            ("s", self.username),
        ];
        client.post("https://news.ycombinator.com/x")
            .form(&form)
            .send()?
            .error_for_status()?;
        Ok(())
    }

    fn match_mail(&self, _mail: &ParsedMail) -> bool {
        false
    }

    fn set_password(
        &self, _client: &Client, _mail: &ParsedMail, _password: &str
    ) -> Result<(), Error> {
        unimplemented!()
    }
}
