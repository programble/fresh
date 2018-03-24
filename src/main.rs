#[macro_use] extern crate serde_derive;
extern crate failure;
extern crate fresh;
extern crate native_tls;
extern crate serde;
extern crate toml;
extern crate xdg;

use failure::Error;
use fresh::mail::Imap;
use fresh::reset::HackerNews;
use std::fs::File;
use std::io::Read;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
struct Password {
    length: usize,
}

impl Default for Password {
    fn default() -> Self {
        Password { length: 50 }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Config<'a> {
    #[serde(default)]
    password: Password,

    #[serde(borrow)]
    imap: Vec<Imap<'a>>,

    #[serde(borrow)]
    hacker_news: Option<HackerNews<'a>>,
}

fn result_main() -> Result<(), Error> {
    let xdg = xdg::BaseDirectories::with_prefix("fresh")?;
    let mut toml = String::new();
    if let Some(path) = xdg.find_config_file("fresh.toml") {
        let mut file = File::open(path)?;
        file.read_to_string(&mut toml)?;
    }
    let config: Config = toml::from_str(&toml)?;

    let tls = native_tls::TlsConnector::builder()?.build()?;
    let mut client = fresh::mail::connect(&tls, &config.imap[0])?;

    Ok(())
}

fn main() {
    result_main().unwrap()
}
