//! 2D transformation matrices.

use glam::{Mat3, Vec2};

/// Create a 2D rotation matrix (around Z axis).
#[must_use]
pub fn rot2(theta_rad: f32) -> Mat3 {
    let (s, c) = theta_rad.sin_cos();
    Mat3::from_cols_array(&[c, s, 0.0, -s, c, 0.0, 0.0, 0.0, 1.0])
}

/// Create a 2D translation matrix.
#[must_use]
pub fn translate2(t: Vec2) -> Mat3 {
    Mat3::from_cols_array(&[1.0, 0.0, 0.0, 0.0, 1.0, 0.0, t.x, t.y, 1.0])
}

/// Create a 2D scale matrix.
#[must_use]
pub fn scale2(s: Vec2) -> Mat3 {
    Mat3::from_cols_array(&[s.x, 0.0, 0.0, 0.0, s.y, 0.0, 0.0, 0.0, 1.0])
}

/// Create a uniform 2D scale matrix.
#[must_use]
pub fn scale2_uniform(s: f32) -> Mat3 {
    scale2(Vec2::splat(s))
}
