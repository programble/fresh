#[macro_use] extern crate failure;
#[macro_use] extern crate serde_derive;
extern crate imap;
extern crate imap_proto;
extern crate mailparse;
extern crate native_tls;
extern crate rand;
extern crate reqwest;
extern crate scraper;
extern crate serde;

pub mod reset;
pub mod mail;

pub fn generate_password(len: usize) -> String {
    use rand::distributions::{IndependentSample, Range};
    let range = Range::new(32u8, 127);
    let mut rng = rand::thread_rng();
    std::iter::repeat(&range)
        .map(|r| r.ind_sample(&mut rng) as char)
        .take(len)
        .collect()
}
