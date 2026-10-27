use winit::window::CursorIcon as WinitCursor;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CursorIcon {
    #[default]
    Default,
    Pointer,
    Text,
    Crosshair,
    Move,
    NotAllowed,
    Grab,
    Grabbing,
    ColResize,
    RowResize,
    Wait,
}

impl CursorIcon {
    pub fn to_winit(self) -> WinitCursor {
        match self {
            Self::Default => WinitCursor::Default,
            Self::Pointer => WinitCursor::Pointer,
            Self::Text => WinitCursor::Text,
            Self::Crosshair => WinitCursor::Crosshair,
            Self::Move => WinitCursor::Move,
            Self::NotAllowed => WinitCursor::NotAllowed,
            Self::Grab => WinitCursor::Grab,
            Self::Grabbing => WinitCursor::Grabbing,
            Self::ColResize => WinitCursor::ColResize,
            Self::RowResize => WinitCursor::RowResize,
            Self::Wait => WinitCursor::Wait,
        }
    }
}
