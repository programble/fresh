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
use std::time::Duration;

use clap::{App, AppSettings, Arg, SubCommand};
use fresh::gmail::InboxBuilder;

mod account;
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

            Arg::with_name("generator")
                .long("generator").short("g").value_name("GEN")
                .possible_values(&["base64", "char", "hex"])
                .help("Password generator"),

            Arg::with_name("length")
                .long("length").short("l").value_name("N")
                .help("Password length [32]"),

            Arg::with_name("tries")
                .long("tries").value_name("N")
                .help("Number of inbox query retries [30]"),

            Arg::with_name("delay")
                .long("delay").value_name("SECS")
                .help("Delay between inbox query retries [1]"),

            Arg::with_name("no_archive")
                .long("no-archive")
                .help("Do not archive password reset message"),

            Arg::with_name("verbose")
                .long("verbose").short("V")
                .help("Verbose output"),
        ])
        .subcommand(
            SubCommand::with_name("hackernews")
                .about("Reset Hacker News password")
                .arg(Arg::with_name("username").required(true).help("Username"))
        )
        .subcommand(
            SubCommand::with_name("lobsters")
                .about("Reset Lobsters password")
                .arg(Arg::with_name("email").required(true).help("Email"))
        )
        .get_matches();

    let verbose = matches.is_present("verbose");

    let token_path = matches.value_of("token_path")
        .map(PathBuf::from)
        .unwrap_or(token_cache::default_path());

    let gen_type = matches.value_of("generator").unwrap_or("base64");
    let length = matches.value_of("length")
        .map(|n| n.parse().unwrap())
        .unwrap_or(32);

    let tries = matches.value_of("tries")
        .map(|n| n.parse().unwrap())
        .unwrap_or(30);
    let delay = Duration::from_secs(
        matches.value_of("delay")
            .map(|n| n.parse().unwrap())
            .unwrap_or(1)
    );

    let archive = !matches.is_present("no_archive");

    let (account_type, account_matches) = matches.subcommand();
    let account_user = match account_type {
        "hackernews" => account_matches.unwrap().value_of("username").unwrap(),
        "lobsters" => account_matches.unwrap().value_of("email").unwrap(),
        _ => unreachable!(),
    };

    let mut token_cache = token_cache::load(&token_path);
    token_cache.authenticate().unwrap();
    token_cache::save(&mut token_cache, &token_path);

    let inbox = InboxBuilder::new(token_cache)
        .find_tries(tries)
        .find_delay(delay)
        .finalize();

    let password = generate::password(gen_type, length);

    account::reset_password(account_type, account_user, &inbox, &password, archive, verbose);
}
