//! VectorCade core types.
//!
//! Basic types with no external dependencies:
//! - [`Rgba`] - Color type
//! - [`GameRng`] - RNG trait for deterministic gameplay
//! - [`Xorshift64`] - Fast RNG implementation

pub mod color;
pub mod rng;

pub use color::Rgba;
pub use rng::{GameRng, GameRngExt, Xorshift64};
