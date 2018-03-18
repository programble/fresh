#[macro_use] extern crate failure;
extern crate mailparse;
extern crate reqwest;
extern crate scraper;
extern crate rand;

pub mod reset;

pub fn generate_password(len: usize) -> String {
    use rand::distributions::{IndependentSample, Range};
    let range = Range::new(32u8, 127);
    let mut rng = rand::thread_rng();
    std::iter::repeat(&range)
        .map(|r| r.ind_sample(&mut rng) as char)
        .take(len)
        .collect()
}
