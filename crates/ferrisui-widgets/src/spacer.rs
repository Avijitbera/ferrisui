use ferrisui_core::{
    build_context::BuildContext,
    color::Color,
    geometry::EdgeInsets,
    view::View,
    view_node::{NodeKind, ViewNode},
};

#[derive(Debug, Clone, Copy)]
pub struct Spacer {
    fixed_size: Option<f32>,
    #[allow(dead_code)]
    flex: f32,
}

impl Spacer {
    pub fn new() -> Self {
        Self {
            fixed_size: None,
            flex: 1.0,
        }
    }

    pub fn fixed(size: f32) -> Self {
        Self {
            fixed_size: Some(size),
            flex: 0.0,
        }
    }

    pub fn flex(weight: f32) -> Self {
        Self {
            fixed_size: None,
            flex: weight.max(0.1),
        }
    }
}

impl Default for Spacer {
    fn default() -> Self {
        Self::new()
    }
}

impl View for Spacer {
    fn build(&self, _ctx: &mut BuildContext) -> ViewNode {
        let pad = match self.fixed_size {
            Some(sz) => EdgeInsets::all(sz / 2.0),
            None => EdgeInsets::ZERO,
        };

        ViewNode::new(NodeKind::Container {
            background: Color::TRANSPARENT,
            corner_radius: 0.0,
            padding: pad,
        })
    }
}
