//! 2D transformation matrix builders.
//!
//! Create affine transformation matrices for use with `DrawCmd::PushTransform`.
//! Matrices are in column-major order compatible with glam.

use glam::{Mat3, Vec2};

/// Create a 2D rotation matrix.
///
/// Rotates counter-clockwise around the origin by `theta_rad` radians.
#[must_use]
pub fn rot2(theta_rad: f32) -> Mat3 {
    let (s, c) = theta_rad.sin_cos();
    Mat3::from_cols_array(&[c, s, 0.0, -s, c, 0.0, 0.0, 0.0, 1.0])
}

/// Create a 2D translation matrix.
///
/// Translates by `(t.x, t.y)` when applied to points.
#[must_use]
pub fn translate2(t: Vec2) -> Mat3 {
    Mat3::from_cols_array(&[1.0, 0.0, 0.0, 0.0, 1.0, 0.0, t.x, t.y, 1.0])
}

/// Create a 2D non-uniform scale matrix.
///
/// Scales by `s.x` horizontally and `s.y` vertically.
#[must_use]
pub fn scale2(s: Vec2) -> Mat3 {
    Mat3::from_cols_array(&[s.x, 0.0, 0.0, 0.0, s.y, 0.0, 0.0, 0.0, 1.0])
}

/// Create a 2D uniform scale matrix.
///
/// Scales equally in both dimensions by factor `s`.
#[must_use]
pub fn scale2_uniform(s: f32) -> Mat3 {
    scale2(Vec2::splat(s))
}
