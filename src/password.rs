//! Password generation.

use std::iter::{self, FromIterator};
use rand::{self, Rng};
use rustc_serialize::base64::{self, ToBase64};

/// Variable-length password generator.
pub trait Generator: Default {
    /// Generate a password of the desired length.
    fn generate(&mut self, length: usize) -> String;
}

/// Dummy password generator.
///
/// Generates passwords filled with a single character.
pub struct DummyGen(char);

impl Default for DummyGen {
    fn default() -> Self {
        DummyGen('a')
    }
}

impl Generator for DummyGen {
    fn generate(&mut self, length: usize) -> String {
        String::from_iter(iter::repeat(self.0).take(length))
    }
}

/// Random base-64 password generator.
pub struct Base64Gen {
    rng: rand::ThreadRng,
    config: base64::Config,
}

impl Base64Gen {
    /// Create a new `Base64Gen` using the thread-local RNG and the supplied
    /// base-64 configuration.
    pub fn new(config: base64::Config) -> Self {
        Base64Gen {
            rng: rand::thread_rng(),
            config: config,
        }
    }
}

impl Default for Base64Gen {
    fn default() -> Self {
        Base64Gen::new(base64::URL_SAFE)
    }
}

impl Generator for Base64Gen {
    fn generate(&mut self, length: usize) -> String {
        let bytes = length * 4 / 3;

        let mut vec = vec![0u8; bytes];
        let slice = &mut vec[..];
        self.rng.fill_bytes(slice);

        let mut string = slice.to_base64(self.config);
        string.truncate(length);

        string
    }
}

#[cfg(test)]
mod tests {
    use super::Generator;

    #[test]
    fn test_dummy() {
        let mut gen = super::DummyGen::default();
        assert_eq!("a", gen.generate(1));
        assert_eq!("aa", gen.generate(2));
        assert_eq!("aaa", gen.generate(3));
    }

    #[test]
    fn test_base64() {
        let mut gen = super::Base64Gen::default();
        for n in 1..6 {
            assert_eq!(n, gen.generate(n).len());
        }
    }
}
