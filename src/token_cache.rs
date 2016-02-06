//! Token caching.

use std::error::Error;
use std::marker::PhantomData;

use hyper::Client as HttpClient;
use inth_oauth2::{Client as OAuth2Client, ClientError, Token};
use inth_oauth2::provider::{Provider, Google};
use yup_oauth2::{GetToken, Token as YupToken};

use authenticator::Authenticator;

/// Token cache for use with Google APIs.
#[allow(missing_debug_implementations)]
pub struct TokenCache<A: Authenticator<Google>> {
    oauth2: OAuth2Client<Google>,
    http: HttpClient,
    scope: String,
    token: Option<<Google as Provider>::Token>,
    authenticator: PhantomData<A>,
}

impl<A: Authenticator<Google>> TokenCache<A> {
    /// Creates a token cache.
    pub fn new(
        oauth2: OAuth2Client<Google>,
        http: HttpClient,
        scope: String,
        token: Option<<Google as Provider>::Token>
    ) -> Self {
        TokenCache {
            oauth2: oauth2,
            http: http,
            scope: scope,
            token: token,
            authenticator: PhantomData,
        }
    }

    /// Performs initial authentication through the `Authenticator`.
    ///
    /// Does nothing if already authenticated.
    pub fn authenticate(&mut self) -> Result<(), ClientError> {
        if self.token.is_some() { return Ok(()); }
        let token = try!(A::authenticate(&self.oauth2, &self.http, &self.scope));
        self.token = Some(token);
        Ok(())
    }

    /// Returns a valid token either from cache, by refreshing, or through the `Authenticator`.
    pub fn token(&mut self) -> Result<&<Google as Provider>::Token, ClientError> {
        if self.token.is_none() {
            try!(self.authenticate());
        }
        let token = try!(self.oauth2.ensure_token(&self.http, self.token.take().unwrap()));
        self.token = Some(token);
        Ok(self.token.as_ref().unwrap())
    }
}

impl<A: Authenticator<Google>> GetToken for TokenCache<A> {
    fn token<'b, I, T>(&mut self, _scopes: I) -> Result<YupToken, Box<Error>>
        where T: AsRef<str> + Ord + 'b, I: IntoIterator<Item=&'b T>
    {
        // TODO: Handle scopes in a reasonable way.
        let token = try!(self.token());
        Ok(YupToken {
            access_token: token.access_token().to_owned(),
            refresh_token: token.lifetime().refresh_token().to_owned(),
            token_type: String::from("Bearer"),
            expires_in: Some(3600), // This is probably okay, right?
            expires_in_timestamp: Some(token.lifetime().expires().timestamp()),
        })
    }

    fn api_key(&mut self) -> Option<String> { None }
}
