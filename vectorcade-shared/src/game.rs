use crate::{draw::DrawCmd, input::InputState};
use glam::Vec2;

#[derive(Clone, Copy, Debug)]
pub struct ScreenInfo {
    pub width_px: u32,
    pub height_px: u32,
    pub dpi_scale: f32,
}

impl ScreenInfo {
    pub fn aspect(&self) -> f32 {
        self.width_px as f32 / self.height_px.max(1) as f32
    }
}

#[derive(Clone, Copy, Debug)]
pub struct GameMeta {
    pub name: &'static str,
    /// Desired aspect ratio for the game's logical coordinate space (optional).
    pub preferred_aspect: Option<f32>,
}

pub trait AudioOut {
    fn beep(&self, _id: &'static str) {}
}

pub trait Assets {}

pub struct GameCtx<'a> {
    pub input: &'a dyn InputState,
    pub audio: &'a dyn AudioOut,
    pub screen: ScreenInfo,
    pub now_s: f64,
}

/// A game produces a display-list each frame; renderer consumes it.
pub trait Game {
    fn metadata(&self) -> GameMeta;

    fn reset(&mut self, _ctx: &mut GameCtx) {}

    /// Fixed-timestep update for determinism.
    fn update(&mut self, ctx: &mut GameCtx, dt: f32);

    /// Append draw commands for current frame.
    fn render(&mut self, ctx: &mut GameCtx, out: &mut Vec<DrawCmd>);
}

/// Helper: map pixel coords into normalized device-ish coordinates (-1..1).
pub fn px_to_ndc(p: Vec2, screen: ScreenInfo) -> Vec2 {
    let w = screen.width_px.max(1) as f32;
    let h = screen.height_px.max(1) as f32;
    // y-down pixels → y-up world
    Vec2::new((p.x / w) * 2.0 - 1.0, 1.0 - (p.y / h) * 2.0)
}
