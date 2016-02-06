//! fresh.

#![warn(
    missing_docs,
    missing_debug_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    variant_size_differences
)]

extern crate google_gmail1;
extern crate hyper;
extern crate inth_oauth2;
extern crate rand;
extern crate rustc_serialize;
extern crate scraper;
extern crate url;
extern crate yup_oauth2;

pub mod account;
pub mod authenticator;
pub mod generator;
pub mod gmail;
pub mod token_cache;
