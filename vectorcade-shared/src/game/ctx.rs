//! Game context and supporting types.

use crate::input::InputState;
use vectorcade_core::GameRng;

/// Information about the display surface.
#[derive(Clone, Copy, Debug)]
pub struct ScreenInfo {
    pub width_px: u32,
    pub height_px: u32,
    pub dpi_scale: f32,
}

impl ScreenInfo {
    /// Calculate aspect ratio (width / height).
    #[must_use]
    pub fn aspect(&self) -> f32 {
        self.width_px as f32 / self.height_px.max(1) as f32
    }
}

impl Default for ScreenInfo {
    fn default() -> Self {
        Self {
            width_px: 800,
            height_px: 600,
            dpi_scale: 1.0,
        }
    }
}

/// Metadata about a game (name, preferred display settings).
#[derive(Clone, Copy, Debug)]
pub struct GameMeta {
    pub name: &'static str,
    /// Desired aspect ratio for the game's logical coordinate space.
    pub preferred_aspect: Option<f32>,
}

/// Audio output interface (stub for now).
pub trait AudioOut {
    /// Play a sound effect by ID.
    fn beep(&self, _id: &'static str) {}
}

/// Asset loading interface (stub for now).
pub trait Assets {}

/// Context passed to game update/render methods.
pub struct GameCtx<'a> {
    pub input: &'a dyn InputState,
    pub audio: &'a dyn AudioOut,
    pub rng: &'a mut dyn GameRng,
    pub screen: ScreenInfo,
    /// Current game time in seconds (monotonic).
    pub now_s: f64,
}
