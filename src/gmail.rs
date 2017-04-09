//! Gmail inbox client.

use std::fmt;
use std::time::Duration;

use google_gmail1::Gmail;
use hyper::Client as Http;

use token_cache::TokenCache;

/// Gmail inbox client.
pub struct Inbox {
    gmail: Gmail<Http, TokenCache>,
    retry_tries: u32,
    retry_interval: Duration,
}
