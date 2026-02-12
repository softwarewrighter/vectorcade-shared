//! Math helpers for 2D and 3D vector game math.

use glam::{Mat3, Vec2, Vec3};

/// Clamp a value to a range.
#[must_use]
pub fn clamp(x: f32, lo: f32, hi: f32) -> f32 {
    x.max(lo).min(hi)
}

/// Wrap a value into the range [-1, 1).
#[must_use]
pub fn wrap_signed_unit(mut x: f32) -> f32 {
    while x < -1.0 {
        x += 2.0;
    }
    while x >= 1.0 {
        x -= 2.0;
    }
    x
}

/// Wrap a 2D position to stay within [-1, 1) on both axes.
///
/// Used for Asteroids-style screen wrapping.
#[must_use]
pub fn wrap_position(p: Vec2) -> Vec2 {
    Vec2::new(wrap_signed_unit(p.x), wrap_signed_unit(p.y))
}

/// Wrap a value into a custom range [lo, hi).
#[must_use]
pub fn wrap_range(mut x: f32, lo: f32, hi: f32) -> f32 {
    let range = hi - lo;
    if range <= 0.0 {
        return lo;
    }
    while x < lo {
        x += range;
    }
    while x >= hi {
        x -= range;
    }
    x
}

/// Linear interpolation between two values.
#[must_use]
pub fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}

/// Inverse linear interpolation: find t such that lerp(a, b, t) = v.
#[must_use]
pub fn inv_lerp(a: f32, b: f32, v: f32) -> f32 {
    if (b - a).abs() < 1e-9 {
        0.0
    } else {
        (v - a) / (b - a)
    }
}

/// Remap a value from one range to another.
#[must_use]
pub fn remap(v: f32, from_lo: f32, from_hi: f32, to_lo: f32, to_hi: f32) -> f32 {
    let t = inv_lerp(from_lo, from_hi, v);
    lerp(to_lo, to_hi, t)
}

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

/// Normalize an angle to [-PI, PI).
#[must_use]
pub fn normalize_angle(mut angle: f32) -> f32 {
    use std::f32::consts::PI;
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

/// Minimal 3D→2D perspective projection for "2.5D vector" games.
///
/// Camera looks down -Z axis. Returns `None` if point is behind camera.
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
