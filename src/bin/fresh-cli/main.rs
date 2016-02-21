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
extern crate xdg;

mod authenticator;

fn main() {
}
