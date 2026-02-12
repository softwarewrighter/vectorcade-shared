#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Key {
    Left,
    Right,
    Up,
    Down,
    Space,
    Enter,
    Escape,
    W,
    S,
    Z,
    X,
    C,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Axis {
    MoveX,
    MoveY,
    AimX,
    AimY,
    Thrust,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Button {
    pub is_down: bool,
    pub went_down: bool,
    pub went_up: bool,
}

impl Button {
    pub const UP: Button = Button {
        is_down: false,
        went_down: false,
        went_up: false,
    };
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Pointer {
    pub x_px: f32,
    pub y_px: f32,
    pub is_down: bool,
}

pub trait InputState {
    fn key(&self, k: Key) -> Button;
    fn axis(&self, a: Axis) -> f32; // -1..1
    fn pointer(&self) -> Option<Pointer>;
}
