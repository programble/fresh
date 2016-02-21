#![warn(
    missing_debug_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    variant_size_differences
)]

#[macro_use(crate_version)]
extern crate clap;
extern crate fresh;
extern crate hyper;
extern crate inth_oauth2;
extern crate rustc_serialize;
extern crate xdg;

use std::path::PathBuf;

use clap::{App, AppSettings, Arg, SubCommand};

mod authenticator;
mod generate;
mod token_cache;

fn main() {
    let matches = App::new("Fresh CLI")
        .about("Random password reset automation")
        .version(crate_version!())
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .setting(AppSettings::GlobalVersion)
        .args(&[
              Arg::with_name("token_path")
                .long("token").short("t").value_name("PATH")
                .help("Path to OAuth 2.0 token JSON"),
        ])
        .subcommand(
            SubCommand::with_name("hackernews")
                .about("Reset Hacker News password")
                .arg(Arg::with_name("username").required(true).help("Username"))
        )
        .get_matches();

    let token_path = matches.value_of("token_path")
        .map(PathBuf::from)
        .unwrap_or(token_cache::default_path());

    let mut cache = token_cache::load(&token_path);

    cache.authenticate().unwrap();

    token_cache::save(&mut cache, &token_path);
}
