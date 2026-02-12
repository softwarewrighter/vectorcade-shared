use crate::{draw::DrawCmd, input::InputState, rng::GameRng};
use glam::Vec2;

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

/// A game produces a display-list each frame; renderer consumes it.
///
/// Games implement this trait to define their logic and rendering.
/// The engine calls `update` at a fixed timestep for determinism,
/// then `render` to collect draw commands.
pub trait Game {
    /// Return metadata about this game.
    fn metadata(&self) -> GameMeta;

    /// Reset game state (called on start and restart).
    fn reset(&mut self, _ctx: &mut GameCtx) {}

    /// Fixed-timestep update for determinism.
    ///
    /// `dt` is the fixed timestep in seconds (typically 1/60).
    fn update(&mut self, ctx: &mut GameCtx, dt: f32);

    /// Append draw commands for current frame.
    ///
    /// Games should push commands to `out`, not clear it
    /// (allows layering multiple games or overlays).
    fn render(&mut self, ctx: &mut GameCtx, out: &mut Vec<DrawCmd>);
}

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
