//! Vector font traits and glyph path types.
//!
//! This module defines the interface for stroke-based vector fonts.
//! Concrete font implementations live in the `vectorcade-fonts` crate.

use glam::Vec2;

/// Identifier for a font style/aesthetic.
///
/// Different styles evoke different eras of vector arcade games:
/// - `ATARI` - Asteroids, Tempest style
/// - `CINEMATRONICS` - Star Castle, Armor Attack style
/// - `MIDWAY` - Omega Race style
///
/// Font providers may support multiple styles.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct FontStyleId(pub u32);

impl FontStyleId {
    /// Default font style (implementation-defined).
    pub const DEFAULT: FontStyleId = FontStyleId(0);
    /// Atari vector game aesthetic.
    pub const ATARI: FontStyleId = FontStyleId(1);
    /// Cinematronics vector game aesthetic.
    pub const CINEMATRONICS: FontStyleId = FontStyleId(2);
    /// Midway vector game aesthetic.
    pub const MIDWAY: FontStyleId = FontStyleId(3);
    /// Scanline/raster-style vector font.
    pub const VECTOR_SCANLINE: FontStyleId = FontStyleId(4);
}

/// Stroke width for glyph rendering.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GlyphStroke {
    /// Stroke width in pixels.
    pub width_px: f32,
}

/// A single path command for drawing glyph strokes.
#[derive(Clone, Debug, PartialEq)]
pub enum GlyphPathCmd {
    /// Move to a position without drawing.
    MoveTo(Vec2),
    /// Draw a line to a position.
    LineTo(Vec2),
    /// Close the current sub-path back to its start.
    Close,
}

/// A stroke path for rendering a glyph.
///
/// Glyphs may consist of multiple paths for segmented/broken aesthetics.
#[derive(Clone, Debug, PartialEq)]
pub struct GlyphPath {
    /// Path commands in font-local coordinates (typically 0..1).
    pub cmds: Vec<GlyphPathCmd>,
}

/// Trait for stroke-based vector font providers.
///
/// Implementations provide glyph geometry as stroke paths that can be
/// rendered as vector lines. Concrete implementations live in `vectorcade-fonts`.
pub trait VectorFont {
    /// Return the style identifier for this font.
    fn style_id(&self) -> FontStyleId;

    /// Check if this font has a glyph for the given character.
    fn has_glyph(&self, ch: char) -> bool;

    /// Return stroke paths for rendering a glyph.
    ///
    /// Returns one or more paths; multiple paths enable "broken" or
    /// "segmented" glyph aesthetics common in vector arcade fonts.
    fn glyph_paths(&self, ch: char) -> Vec<GlyphPath>;

    /// Return the horizontal advance width for a character.
    ///
    /// Value is in font units (typically 0..1 scale).
    fn advance(&self, ch: char) -> f32;
}
