//! VectorCade shared API contract.
//!
//! This crate must remain pure Rust and platform-agnostic.
//!
//! # Modules
//!
//! - [`color`] - RGBA colors with classic vector display constants
//! - [`collision`] - AABB and circle collision detection
//! - [`draw`] - Display-list drawing commands
//! - [`font`] - Vector font traits and glyph types
//! - [`game`] - Game lifecycle trait and context
//! - [`input`] - Input abstraction (keyboard, axes, pointer)
//! - [`math`] - 2D/3D math helpers
//! - [`rng`] - Seedable RNG for deterministic gameplay

pub mod collision;
pub mod color;
pub mod draw;
pub mod font;
pub mod game;
pub mod input;
pub mod math;
pub mod rng;
