/// RGBA color with components in 0.0..1.0 range.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Rgba(pub f32, pub f32, pub f32, pub f32);

impl Rgba {
    // Classic vector display colors
    pub const BLACK: Rgba = Rgba(0.0, 0.0, 0.0, 1.0);
    pub const WHITE: Rgba = Rgba(1.0, 1.0, 1.0, 1.0);
    pub const GREEN: Rgba = Rgba(0.0, 1.0, 0.0, 1.0);

    // Extended palette for colored vector games (Tempest, etc.)
    pub const RED: Rgba = Rgba(1.0, 0.0, 0.0, 1.0);
    pub const BLUE: Rgba = Rgba(0.0, 0.0, 1.0, 1.0);
    pub const YELLOW: Rgba = Rgba(1.0, 1.0, 0.0, 1.0);
    pub const CYAN: Rgba = Rgba(0.0, 1.0, 1.0, 1.0);
    pub const MAGENTA: Rgba = Rgba(1.0, 0.0, 1.0, 1.0);
    pub const ORANGE: Rgba = Rgba(1.0, 0.5, 0.0, 1.0);

    /// Phosphor green (classic vector monitor color).
    pub const PHOSPHOR: Rgba = Rgba(0.2, 1.0, 0.2, 1.0);

    /// Create color from RGB, alpha defaults to 1.0.
    #[must_use]
    pub const fn rgb(r: f32, g: f32, b: f32) -> Rgba {
        Rgba(r, g, b, 1.0)
    }

    /// Return a copy with modified alpha.
    #[must_use]
    pub const fn with_a(self, a: f32) -> Rgba {
        Rgba(self.0, self.1, self.2, a)
    }

    /// Linearly interpolate between two colors.
    #[must_use]
    pub fn lerp(self, other: Rgba, t: f32) -> Rgba {
        Rgba(
            self.0 + (other.0 - self.0) * t,
            self.1 + (other.1 - self.1) * t,
            self.2 + (other.2 - self.2) * t,
            self.3 + (other.3 - self.3) * t,
        )
    }
}

impl Default for Rgba {
    fn default() -> Self {
        Self::WHITE
    }
}
