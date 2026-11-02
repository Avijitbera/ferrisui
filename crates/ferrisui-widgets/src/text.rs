use ferrisui_core::build_context::BuildContext;
use ferrisui_core::color::Color;
use ferrisui_core::view::View;
use ferrisui_core::view_node::{NodeKind, ViewNode};
pub struct Text {
    content: String,
    font_size: f32,
    color: Color,
}

impl Text {
    pub fn new(content: impl Into<String>) -> Self {
        Self {
            content: content.into(),
            font_size: 16.0,
            color: Color::rgba(1.0, 1.0, 1.0, 0.95),
        }
    }

    pub fn font_size(mut self, size: f32) -> Self {
        self.font_size = size;
        self
    }

    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }
}

impl View for Text {
    fn build(&self, _ctx: &mut BuildContext) -> ViewNode {
        ViewNode::new(NodeKind::Text {
            text: self.content.clone(),
            font_size: self.font_size,
            color: self.color,
        })
    }
}
