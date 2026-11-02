// divider.rs
use ferrisui_core::build_context::BuildContext;
use ferrisui_core::color::Color;
use ferrisui_core::geometry::EdgeInsets;
use ferrisui_core::view::View;
use ferrisui_core::view_node::{NodeKind, ViewNode};

pub struct Divider {
    color: Color,
}

impl Divider {
    pub fn new() -> Self {
        Self {
            color: Color::rgba(1.0, 1.0, 1.0, 0.10),
        }
    }

    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }
}

impl View for Divider {
    fn build(&self, _ctx: &mut BuildContext) -> ViewNode {
        ViewNode::new(NodeKind::Container {
            background: self.color,
            corner_radius: 0.5,
            padding: EdgeInsets::ZERO,
        })
    }
}
