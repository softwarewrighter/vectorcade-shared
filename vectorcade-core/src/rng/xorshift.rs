//! Xorshift64 RNG implementation.

use super::GameRng;

/// Simple xorshift64 RNG - fast and good enough for games.
///
/// Not cryptographically secure, but deterministic and portable.
#[derive(Clone, Debug)]
pub struct Xorshift64 {
    state: u64,
}

impl Xorshift64 {
    /// Create a new RNG with the given seed.
    ///
    /// Seed must be non-zero; zero seeds are replaced with a default.
    #[must_use]
    pub const fn new(seed: u64) -> Self {
        let state = if seed == 0 {
            0xDEAD_BEEF_CAFE_BABE
        } else {
            seed
        };
        Self { state }
    }

    /// Create with a default seed (useful for testing).
    #[must_use]
    pub const fn default_seed() -> Self {
        Self::new(0x1234_5678_9ABC_DEF0)
    }
}

impl GameRng for Xorshift64 {
    fn next_u32(&mut self) -> u32 {
        self.next_u64() as u32
    }

    fn next_u64(&mut self) -> u64 {
        let mut x = self.state;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.state = x;
        x
    }
}

impl Default for Xorshift64 {
    fn default() -> Self {
        Self::default_seed()
    }
}
