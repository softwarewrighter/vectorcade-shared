//! General math helper functions.
//!
//! Common operations for game math: interpolation, clamping, and
//! screen-wrapping for toroidal gameplay (Asteroids-style).

use glam::Vec2;

/// Clamp a value to a range `[lo, hi]`.
///
/// Returns `lo` if `x < lo`, `hi` if `x > hi`, otherwise `x`.
#[must_use]
pub fn clamp(x: f32, lo: f32, hi: f32) -> f32 {
    x.max(lo).min(hi)
}

/// Wrap a value into the range `[-1, 1)`.
///
/// Useful for normalized screen coordinates where objects wrap around edges.
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

/// Wrap a value into a custom range `[lo, hi)`.
///
/// The value wraps around when it exceeds either bound.
/// Returns `lo` if the range is empty or invalid.
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
///
/// When `t = 0`, returns `a`. When `t = 1`, returns `b`.
/// Values of `t` outside `[0, 1]` extrapolate beyond the range.
#[must_use]
pub fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}

/// Inverse linear interpolation.
///
/// Finds `t` such that `lerp(a, b, t) = v`.
/// Returns 0.0 if `a` and `b` are nearly equal.
#[must_use]
pub fn inv_lerp(a: f32, b: f32, v: f32) -> f32 {
    if (b - a).abs() < 1e-9 {
        0.0
    } else {
        (v - a) / (b - a)
    }
}

/// Remap a value from one range to another.
///
/// Equivalent to `lerp(to_lo, to_hi, inv_lerp(from_lo, from_hi, v))`.
#[must_use]
pub fn remap(v: f32, from_lo: f32, from_hi: f32, to_lo: f32, to_hi: f32) -> f32 {
    let t = inv_lerp(from_lo, from_hi, v);
    lerp(to_lo, to_hi, t)
}

/// Wrap a 2D position to stay within [-1, 1) on both axes.
///
/// Used for Asteroids-style screen wrapping.
#[must_use]
pub fn wrap_position(p: Vec2) -> Vec2 {
    Vec2::new(wrap_signed_unit(p.x), wrap_signed_unit(p.y))
}
