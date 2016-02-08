#[macro_use(crate_version)]
extern crate clap;
extern crate fresh;
extern crate hyper;
extern crate inth_oauth2;
extern crate xdg;

use std::io;
use std::path::PathBuf;

use clap::{App, AppSettings, Arg, SubCommand};
use fresh::authenticator::Authenticator;
use hyper::Client as HttpClient;
use inth_oauth2::{Client as OAuth2Client, ClientError as OAuth2ClientError};
use inth_oauth2::provider::Provider;
use xdg::BaseDirectories;

struct Prompt;
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

fn main() {
    let xdg = BaseDirectories::with_prefix("fresh").unwrap();

    let matches = App::new("Fresh CLI")
        .about("Random password reset automation")
        .version(crate_version!())
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .setting(AppSettings::GlobalVersion)
        .args(&[
            Arg::with_name("token_path")
                .long("token").short("t").value_name("PATH")
                .help("Path to OAuth 2.0 token JSON"),

            Arg::with_name("generator")
                .long("generator").short("g").value_name("GEN")
                .possible_values(&["char", "hex", "base64"])
                .help("Password generator"),

            Arg::with_name("length")
                .long("length").short("l").value_name("N")
                .help("Password length"),

            Arg::with_name("tries")
                .long("tries").value_name("N")
                .help("Number of inbox query retries"),

            Arg::with_name("delay")
                .long("delay").value_name("SECS")
                .help("Delay between inbox query retries"),

            Arg::with_name("no_archive")
                .long("no-archive")
                .help("Do not archive password reset message"),

            Arg::with_name("open")
                .long("open").short("o")
                .help("Open login page after reset")
        ])
        .subcommand(
            SubCommand::with_name("hackernews")
                .about("Reset Hacker News password")
                .arg(Arg::with_name("username").required(true).help("Username"))
        )
        .get_matches();

    let (subcommand, sub_matches) = matches.subcommand();
    let sub_matches = sub_matches.unwrap();

    let token_path = matches.value_of("token_path")
        .map(PathBuf::from)
        .unwrap_or(xdg.place_config_file("token.json").unwrap());
}
