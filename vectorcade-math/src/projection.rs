//! 3D projection and angle helpers for 2.5D games.
//!
//! Provides perspective projection for pseudo-3D effects (like Tempest or
//! Star Wars) and angle manipulation utilities.

use glam::{Vec2, Vec3};
use std::f32::consts::PI;

/// Project a 3D point to 2D screen coordinates.
///
/// Uses perspective projection with the camera looking down the -Z axis.
/// Points behind the camera (positive Z) return `None`.
///
/// # Arguments
///
/// * `p` - Point in 3D world space
/// * `fov_y_rad` - Vertical field of view in radians
/// * `aspect` - Aspect ratio (width / height)
///
/// # Returns
///
/// 2D point in normalized device coordinates, or `None` if behind camera.
#[must_use]
pub fn project_persp(p: Vec3, fov_y_rad: f32, aspect: f32) -> Option<Vec2> {
    let z = -p.z;
    if z <= 1e-3 {
        return None;
    }
    let f = 1.0 / (0.5 * fov_y_rad).tan();
    let x = (p.x * f / aspect) / z;
    let y = (p.y * f) / z;
    Some(Vec2::new(x, y))
}

/// Distance-based intensity falloff for depth cueing.
///
/// Returns a value in [0, 1] that decreases with distance.
#[must_use]
pub fn depth_intensity(distance: f32, near: f32, far: f32) -> f32 {
    if distance <= near {
        1.0
    } else if distance >= far {
        0.0
    } else {
        1.0 - (distance - near) / (far - near)
    }
}

/// Normalize an angle to [-PI, PI).
#[must_use]
pub fn normalize_angle(mut angle: f32) -> f32 {
    while angle < -PI {
        angle += 2.0 * PI;
    }
    while angle >= PI {
        angle -= 2.0 * PI;
    }
    angle
}

/// Calculate the shortest angular distance between two angles.
#[must_use]
pub fn angle_diff(from: f32, to: f32) -> f32 {
    normalize_angle(to - from)
}
