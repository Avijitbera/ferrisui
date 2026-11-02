use ferrisui_core::build_context::BuildContext;
use ferrisui_core::callback::Callback;
use ferrisui_core::color::Color;
use ferrisui_core::geometry::EdgeInsets;
use ferrisui_core::view::View;
use ferrisui_core::view_node::{NodeKind, ViewNode};

pub struct Toggle {
    is_on: bool,
    active_color: Color,
    inactive_color: Color,
    thumb_color: Color,
    on_toggle: Option<Callback<bool>>,
}

impl Toggle {
    pub fn new(is_on: bool) -> Self {
        Self {
            is_on,
            active_color: Color::hex(0x10B981),
            inactive_color: Color::hex(0x334155),
            thumb_color: Color::WHITE,
            on_toggle: None,
        }
    }

    pub fn active_color(mut self, color: Color) -> Self {
        self.active_color = color;
        self
    }
    pub fn inactive_color(mut self, color: Color) -> Self {
        self.inactive_color = color;
        self
    }
    pub fn on_toggle<F: Fn(bool) + Send + Sync + 'static>(mut self, handler: F) -> Self {
        self.on_toggle = Some(Callback::new(handler));
        self
    }
}

impl View for Toggle {
    fn build(&self, _ctx: &mut BuildContext) -> ViewNode {
        let track_bg = if self.is_on {
            self.active_color
        } else {
            self.inactive_color
        };

        let track = ViewNode::new(NodeKind::Container {
            background: track_bg,
            corner_radius: 16.0,
            padding: EdgeInsets::symmetric(4.0, 3.0),
        })
        .with_child(ViewNode::new(NodeKind::Container {
            background: self.thumb_color,
            corner_radius: 12.0,
            padding: EdgeInsets::ZERO,
        }));

        track
    }
}
