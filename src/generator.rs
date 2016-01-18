//! Password generators.

use std::iter;
use rand::{self, Rng};
use rustc_serialize::hex::ToHex;
use rustc_serialize::base64::{self, ToBase64};

/// Variable-length password generator.
pub trait Generator {
    /// Generate a password of the desired length.
    fn generate(&self, length: usize) -> String;
}

/// Generates passwords filled with a single character.
///
/// Only suitable for testing.
#[derive(Debug)]
pub struct Char(pub char);
impl Generator for Char {
    fn generate(&self, length: usize) -> String {
        iter::repeat(self.0).take(length).collect()
    }
}

/// Generates random hexadecimal passwords.
#[derive(Debug)]
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

/// Generates random base64 passwords.
// FIXME: New rustc_serialize release will allow deriving Debug.
#[allow(missing_debug_implementations)]
pub struct Base64(pub base64::Config);
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
impl Default for Base64 {
    fn default() -> Self { Base64(base64::URL_SAFE) }
}

#[cfg(test)]
mod tests {
    use super::{Generator, Char, Hex, Base64};

    fn test_length<G: Generator>(gen: G) {
        for n in 1..33 {
            assert_eq!(n, gen.generate(n).len());
        }
    }

    #[test]
    fn test_char_length() {
        test_length(Char('a'));
    }

    #[test]
    fn test_hex_length() {
        test_length(Hex);
    }

    #[test]
    fn test_base64_length() {
        test_length(Base64::default());
    }
}
