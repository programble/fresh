//! OAuth 2.0 token cache.

use std::error::Error;

use chrono::UTC;
use hyper::Client as Http;
use inth_oauth2::{Client, ClientError, Token};
use inth_oauth2::provider::google::Installed;
use inth_oauth2::token::{Bearer, Refresh};
use yup_oauth2::{GetToken, Token as YupToken};

/// OAuth 2.0 token cache.
///
/// Implements `GetToken` from `yup_oauth2` for use with `google_gmail1`.
#[derive(Debug)]
pub struct TokenCache {
    client: Client<Installed>,
    http: Http,
    token: Option<Bearer<Refresh>>,
}

impl TokenCache {
    /// Creates a token cache.
    pub fn new(client: Client<Installed>, http: Http, token: Bearer<Refresh>) -> Self {
        TokenCache {
            client: client,
            http: http,
            token: Some(token),
        }
    }

    /// Returns a valid token from cache, refreshing as necessary.
    pub fn token(&mut self) -> Result<&Bearer<Refresh>, ClientError> {
        let current = self.token.take().unwrap();
        let ensured = try!(self.client.ensure_token(&self.http, current));
        self.token = Some(ensured);
        Ok(self.token.as_ref().unwrap())
    }

    /// Returns a `yup_oauth2::Token` for use with `google_gmail1`.
    pub fn yup_token(&mut self) -> Result<YupToken, ClientError> {
        let token = try!(self.token());
        let expires = token.lifetime().expires();
        let expires_in = *expires - UTC::now();

        Ok(YupToken {
            access_token: token.access_token().to_owned(),
            refresh_token: token.lifetime().refresh_token().to_owned(),
            token_type: String::from("Bearer"),
            expires_in: Some(expires_in.num_seconds()),
            expires_in_timestamp: Some(expires.timestamp()),
        })
    }
}

/// Ignores passed in scopes.
impl GetToken for TokenCache {
    fn token<'a, I, T>(&mut self, _scopes: I) -> Result<YupToken, Box<Error>>
        where T: AsRef<str> + Ord + 'a, I: IntoIterator<Item = &'a T>
    {
        Ok(try!(self.yup_token()))
    }

    fn api_key(&mut self) -> Option<String> { None }
}
