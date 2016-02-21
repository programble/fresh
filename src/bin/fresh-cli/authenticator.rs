use fresh::authenticator::Authenticator;
use hyper::Client as HttpClient;
use inth_oauth2::{Client as OAuth2Client, ClientError as OAuth2ClientError};
use inth_oauth2::provider::Provider;

use std::io;

pub struct Prompt;
impl<P: Provider> Authenticator<P> for Prompt {
    fn authenticate(
        oauth2: &OAuth2Client<P>,
        http: &HttpClient,
        scope: &str
    ) -> Result<P::Token, OAuth2ClientError> {
        let auth_uri = try!(oauth2.auth_uri(Some(scope), None));
        println!("To authorize, open the following URL and paste the code below:\n{}", auth_uri);

        let mut code = String::new();
        try!(io::stdin().read_line(&mut code));

        oauth2.request_token(http, code.trim())
    }
}
