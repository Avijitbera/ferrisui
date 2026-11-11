use ferrisui_core::build_context::BuildContext;
use ferrisui_core::callback::Callback;
use ferrisui_core::color::Color;
use ferrisui_core::geometry::EdgeInsets;
use ferrisui_core::view::View;
use ferrisui_core::view_node::{NodeKind, ViewNode};

pub struct Checkbox {
    checked: bool,
    label: Option<String>,
    accent_color: Color,
    on_change: Option<Callback<bool>>,
}

impl Checkbox {
    pub fn new(checked: bool) -> Self {
        Self {
            checked,
            label: None,
            accent_color: Color::hex(0x38BDF8),
            on_change: None,
        }
    }

    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }
    pub fn accent_color(mut self, color: Color) -> Self {
        self.accent_color = color;
        self
    }
    pub fn on_change<F: Fn(bool) + Send + Sync + 'static>(mut self, handler: F) -> Self {
        self.on_change = Some(Callback::new(handler));
        self
    }
}

impl View for Checkbox {
    fn build(&self, _ctx: &mut BuildContext) -> ViewNode {
        let box_bg = if self.checked {
            self.accent_color
        } else {
            Color::rgba(1.0, 1.0, 1.0, 0.08)
        };

        let mut check_box = ViewNode::new(NodeKind::Container {
            background: box_bg,
            corner_radius: 6.0,
            padding: EdgeInsets::symmetric(8.0, 4.0),
        });

        if self.checked {
            check_box = check_box.with_child(ViewNode::new(NodeKind::Text {
                text: "v".to_string(),
                font_size: 11.0,
                color: Color::WHITE,
            }));
        } else {
            check_box = check_box.with_child(ViewNode::new(NodeKind::Text {
                text: " ".to_string(),
                font_size: 11.0,
                color: Color::rgba(0.0, 0.0, 0.0, 0.0),
            }));
        }

        let mut node = if let Some(ref text) = self.label {
            ViewNode::new(NodeKind::Flex {
                is_vertical: false,
                spacing: 10.0,
            })
            .with_child(check_box)
            .with_child(ViewNode::new(NodeKind::Text {
                text: text.clone(),
                font_size: 15.0,
                color: if self.checked {
                    Color::rgba(1.0, 1.0, 1.0, 0.5)
                } else {
                    Color::WHITE
                },
            }))
        } else {
            check_box
        };

        if let Some(ref cb) = self.on_change {
            let cb = cb.clone();
            let next_val = !self.checked;
            node.on_click = Some(Callback::new(move |_| {
                cb.emit(next_val);
            }));
        }

        node
    }
}
