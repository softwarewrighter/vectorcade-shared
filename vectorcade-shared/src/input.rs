//! Input abstraction for keyboard, gamepad, and pointer devices.
//!
//! Games read input through the [`InputState`] trait, which abstracts
//! over platform-specific input handling.

/// Keyboard keys commonly used in arcade-style games.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Key {
    /// Left arrow or equivalent.
    Left,
    /// Right arrow or equivalent.
    Right,
    /// Up arrow or equivalent.
    Up,
    /// Down arrow or equivalent.
    Down,
    /// Spacebar (typically fire/action).
    Space,
    /// Enter/return key (typically start/confirm).
    Enter,
    /// Escape key (typically pause/menu).
    Escape,
    /// W key (alternate up).
    W,
    /// S key (alternate down).
    S,
    /// Z key (action button).
    Z,
    /// X key (action button).
    X,
    /// C key (action button).
    C,
}

/// Virtual axes for analog input (gamepad sticks, touch controls).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Axis {
    /// Horizontal movement axis.
    MoveX,
    /// Vertical movement axis.
    MoveY,
    /// Horizontal aim/look axis.
    AimX,
    /// Vertical aim/look axis.
    AimY,
    /// Thrust/acceleration axis (e.g., trigger).
    Thrust,
}

/// State of a digital button (keyboard key or gamepad button).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Button {
    /// True if the button is currently held down.
    pub is_down: bool,
    /// True if the button was pressed this frame.
    pub went_down: bool,
    /// True if the button was released this frame.
    pub went_up: bool,
}

impl Button {
    /// A button in the "not pressed" state.
    pub const UP: Button = Button {
        is_down: false,
        went_down: false,
        went_up: false,
    };
}

/// State of a pointer device (mouse or touch).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Pointer {
    /// X position in screen pixels.
    pub x_px: f32,
    /// Y position in screen pixels.
    pub y_px: f32,
    /// True if the pointer button is down (mouse click or touch active).
    pub is_down: bool,
}

/// Trait for reading input state.
///
/// Platform backends implement this trait to provide input to games.
/// Games should read input through [`GameCtx::input`](crate::game::GameCtx).
pub trait InputState {
    /// Get the state of a keyboard key.
    fn key(&self, k: Key) -> Button;

    /// Get the value of an analog axis.
    ///
    /// Returns a value in the range -1.0 to 1.0.
    /// Returns 0.0 if the axis is not active or not supported.
    fn axis(&self, a: Axis) -> f32;

    /// Get the current pointer state, if available.
    ///
    /// Returns `None` if no pointer device is active.
    fn pointer(&self) -> Option<Pointer>;
}
