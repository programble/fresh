//! Accounts for which passwords can be reset.

use hyper::Client;

/// An account whose password can be reset.
pub trait Account {
    /// Returns a URL at which the user can log in to the account.
    fn login_url(&self) -> String;

    /// Initiates the password reset flow, usually through a "forgot password" form.
    fn initiate_reset(&self, client: &Client) -> Result<(), AccountError>;

    /// Returns a Gmail search query for password reset emails.
    fn gmail_query(&self) -> String;
}

pub use self::error::AccountError;
pub mod error;

pub mod helpers;

pub use self::hacker_news::HackerNews;
mod hacker_news;
