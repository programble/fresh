use google_gmail1::Message;
use hyper::Client as HttpClient;
use hyper::client::RedirectPolicy;
use hyper::status::StatusCode;
use inth_oauth2::provider::Google;
use url::Url;

use authenticator::Authenticator;
use gmail::Inbox;
use super::{Account, AccountError};
use super::helpers;

/// A Lobsters account.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Lobsters {
    /// Email.
    pub email: String,
}

const LOGIN_URL: &'static str = "https://lobste.rs/login";
const FORGOT_URL: &'static str = "https://lobste.rs/login/forgot_password";
const RESET_URL: &'static str = "https://lobste.rs/login/reset_password";
const SET_NEW_URL: &'static str = "https://lobste.rs/login/set_new_password";

const INPUT_AUTHENTICITY: &'static str = r#"input[name="authenticity_token"]"#;

const GMAIL_QUERY: &'static str =
    "from:(nobody@lobste.rs) subject:([Lobsters] Reset your password)";

const TOKEN_REGEX: &'static str = "token=([A-Za-z0-9-]+)";

impl Account for Lobsters {
    type ResetKey = String;

    fn initiate_reset(&self, http: &HttpClient) -> Result<(), AccountError> {
        let mut response = try!(helpers::get_ok(http, FORGOT_URL));
        let html = try!(helpers::read_to_html(&mut response));
        let authenticity = try!(
            helpers::select_attr(FORGOT_URL, &html, INPUT_AUTHENTICITY, "value")
        );

        let body_pairs = [
            ("utf8", "✓"),
            ("authenticity_token", authenticity),
            ("email", &self.email),
        ];
        try!(helpers::post_ok(http, RESET_URL, &body_pairs));
        Ok(())
    }

    fn find_message<A: Authenticator<Google>>(
        &self,
        inbox: &Inbox<A>,
    ) -> Result<Message, AccountError> {
        helpers::inbox_find(inbox, GMAIL_QUERY)
    }

    fn parse_message(&self, message: &Message) -> Result<String, AccountError> {
        let body = try!(helpers::decode_part(message, "text/plain"));
        let captures = try!(helpers::regex_captures(TOKEN_REGEX, &body));
        let token = captures.at(1).unwrap();
        Ok(String::from(token))
    }

    fn set_password(
        &self,
        http: &HttpClient,
        key: &String,
        password: &str,
    ) -> Result<(), AccountError> {
        let mut url = Url::parse(SET_NEW_URL).unwrap();
        url.set_query_from_pairs(&[("token", key)]);
        let url = url.serialize();

        let mut response = try!(helpers::get_ok(http, &url));
        let html = try!(helpers::read_to_html(&mut response));
        let authenticity = try!(
            helpers::select_attr(&url, &html, INPUT_AUTHENTICITY, "value")
        );

        let body_pairs = [
            ("utf8", "✓"),
            ("authenticity_token", authenticity),
            ("token", key),
            ("password", password),
            ("password_confirmation", password),
        ];
        try!(helpers::post_expect(http, SET_NEW_URL, &body_pairs, StatusCode::Found));
        Ok(())
    }

    fn login_url(&self) -> String {
        String::from(LOGIN_URL)
    }

    fn redirect_policy(&self) -> RedirectPolicy {
        RedirectPolicy::FollowNone
    }
}
