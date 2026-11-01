use ferrisui_core::build_context::BuildContext;
use ferrisui_core::callback::Callback;
use ferrisui_core::color::Color;
use ferrisui_core::geometry::EdgeInsets;
use ferrisui_core::view::View;
use ferrisui_core::view_node::{NodeKind, ViewNode};

pub struct Slider {
    value: f32,
    min: f32,
    max: f32,
    on_change: Option<Callback<f32>>,
}

impl Slider {
    pub fn new(value: f32) -> Self {
        Self {
            value: value.clamp(0.0, 1.0),
            min: 0.0,
            max: 1.0,
            on_change: None,
        }
    }

    pub fn range(mut self, min: f32, max: f32) -> Self {
        self.min = min;
        self.max = max;
        self
    }

    pub fn on_change<F: Fn(f32) + Send + Sync + 'static>(mut self, handler: F) -> Self {
        self.on_change = Some(Callback::new(handler));
        self
    }
}

impl View for Slider {
    fn build(&self, ctx: &mut BuildContext) -> ViewNode {
        let fill_pct = (self.value - self.min) / (self.max - self.min).max(0.001);

        ViewNode::new(NodeKind::Container {
            background: Color::rgba(1.0, 1.0, 1.0, 0.15),
            corner_radius: 4.0,
            padding: EdgeInsets::ZERO,
        })
        .with_child(ViewNode::new(NodeKind::Container {
            background: Color::hex(0x38BDF8),
            corner_radius: 4.0,
            padding: EdgeInsets::ZERO,
        }))
    }
}
