#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Rgba(pub f32, pub f32, pub f32, pub f32);

impl Rgba {
    pub const BLACK: Rgba = Rgba(0.0, 0.0, 0.0, 1.0);
    pub const WHITE: Rgba = Rgba(1.0, 1.0, 1.0, 1.0);
    pub const GREEN: Rgba = Rgba(0.0, 1.0, 0.0, 1.0);

    pub fn with_a(self, a: f32) -> Rgba { Rgba(self.0, self.1, self.2, a) }
}
