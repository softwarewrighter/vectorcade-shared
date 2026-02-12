//! VectorCade shared API contracts.
//!
//! This crate provides the game-facing API:
//! - [`draw`] - Display-list drawing commands
//! - [`font`] - Vector font traits
//! - [`game`] - Game lifecycle trait and context
//! - [`input`] - Input abstraction
//! - [`projectile`] - 2D and 3D projectile systems
//!
//! Core types and math are re-exported from dependencies:
//! - [`vectorcade_core`] - Rgba, GameRng, Xorshift64
//! - [`vectorcade_math`] - Math helpers, collision

pub mod draw;
pub mod font;
pub mod game;
pub mod input;
pub mod projectile;

// Re-export core types
pub use vectorcade_core::{GameRng, GameRngExt, Rgba, Xorshift64};

// Re-export math types
pub use vectorcade_math::{
    Aabb, Circle, angle_diff, clamp, depth_intensity, inv_lerp, lerp, line_aabb_intersect,
    line_circle_intersect, normalize_angle, project_line_3d, project_persp, remap, rot2,
    rotate_point_y, scale2, scale2_uniform, translate2, wrap_position, wrap_range,
    wrap_signed_unit,
};
