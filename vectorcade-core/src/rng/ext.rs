//! Extension trait for GameRng with generic methods.

use super::GameRng;

/// Extension trait for picking random elements (not dyn-compatible).
pub trait GameRngExt: GameRng {
    /// Pick a random element from a slice.
    fn pick<'a, T>(&mut self, slice: &'a [T]) -> Option<&'a T> {
        self.pick_index(slice.len()).map(|i| &slice[i])
    }
}

impl<R: GameRng> GameRngExt for R {}
