//! Accounts for which passwords can be reset.

use std::time::Duration;

use google_gmail1::Message;
use hyper::Client as HttpClient;
use inth_oauth2::provider::Google;

use authenticator::Authenticator;
use generator::Generator;
use gmail::Inbox;

/// An account whose password can be reset.
pub trait Account {
    /// Information required to set the account password.
    type ResetKey;

    /// Initiates the password reset flow, usually through a "forgot password" form.
    fn initiate_reset(&self, http: &HttpClient) -> Result<(), AccountError>;

    /// Returns a Gmail search query for password reset emails.
    fn gmail_query(&self) -> String;

    /// Parses a Gmail message into a `ResetKey` which can be used to set the password.
    fn parse_message(&self, message: &Message) -> Result<Self::ResetKey, AccountError>;

    /// Sets the account password.
    fn set_password(
        &self,
        http: &HttpClient,
        key: &Self::ResetKey,
        password: &str
    ) -> Result<(), AccountError>;

    /// Returns a URL at which the user can log in to the account.
    fn login_url(&self) -> String;
}

pub use self::error::AccountError;
pub mod error;

pub mod helpers;

pub use self::hacker_news::HackerNews;
mod hacker_news;

/// Reset configuration.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ResetConfig {
    /// Number of inbox query retries.
    pub inbox_tries: u32,

    /// Delay between inbox query retries.
    pub inbox_delay: Duration,

    /// Archive inbox message after successful reset.
    pub inbox_archive: bool,

    /// Length of password to generate.
    pub password_length: usize,
}

/// Resets an account password.
pub fn reset_password<A: Account, G: Generator, U: Authenticator<Google>>(
    account: &A,
    generator: &G,
    inbox: &Inbox<U>,
    config: &ResetConfig,
    http: &HttpClient
) -> Result<(), AccountError> {
    try!(account.initiate_reset(http));

    let query = account.gmail_query();
    let message = match try!(inbox.find_retry(&query, config.inbox_tries, config.inbox_delay)) {
        Some(m) => m,
        None => return Err(error::MessageError::Missing(query).into()),
    };

    let key = try!(account.parse_message(&message));
    let password = generator.generate(config.password_length);

    account.set_password(&http, &key, &password)
}
