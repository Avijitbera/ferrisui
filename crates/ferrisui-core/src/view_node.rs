use crate::color::Color;
use crate::geometry::{EdgeInsets, Rect};
use crate::id::ViewId;

#[derive(Debug, Clone)]
pub enum NodeKind {
    Empty,
    Text {
        text: String,
        font_size: f32,
        color: Color,
    },
    Container {
        background: Color,
        corner_radius: f32,
        padding: EdgeInsets,
    },
    Flex {
        is_vertical: bool,
        spacing: f32,
    },
}

#[derive(Debug, Clone)]
pub struct ViewNode {
    pub id: ViewId,
    pub kind: NodeKind,
    pub bounds: Rect,
    pub children: Vec<ViewNode>,
}

impl ViewNode {
    pub fn new(kind: NodeKind) -> Self {
        Self {
            id: ViewId::next(),
            kind,
            bounds: Rect::ZERO,
            children: Vec::new(),
        }
    }

    pub fn with_child(mut self, child: ViewNode) -> Self {
        self.children.push(child);
        self
    }
}
