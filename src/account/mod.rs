//! Accounts for which passwords can be reset.

use std::fmt::Debug;

use google_gmail1::Message;
use hyper::Client as HttpClient;
use hyper::client::RedirectPolicy;
use inth_oauth2::provider::google::Installed;

use authenticator::Authenticator;
use gmail::Inbox;

/// An account whose password can be reset.
pub trait Account {
    /// Information required to set the account password.
    type ResetKey: Debug;

    /// Initiates the password reset flow, usually through a "forgot password" form.
    fn initiate_reset(&self, http: &HttpClient) -> Result<(), AccountError>;

    /// Finds a Gmail message that can be parsed into a `ResetKey`.
    fn find_message<A: Authenticator<Installed>>(
        &self,
        inbox: &Inbox<A>
    ) -> Result<Message, AccountError>;

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

    /// Returns the desired Hyper redirect policy.
    ///
    /// Will be set for HTTP clients passed to `initiate_reset` and `set_password`.
    fn redirect_policy(&self) -> RedirectPolicy { Default::default() }
}

pub use self::error::AccountError;
pub mod error;

pub mod helpers;

pub use self::hacker_news::HackerNews;
mod hacker_news;

pub use self::lobsters::Lobsters;
mod lobsters;
