use ferrisui_core::build_context::BuildContext;
use ferrisui_core::color::Color;
use ferrisui_core::geometry::EdgeInsets;
use ferrisui_core::view::View;
use ferrisui_core::view_node::{NodeKind, ViewNode};
pub struct Container<V: View> {
    child: Option<V>,
    background: Color,
    corner_radius: f32,
    padding: EdgeInsets,
}

impl Container<()> {
    pub fn new() -> Self {
        Container {
            child: None,
            background: Color::TRANSPARENT,
            corner_radius: 0.0,
            padding: EdgeInsets::ZERO,
        }
    }
}

impl<V: View> Container<V> {
    pub fn child<C: View>(self, child: C) -> Container<C> {
        Container {
            child: Some(child),
            background: self.background,
            corner_radius: self.corner_radius,
            padding: self.padding,
        }
    }

    pub fn background(mut self, color: Color) -> Self {
        self.background = color;
        self
    }

    pub fn corner_radius(mut self, radius: f32) -> Self {
        self.corner_radius = radius;
        self
    }

    pub fn padding(mut self, padding: EdgeInsets) -> Self {
        self.padding = padding;
        self
    }
}

impl<V: View> View for Container<V> {
    fn build(&self, ctx: &mut BuildContext) -> ViewNode {
        let mut node = ViewNode::new(NodeKind::Container {
            background: self.background,
            corner_radius: self.corner_radius,
            padding: self.padding,
        });
        if let Some(ref child) = self.child {
            node = node.with_child(child.build(ctx));
        }
        node
    }
}
