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

extern crate hyper;
extern crate rand;
extern crate rustc_serialize;
extern crate scraper;
extern crate url;

pub mod generator;
pub mod account;
