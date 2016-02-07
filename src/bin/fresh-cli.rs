#[macro_use(crate_version)]
extern crate clap;

use clap::{App, AppSettings, Arg, SubCommand};

fn main() {
    let m = App::new("Fresh CLI")
        .about("Random password reset automation")
        .version(crate_version!())
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .setting(AppSettings::GlobalVersion)

        .arg(
            Arg::with_name("token")
                .long("token")
                .short("t")
                .help("Path to OAuth 2.0 token JSON")
                .global(true)
                .value_name("PATH")
        )
        .arg(
            Arg::with_name("open")
                .long("open")
                .short("o")
                .help("Open login page after reset")
                .global(true)
        )

        .subcommand(
            SubCommand::with_name("hackernews")
                .about("Reset Hacker News password")
                .arg(
                    Arg::with_name("username")
                        .help("Hacker News username")
                        .required(true)
                )
        )

        .get_matches();

    println!("{:#?}", m);
}
