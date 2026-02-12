use crate::color::Rgba;
use glam::{Mat3, Vec2};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Stroke {
    pub color: Rgba,
    /// Width in *screen pixels* to mimic vector display beams.
    pub width_px: f32,
    /// Optional "phosphor glow" hint; renderer may ignore.
    pub glow: f32,
}

impl Stroke {
    pub fn new(color: Rgba, width_px: f32) -> Self {
        Self { color, width_px, glow: 0.0 }
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
    Clear { color: Rgba },

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
    BeginLayer { name: &'static str },
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
