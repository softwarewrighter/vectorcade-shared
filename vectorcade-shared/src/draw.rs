//! Display-list drawing primitives.
//!
//! Games emit a `Vec<DrawCmd>` each frame describing what to render.
//! The renderer consumes these commands to produce the final image.

use glam::{Mat3, Vec2};
use vectorcade_core::Rgba;

/// Stroke style for vector lines.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Stroke {
    pub color: Rgba,
    /// Width in screen pixels (mimics vector display beam width).
    pub width_px: f32,
    /// Phosphor glow intensity hint (0.0 = none, 1.0 = full).
    /// Renderer may ignore this.
    pub glow: f32,
}

impl Stroke {
    /// Create a stroke with given color and width, no glow.
    #[must_use]
    pub const fn new(color: Rgba, width_px: f32) -> Self {
        Self {
            color,
            width_px,
            glow: 0.0,
        }
    }

    /// Create a stroke with glow effect.
    #[must_use]
    pub const fn with_glow(color: Rgba, width_px: f32, glow: f32) -> Self {
        Self {
            color,
            width_px,
            glow,
        }
    }
}

impl Default for Stroke {
    fn default() -> Self {
        Self::new(Rgba::WHITE, 1.0)
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Line2 {
    pub a: Vec2,
    pub b: Vec2,
    pub stroke: Stroke,
}

#[derive(Clone, Debug, PartialEq)]
pub enum DrawCmd {
    Clear {
        color: Rgba,
    },

    Line(Line2),

    Polyline {
        pts: Vec<Vec2>,
        closed: bool,
        stroke: Stroke,
    },

    /// Optional early text support; can be implemented by a vector-font backend.
    Text {
        pos: Vec2,
        text: String,
        size_px: f32,
        color: Rgba,
        style: crate::font::FontStyleId,
    },

    PushTransform(Mat3),
    PopTransform,

    /// Optional: a way to group commands (helps render backends batch).
    BeginLayer {
        name: &'static str,
    },
    EndLayer,
}

/// Helper for building a "wire rectangle" in world coords.
pub fn rect_wire(min: Vec2, max: Vec2, stroke: Stroke) -> DrawCmd {
    DrawCmd::Polyline {
        pts: vec![
            Vec2::new(min.x, min.y),
            Vec2::new(max.x, min.y),
            Vec2::new(max.x, max.y),
            Vec2::new(min.x, max.y),
        ],
        closed: true,
        stroke,
    }
}
