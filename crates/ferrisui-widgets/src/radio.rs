use ferrisui_core::build_context::BuildContext;
use ferrisui_core::callback::Callback;
use ferrisui_core::color::Color;
use ferrisui_core::geometry::EdgeInsets;
use ferrisui_core::view::View;
use ferrisui_core::view_node::{NodeKind, ViewNode};
pub struct Radio {
    selected: bool,
    label: String,
    accent_color: Color,
    on_select: Option<Callback<()>>,
}

impl Radio {
    pub fn new(selected: bool, label: impl Into<String>) -> Self {
        Self {
            selected,
            label: label.into(),
            accent_color: Color::hex(0x38BDF8),
            on_select: None,
        }
    }
    pub fn accent_color(mut self, color: Color) -> Self {
        self.accent_color = color;
        self
    }
    pub fn on_select<F: Fn() + Send + Sync + 'static>(mut self, handler: F) -> Self {
        self.on_select = Some(Callback::new(move |_| handler()));
        self
    }
}

impl View for Radio {
    fn build(&self, _ctx: &mut BuildContext) -> ViewNode {
        let outer_circle = ViewNode::new(NodeKind::Container {
            background: if self.selected {
                self.accent_color
            } else {
                Color::rgba(1.0, 1.0, 1.0, 0.12)
            },
            corner_radius: 10.0,
            padding: EdgeInsets::all(3.0),
        });
        ViewNode::new(NodeKind::Flex {
            is_vertical: false,
            spacing: 8.0,
        })
        .with_child(outer_circle)
        .with_child(ViewNode::new(NodeKind::Text {
            text: self.label.clone(),
            font_size: 15.0,
            color: Color::WHITE,
        }))
    }
}
