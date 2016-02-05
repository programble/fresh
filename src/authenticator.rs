//! Authentication.

use std::io;

use hyper::Client as HttpClient;
use inth_oauth2::{Client as OAuth2Client, ClientError};
use inth_oauth2::provider::Provider;

/// OAuth2 authenticators.
pub trait Authenticator<P: Provider> {
    /// Perform initial authentication.
    fn authenticate(
        oauth2: &OAuth2Client<P>,
        http: &HttpClient,
        scope: &str
    ) -> Result<P::Token, ClientError>;
}

/// Simple authenticator via stdout and stdin.
#[derive(Debug)]
pub struct Simple;
impl<P: Provider> Authenticator<P> for Simple {
    fn authenticate(
        oauth2: &OAuth2Client<P>,
        http: &HttpClient,
        scope: &str
    ) -> Result<P::Token, ClientError> {
        let auth_uri = try!(oauth2.auth_uri(Some(scope), None));
        println!("{}", auth_uri);

        let mut code = String::new();
        try!(io::stdin().read_line(&mut code));

        oauth2.request_token(http, code.trim())
    }
}
