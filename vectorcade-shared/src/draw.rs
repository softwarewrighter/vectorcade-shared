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

/// A line segment with stroke style.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Line2 {
    /// Start point of the line.
    pub a: Vec2,
    /// End point of the line.
    pub b: Vec2,
    /// Stroke style (color, width, glow).
    pub stroke: Stroke,
}

/// Display-list command for vector rendering.
///
/// Games emit a `Vec<DrawCmd>` each frame. The renderer processes
/// these commands in order to produce the final image.
#[derive(Clone, Debug, PartialEq)]
pub enum DrawCmd {
    /// Fill the entire screen with a solid color.
    Clear {
        /// Background color.
        color: Rgba,
    },

    /// Draw a single line segment.
    Line(Line2),

    /// Draw a series of connected line segments.
    Polyline {
        /// Points forming the polyline path.
        pts: Vec<Vec2>,
        /// If true, connect the last point back to the first.
        closed: bool,
        /// Stroke style for all segments.
        stroke: Stroke,
    },

    /// Draw text using a vector font.
    ///
    /// Requires a font backend that implements the `VectorFont` trait.
    Text {
        /// Position of the text baseline origin.
        pos: Vec2,
        /// The text string to render.
        text: String,
        /// Font size in screen pixels.
        size_px: f32,
        /// Text color.
        color: Rgba,
        /// Font style (Atari, Midway, etc.).
        style: crate::font::FontStyleId,
    },

    /// Push a transformation matrix onto the transform stack.
    ///
    /// Subsequent draw commands are transformed by this matrix
    /// until a matching `PopTransform` is issued.
    PushTransform(Mat3),

    /// Pop the most recent transformation from the stack.
    PopTransform,

    /// Begin a named render layer (optional grouping hint).
    ///
    /// Helps render backends optimize batching.
    BeginLayer {
        /// Layer identifier for debugging/profiling.
        name: &'static str,
    },

    /// End the current render layer.
    EndLayer,
}

/// Create a wireframe rectangle from corner coordinates.
///
/// Returns a closed `Polyline` forming a rectangle from `min` to `max`.
///
/// # Example
///
/// ```ignore
/// let rect = rect_wire(
///     Vec2::new(-0.5, -0.5),
///     Vec2::new(0.5, 0.5),
///     Stroke::new(Rgba::GREEN, 2.0),
/// );
/// ```
#[must_use]
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
