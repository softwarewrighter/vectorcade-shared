//! 3D projection and angle helpers for 2.5D games.
//!
//! Provides perspective projection for pseudo-3D effects (like Tempest,
//! Battlezone, or Star Wars) and angle manipulation utilities.

use glam::{Vec2, Vec3};
use std::f32::consts::PI;

/// Minimum distance from camera for projection (near plane).
const NEAR_PLANE: f32 = 1e-3;

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
    if z <= NEAR_PLANE {
        return None;
    }
    let f = 1.0 / (0.5 * fov_y_rad).tan();
    let x = (p.x * f / aspect) / z;
    let y = (p.y * f) / z;
    Some(Vec2::new(x, y))
}

/// Project a 3D line segment to 2D with near-plane clipping.
///
/// Handles the case where one endpoint is behind the camera by clipping
/// the line against the near plane. Returns `None` if the entire line
/// is behind the camera.
///
/// # Arguments
///
/// * `a` - First endpoint in 3D world space
/// * `b` - Second endpoint in 3D world space
/// * `fov_y_rad` - Vertical field of view in radians
/// * `aspect` - Aspect ratio (width / height)
///
/// # Returns
///
/// Tuple of projected 2D endpoints, or `None` if entirely behind camera.
#[must_use]
pub fn project_line_3d(a: Vec3, b: Vec3, fov_y_rad: f32, aspect: f32) -> Option<(Vec2, Vec2)> {
    let z_a = -a.z;
    let z_b = -b.z;

    // Both behind camera
    if z_a <= NEAR_PLANE && z_b <= NEAR_PLANE {
        return None;
    }

    // Both in front - simple case
    if z_a > NEAR_PLANE && z_b > NEAR_PLANE {
        let p_a = project_persp(a, fov_y_rad, aspect)?;
        let p_b = project_persp(b, fov_y_rad, aspect)?;
        return Some((p_a, p_b));
    }

    // One point behind camera - clip against near plane
    let (front, behind) = if z_a > NEAR_PLANE { (a, b) } else { (b, a) };
    let z_front = -front.z;
    let z_behind = -behind.z;

    // Interpolate to find intersection with near plane (with small offset to ensure visibility)
    let clip_z = NEAR_PLANE + 1e-4;
    let t = (clip_z - z_front) / (z_behind - z_front);
    let clipped = front + (behind - front) * t;

    let p_front = project_persp(front, fov_y_rad, aspect)?;
    let p_clipped = project_persp(clipped, fov_y_rad, aspect)?;

    // Preserve original order
    if z_a > NEAR_PLANE {
        Some((p_front, p_clipped))
    } else {
        Some((p_clipped, p_front))
    }
}

/// Rotate a 3D point around the Y axis (vertical axis).
///
/// Useful for first-person camera rotation (tank turning in Battlezone)
/// or rotating objects in the world.
///
/// # Arguments
///
/// * `p` - Point to rotate
/// * `angle` - Rotation angle in radians (positive = counter-clockwise when viewed from above)
#[must_use]
pub fn rotate_point_y(p: Vec3, angle: f32) -> Vec3 {
    let (s, c) = angle.sin_cos();
    Vec3::new(p.x * c + p.z * s, p.y, -p.x * s + p.z * c)
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
