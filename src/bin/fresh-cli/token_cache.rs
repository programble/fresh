use std::fs::File;
use std::io::{Read, Write, ErrorKind};
use std::path::{PathBuf, Path};

use fresh::credentials::{CLIENT_ID, CLIENT_SECRET};
use fresh::gmail::Scope;
use fresh::token_cache::TokenCache;
use inth_oauth2::Client as OAuth2Client;
use rustc_serialize::json;
use xdg::BaseDirectories;

use authenticator::Prompt;

pub fn default_path() -> PathBuf {
    let xdg = BaseDirectories::with_prefix("fresh").unwrap();
    xdg.place_config_file("token.json").unwrap()
}

pub fn load(path: &Path) -> TokenCache<Prompt> {
    let file = match File::open(path) {
        Ok(f) => Some(f),
        Err(ref err) if err.kind() == ErrorKind::NotFound => None,
        Err(err) => panic!(err),
    };

    let token = file.map(|mut f| {
        let mut json = String::new();
        f.read_to_string(&mut json).unwrap();
        json::decode(&json).unwrap()
    });

    TokenCache::new(
        OAuth2Client::new(
            String::from(CLIENT_ID),
            String::from(CLIENT_SECRET),
            Some(String::from("urn:ietf:wg:oauth:2.0:oob")),
        ),
        Default::default(),
        String::from(Scope::Modify.as_ref()),
        token,
    )
}

pub fn save(token_cache: &mut TokenCache<Prompt>, path: &Path) {
    let token = token_cache.token().unwrap();
    let json = json::encode(token).unwrap();
    let mut file = File::create(path).unwrap();
    file.write_all(json.as_bytes()).unwrap();
}
