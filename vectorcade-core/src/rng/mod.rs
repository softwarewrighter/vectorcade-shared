//! Seedable RNG abstraction for deterministic gameplay.

mod ext;
mod xorshift;

pub use ext::GameRngExt;
pub use xorshift::Xorshift64;

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
    fn pick_index(&mut self, len: usize) -> Option<usize> {
        if len == 0 {
            None
        } else {
            Some(self.next_u32() as usize % len)
        }
    }
}
