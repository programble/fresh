use std::io::Read;

use hyper::{self, Client};
use hyper::header::ContentType;
use scraper::{Html, Selector};
use url::form_urlencoded;

use super::{Account, StatusError, MarkupError, AccountError};

/// A Hacker News account.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HackerNews {
    /// Username.
    pub username: String,
}

const INPUT_FNID_STR: &'static str = r#"input[name="fnid"]"#;

lazy_static! {
    static ref INPUT_FNID: Selector = Selector::parse(INPUT_FNID_STR).unwrap();
}

impl Account for HackerNews {
    fn login_url(&self) -> String {
        String::from("https://news.ycombinator.com/login")
    }

    fn initiate_reset(&self, client: &Client) -> Result<(), AccountError> {
        let request = client.get("https://news.ycombinator.com/forgot");
        let mut response = try!(request.send());
        if response.status != hyper::Ok {
            return Err(StatusError(response.status).into());
        }

        let mut body = String::new();
        try!(response.read_to_string(&mut body));
        let html = Html::parse_document(&body);

        let input_fnid = try! {
            html.select(&INPUT_FNID)
                .next()
                .ok_or_else(|| MarkupError::MissingElement(String::from(INPUT_FNID_STR)))
        };
        let fnid = try! {
            input_fnid
                .value()
                .as_element()
                .and_then(|e| e.attr("value"))
                .ok_or_else(|| MarkupError::MissingAttr(String::from("value")))
        };

        let body_pairs = vec![
            ("fnop", "forgot-password"),
            ("fnid", fnid),
            ("s", &self.username),
        ];
        let body = form_urlencoded::serialize(body_pairs);

        let request = client.post("https://news.ycombinator.com/x")
            .header(ContentType::form_url_encoded())
            .body(&body);
        let response = try!(request.send());
        if response.status != hyper::Ok {
            return Err(StatusError(response.status).into());
        }
        Ok(())
    }
}
