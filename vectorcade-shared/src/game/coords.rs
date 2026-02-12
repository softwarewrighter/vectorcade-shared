//! Coordinate conversion utilities.

use super::ScreenInfo;
use glam::Vec2;

/// Map pixel coordinates to normalized device coordinates (-1..1).
///
/// Converts from screen space (origin top-left, Y-down) to
/// world space (origin center, Y-up).
#[must_use]
pub fn px_to_ndc(p: Vec2, screen: ScreenInfo) -> Vec2 {
    let w = screen.width_px.max(1) as f32;
    let h = screen.height_px.max(1) as f32;
    Vec2::new((p.x / w) * 2.0 - 1.0, 1.0 - (p.y / h) * 2.0)
}

/// Map normalized device coordinates (-1..1) to pixel coordinates.
#[must_use]
pub fn ndc_to_px(p: Vec2, screen: ScreenInfo) -> Vec2 {
    let w = screen.width_px.max(1) as f32;
    let h = screen.height_px.max(1) as f32;
    Vec2::new((p.x + 1.0) * 0.5 * w, (1.0 - p.y) * 0.5 * h)
}
