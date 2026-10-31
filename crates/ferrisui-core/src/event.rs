use crate::geometry::Point;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
    Other(u16),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ElementState {
    Pressed,
    Released,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Modifiers {
    pub shift: bool,
    pub ctrl: bool,
    pub alt: bool,
    pub meta: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum MouseEvent {
    Moved {
        position: Point,
    },
    Button {
        position: Point,
        button: MouseButton,
        state: ElementState,
    },
    Wheel {
        delta_x: f32,
        delta_y: f32,
    },
    Entered,
    Left,
}

#[derive(Debug, Clone, PartialEq)]
pub enum KeyEvent {
    Key {
        key_code: u32,
        state: ElementState,
        modifiers: Modifiers,
    },
    Character(char),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Event {
    Mouse(MouseEvent),
    Key(KeyEvent),
    WindowFocused(bool),
    ScaleFactorChanged(f32),
    Resized { width: u32, height: u32 },
    RedrawRequested,
}
