//! Password generators.

use std::iter;
use rand::{self, Rng};
use rustc_serialize::hex::ToHex;
use rustc_serialize::base64::{self, ToBase64};

/// Variable-length password generator.
pub trait Generator {
    /// Generates a password of the desired length.
    fn generate(&self, length: usize) -> String;
}

/// Creates a boxed instance of a [`Generator`](trait.Generator.html) from a string.
///
/// - `"char"`: [`Char`](struct.Char.html)
/// - `"hex"`: [`Hex`](struct.Hex.html)
/// - `"base64"`: [`Base64`](struct.Base64.html)
pub fn instance(ty: &str) -> Option<Box<Generator>> {
    match ty {
        "char" => Some(Box::new(Char::default())),
        "hex" => Some(Box::new(Hex::default())),
        "base64" => Some(Box::new(Base64::default())),
        _ => None,
    }
}

/// Generator of passwords filled with a single character.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Char(pub char);

impl Default for Char {
    fn default() -> Self { Char('a') }
}

impl Generator for Char {
    fn generate(&self, length: usize) -> String {
        iter::repeat(self.0).take(length).collect()
    }
}

/// Generator of random hexadecimal passwords.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Hex;

impl Generator for Hex {
    fn generate(&self, length: usize) -> String {
        let bytes = (length + 1) / 2;
        let mut vec = vec![0u8; bytes];
        rand::thread_rng().fill_bytes(&mut vec[..]);

        let mut hex = vec.to_hex();
        hex.truncate(length);
        hex
    }
}

/// Generator of random base64 passwords.
#[derive(Debug, Clone)]
pub struct Base64(pub base64::Config);

impl Default for Base64 {
    fn default() -> Self { Base64(base64::URL_SAFE) }
}

impl Generator for Base64 {
    fn generate(&self, length: usize) -> String {
        let bytes = length * 4 / 3;
        let mut vec = vec![0u8; bytes];
        rand::thread_rng().fill_bytes(&mut vec[..]);

        let mut base64 = vec.to_base64(self.0);
        base64.truncate(length);
        base64
    }
}

#[cfg(test)]
mod tests {
    use super::{instance, Generator, Char, Hex, Base64};

    #[test]
    fn instance_char() {
        let _ = instance("char").unwrap();
    }

    #[test]
    fn instance_hex() {
        let _ = instance("hex").unwrap();
    }

    #[test]
    fn instance_base64() {
        let _ = instance("base64").unwrap();
    }

    fn test_length<G: Generator>(gen: G) {
        for n in 1..33 {
            assert_eq!(n, gen.generate(n).len());
        }
    }

    #[test]
    fn char_length() {
        test_length(Char::default());
    }

    #[test]
    fn hex_length() {
        test_length(Hex::default());
    }

    #[test]
    fn base64_length() {
        test_length(Base64::default());
    }
}
