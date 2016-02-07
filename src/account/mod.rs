//! Accounts for which passwords can be reset.

use google_gmail1::Message;
use hyper::Client;

/// An account whose password can be reset.
pub trait Account {
    /// Information required to set the account password.
    type ResetKey;

    /// Initiates the password reset flow, usually through a "forgot password" form.
    fn initiate_reset(&self, client: &Client) -> Result<(), AccountError>;

    /// Returns a Gmail search query for password reset emails.
    fn gmail_query(&self) -> String;

    /// Parses a Gmail message into a `ResetKey` which can be used to set the password.
    fn parse_message(&self, message: &Message) -> Result<Self::ResetKey, AccountError>;

    /// Sets the account password.
    fn set_password(&self, key: &Self::ResetKey, password: String) -> Result<(), AccountError>;

    /// Returns a URL at which the user can log in to the account.
    fn login_url(&self) -> String;
}

pub use self::error::AccountError;
pub mod error;

pub mod helpers;

pub use self::hacker_news::HackerNews;
mod hacker_news;
