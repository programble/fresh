use google_gmail1::Message;
use hyper::Client as HttpClient;
use inth_oauth2::provider::google::Installed;
use url::Url;

use authenticator::Authenticator;
use gmail::Inbox;
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
const R_URL: &'static str = "https://news.ycombinator.com/r";

const INPUT_FNID: &'static str = r#"input[name="fnid"]"#;

const GMAIL_QUERY: &'static str =
    "from:(hn@ycombinator.com) subject:(Hacker News Password Recovery)";

const FNID_REGEX: &'static str = "fnid=([A-Za-z0-9]+)";

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

    fn find_message<A: Authenticator<Installed>>(
        &self,
        inbox: &Inbox<A>
    ) -> Result<Message, AccountError> {
        helpers::inbox_find(inbox, GMAIL_QUERY)
    }

    fn parse_message(&self, message: &Message) -> Result<String, AccountError> {
        let body = try!(helpers::decode_part(message, "text/plain"));
        let captures = try!(helpers::regex_captures(FNID_REGEX, &body));
        let fnid = captures.at(1).unwrap();
        Ok(String::from(fnid))
    }

    fn set_password(
        &self,
        http: &HttpClient,
        key: &String,
        password: &str
    ) -> Result<(), AccountError> {
        let mut url = Url::parse(X_URL).unwrap();
        url.set_query_from_pairs(&[
            ("fnop", "passwd-reset"),
            ("fnid", key),
        ]);
        let url = url.serialize();

        let mut response = try!(helpers::get_ok(http, &url));
        let html = try!(helpers::read_to_html(&mut response));
        let fnid = try!(helpers::select_attr(&url, &html, INPUT_FNID, "value"));

        let body_pairs = [
            ("fnop", "changepw-page"),
            ("fnid", fnid),
            ("pw", password),
        ];
        try!(helpers::post_ok(http, R_URL, &body_pairs));
        Ok(())
    }

    fn login_url(&self) -> String {
        String::from(LOGIN_URL)
    }
}
