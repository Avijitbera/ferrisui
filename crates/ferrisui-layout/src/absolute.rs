use ferrisui_core::geometry::{Rect, Size};

#[derive(Debug, Clone, Copy, Default)]
pub struct PositionedOffset {
    pub top: Option<f32>,
    pub right: Option<f32>,
    pub bottom: Option<f32>,
    pub left: Option<f32>,
}

pub fn resolve_absolute(parent_size: Size, child_size: Size, offset: PositionedOffset) -> Rect {
    let x = match (offset.left, offset.right) {
        (Some(l), _) => l,
        (None, Some(r)) => parent_size.width - child_size.width - r,
        (None, None) => 0.0,
    };

    let y = match (offset.top, offset.bottom) {
        (Some(t), _) => t,
        (None, Some(b)) => parent_size.height - child_size.height - b,
        (None, None) => 0.0,
    };

    Rect::new(x, y, child_size.width, child_size.height)
}
