use hyper::Client;

use super::{Account, AccountError};
use super::helpers;

/// A Hacker News account.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HackerNews {
    /// Username.
    pub username: String,
}

const LOGIN_URL: &'static str = "https://news.ycombinator.com/login";
const FORGOT_URL: &'static str = "https://news.ycombinator.com/forgot";
const X_URL: &'static str = "https://news.ycombinator.com/x";

const INPUT_FNID: &'static str = r#"input[name="fnid"]"#;

impl Account for HackerNews {
    fn login_url(&self) -> String {
        String::from(LOGIN_URL)
    }

    fn initiate_reset(&self, client: &Client) -> Result<(), AccountError> {
        let mut response = try!(helpers::get_ok(client, FORGOT_URL));
        let html = try!(helpers::read_to_html(&mut response));
        let fnid = try!(helpers::select_attr(FORGOT_URL, &html, INPUT_FNID, "value"));

        let body_pairs = [
            ("fnop", "forgot-password"),
            ("fnid", fnid),
            ("s", &self.username),
        ];
        try!(helpers::post_ok(client, X_URL, &body_pairs));
        Ok(())
    }
}
