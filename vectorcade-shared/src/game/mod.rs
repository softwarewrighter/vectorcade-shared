//! Game lifecycle traits and context.

mod coords;
mod ctx;

pub use coords::{ndc_to_px, px_to_ndc};
pub use ctx::{Assets, AudioOut, GameCtx, GameMeta, ScreenInfo};

use crate::draw::DrawCmd;

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
