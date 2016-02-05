//! Authentication.

use std::error::Error;
use std::io;
use std::marker::PhantomData;

use hyper;
use inth_oauth2::{Client, ClientError, Token};
use inth_oauth2::provider::Google;
use inth_oauth2::token::{Bearer, Expiring};
use yup_oauth2;

/// Google OAuth2 authenticators.
pub trait Authenticator {
    /// Perform initial authentication.
    fn authenticate(
        client: &Client<Google>,
        http: &hyper::Client,
        scope: &str
    ) -> Result<Bearer<Expiring>, ClientError>;
}

/// Simple authenticator via stdout and stdin.
#[derive(Debug)]
pub struct SimpleAuthenticator;
impl Authenticator for SimpleAuthenticator {
    fn authenticate(
        client: &Client<Google>,
        http: &hyper::Client,
        scope: &str
    ) -> Result<Bearer<Expiring>, ClientError> {
        let auth_uri = try!(client.auth_uri(Some(scope), None));
        println!("{}", auth_uri);

        let mut code = String::new();
        try!(io::stdin().read_line(&mut code));

        client.request_token(http, code.trim())
    }
}

/// Token cache for use with Google APIs.
#[allow(missing_debug_implementations)]
pub struct TokenCache<A: Authenticator> {
    client: Client<Google>,
    http: hyper::Client,
    scope: String,
    token: Option<Bearer<Expiring>>,
    authenticator: PhantomData<A>,
}

impl<A: Authenticator> TokenCache<A> {
    /// Creates a token cache.
    pub fn new<S>(
        client: Client<Google>,
        http: hyper::Client,
        scope: S,
        token: Option<Bearer<Expiring>>
    ) -> Self where S: AsRef<str> {
        TokenCache {
            client: client,
            http: http,
            scope: scope.as_ref().to_owned(),
            token: token,
            authenticator: PhantomData,
        }
    }

    /// Returns a valid token from cache, by refreshing, or through the `Authenticator`.
    pub fn token(&mut self) -> Result<&Bearer<Expiring>, ClientError> {
        let token = match self.token.take() {
            Some(token) => try!(self.client.ensure_token(&self.http, token)),
            None => try!(A::authenticate(&self.client, &self.http, &self.scope)),
        };
        self.token = Some(token);
        Ok(self.token.as_ref().unwrap())
    }
}

impl<A: Authenticator> yup_oauth2::GetToken for TokenCache<A> {
    fn token<'b, I, T>(&mut self, _scopes: I) -> Result<yup_oauth2::Token, Box<Error>>
        where T: AsRef<str> + Ord + 'b, I: IntoIterator<Item=&'b T>
    {
        let token = try!(self.token());
        Ok(yup_oauth2::Token {
            access_token: token.access_token().to_owned(),
            refresh_token: token.lifetime().refresh_token().to_owned(),
            token_type: String::from("Bearer"),
            expires_in: Some(3600), // This is probably okay, right?
            expires_in_timestamp: Some(token.lifetime().expires().timestamp()),
        })
    }

    fn api_key(&mut self) -> Option<String> {
        None
    }
}
