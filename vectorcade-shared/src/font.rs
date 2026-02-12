use glam::Vec2;

/// Identify a font "look" (to avoid every game having identical glyphs).
/// Think: Atari, Midway, Cinematronics-ish vibes.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct FontStyleId(pub u32);

impl FontStyleId {
    pub const DEFAULT: FontStyleId = FontStyleId(0);
    pub const ATARI: FontStyleId = FontStyleId(1);
    pub const CINEMATRONICS: FontStyleId = FontStyleId(2);
    pub const MIDWAY: FontStyleId = FontStyleId(3);
    pub const VECTOR_SCANLINE: FontStyleId = FontStyleId(4);
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GlyphStroke {
    pub width_px: f32,
}

#[derive(Clone, Debug, PartialEq)]
pub enum GlyphPathCmd {
    MoveTo(Vec2),
    LineTo(Vec2),
    Close,
}

#[derive(Clone, Debug, PartialEq)]
pub struct GlyphPath {
    /// Coordinates are in font-local space, typically (0..1) units.
    pub cmds: Vec<GlyphPathCmd>,
}

/// Vector font provider. Concrete implementations live in `vectorcade-fonts`.
pub trait VectorFont {
    fn style_id(&self) -> FontStyleId;
    fn has_glyph(&self, ch: char) -> bool;

    /// Return one or more stroke paths for the glyph.
    /// Multiple paths enables "broken" or "segmented" glyph aesthetics.
    fn glyph_paths(&self, ch: char) -> Vec<GlyphPath>;

    /// Suggested advance in font units.
    fn advance(&self, ch: char) -> f32;
}
