use google_gmail1::Message;
use hyper::Client as HttpClient;
use inth_oauth2::provider::Google;
use regex::Regex;

use authenticator::Authenticator;
use gmail::Inbox;
use super::{Account, AccountError};
use super::error::MessageError;
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

const GMAIL_QUERY: &'static str =
    "from:(hn@ycombinator.com) subject:(Hacker News Password Recovery)";

const FNID_REGEX: &'static str = r"fnid=([A-Za-z1-9]+)";

impl Account for HackerNews {
    type ResetKey = String;

    fn initiate_reset(&self, http: &HttpClient) -> Result<(), AccountError> {
        let mut response = try!(helpers::get_ok(http, FORGOT_URL));
        let html = try!(helpers::read_to_html(&mut response));
        let fnid = try!(helpers::select_attr(FORGOT_URL, &html, INPUT_FNID, "value"));

        let body_pairs = [
            ("fnop", "forgot-password"),
            ("fnid", fnid),
            ("s", &self.username),
        ];
        try!(helpers::post_ok(http, X_URL, &body_pairs));
        Ok(())
    }

    fn find_message<A: Authenticator<Google>>(
        &self,
        inbox: &Inbox<A>
    ) -> Result<Message, AccountError> {
        helpers::inbox_find(inbox, GMAIL_QUERY)
    }

    fn parse_message(&self, message: &Message) -> Result<String, AccountError> {
        let re = Regex::new(FNID_REGEX).unwrap();
        let body = try!(helpers::decode_part(message, "text/plain"));

        let captures = match re.captures(&body) {
            Some(c) => c,
            None => return Err(MessageError::Regex(String::from(FNID_REGEX)).into()),
        };
        let fnid = captures.at(1).unwrap();

        Ok(String::from(fnid))
    }

    fn set_password(
        &self,
        _http: &HttpClient,
        _key: &String,
        _password: &str
    ) -> Result<(), AccountError> {
        unimplemented!()
    }

    fn login_url(&self) -> String {
        String::from(LOGIN_URL)
    }
}
