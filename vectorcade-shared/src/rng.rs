//! Seedable RNG abstraction for deterministic gameplay.
//!
//! Games should use the RNG provided in `GameCtx` rather than
//! system randomness to enable replays and deterministic testing.

/// Minimal RNG trait for game use.
///
/// Implementations should be seedable and deterministic.
pub trait GameRng {
    /// Return a random u32.
    fn next_u32(&mut self) -> u32;

    /// Return a random u64.
    fn next_u64(&mut self) -> u64 {
        let hi = self.next_u32() as u64;
        let lo = self.next_u32() as u64;
        (hi << 32) | lo
    }

    /// Return a random f32 in [0.0, 1.0).
    fn next_f32(&mut self) -> f32 {
        // Use upper 23 bits for mantissa
        (self.next_u32() >> 9) as f32 * (1.0 / (1u32 << 23) as f32)
    }

    /// Return a random f32 in [lo, hi).
    fn range_f32(&mut self, lo: f32, hi: f32) -> f32 {
        lo + self.next_f32() * (hi - lo)
    }

    /// Return a random i32 in [lo, hi) (exclusive upper bound).
    fn range_i32(&mut self, lo: i32, hi: i32) -> i32 {
        if lo >= hi {
            return lo;
        }
        let range = (hi - lo) as u32;
        lo + (self.next_u32() % range) as i32
    }

    /// Return true with probability `p` (0.0 = never, 1.0 = always).
    fn chance(&mut self, p: f32) -> bool {
        self.next_f32() < p
    }

    /// Return a random index for a slice of given length.
    ///
    /// Returns `None` if `len` is 0.
    fn pick_index(&mut self, len: usize) -> Option<usize> {
        if len == 0 {
            None
        } else {
            Some(self.next_u32() as usize % len)
        }
    }
}

/// Extension trait for picking random elements (not dyn-compatible).
pub trait GameRngExt: GameRng {
    /// Pick a random element from a slice.
    fn pick<'a, T>(&mut self, slice: &'a [T]) -> Option<&'a T> {
        self.pick_index(slice.len()).map(|i| &slice[i])
    }
}

impl<R: GameRng> GameRngExt for R {}

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
        let state = if seed == 0 { 0xDEAD_BEEF_CAFE_BABE } else { seed };
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deterministic() {
        let mut rng1 = Xorshift64::new(42);
        let mut rng2 = Xorshift64::new(42);

        for _ in 0..100 {
            assert_eq!(rng1.next_u64(), rng2.next_u64());
        }
    }

    #[test]
    fn range_f32_bounds() {
        let mut rng = Xorshift64::new(123);
        for _ in 0..1000 {
            let v = rng.range_f32(-1.0, 1.0);
            assert!(v >= -1.0 && v < 1.0);
        }
    }

    #[test]
    fn range_i32_bounds() {
        let mut rng = Xorshift64::new(456);
        for _ in 0..1000 {
            let v = rng.range_i32(0, 10);
            assert!(v >= 0 && v < 10);
        }
    }

    #[test]
    fn pick_index_works() {
        let mut rng = Xorshift64::new(789);
        for _ in 0..100 {
            let idx = rng.pick_index(5).unwrap();
            assert!(idx < 5);
        }
        assert!(rng.pick_index(0).is_none());
    }

    #[test]
    fn pick_ext_works() {
        let mut rng = Xorshift64::new(789);
        let items = [1, 2, 3, 4, 5];
        for _ in 0..100 {
            let picked = rng.pick(&items).unwrap();
            assert!(items.contains(picked));
        }
    }
}
