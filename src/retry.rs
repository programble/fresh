//! Retry iterator.

use std::time::Duration;

/// Retry iterator.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Retry {
    /// Number of tries.
    pub tries: u32,

    /// Interval between tries.
    pub interval: Duration,
}

impl Retry {
    /// Creates an iterator which only tries once.
    pub fn none() -> Self {
        Retry { tries: 1, interval: Duration::new(0, 0) }
    }

    /// Creates a retry iterator.
    pub fn new(tries: u32, interval: Duration) -> Self {
        Retry  { tries: tries, interval: interval }
    }
}

impl Iterator for Retry {
    type Item = ();

    fn next(&mut self) -> Option<()> {
        None
    }
}
