//! Fresh.

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

extern crate chrono;
extern crate hyper;
extern crate inth_oauth2;
extern crate rand;
extern crate rustc_serialize;
extern crate yup_oauth2;

pub mod generator;
pub mod token_cache;
