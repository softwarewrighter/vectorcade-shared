//! VectorCade math utilities.
//!
//! Math helpers and collision detection for 2D/3D vector games:
//!
//! - [`collision`] - AABB, circle, and line intersection tests
//! - [`helpers`] - Interpolation, clamping, and screen wrapping
//! - [`projection`] - 3D perspective projection and angle utilities
//! - [`transform`] - 2D transformation matrix builders

pub mod collision;
pub mod helpers;
pub mod projection;
pub mod transform;

// Re-export commonly used items
pub use collision::{Aabb, Circle, line_aabb_intersect, line_circle_intersect};
pub use helpers::{clamp, inv_lerp, lerp, remap, wrap_position, wrap_range, wrap_signed_unit};
pub use projection::{angle_diff, depth_intensity, normalize_angle, project_persp};
pub use transform::{rot2, scale2, scale2_uniform, translate2};
