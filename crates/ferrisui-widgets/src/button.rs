use ferrisui_core::build_context::BuildContext;
use ferrisui_core::callback::Callback;
use ferrisui_core::color::Color;
use ferrisui_core::geometry::EdgeInsets;
use ferrisui_core::view::View;
use ferrisui_core::view_node::{NodeKind, ViewNode};
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ButtonVariant {
    Filled,
    Outlined,
    Text,
}

#[derive(Debug, Clone)]
pub struct ButtonStyle {
    pub variant: ButtonVariant,
    pub background: Color,
    pub text_color: Color,
    pub border_color: Color,
    pub corner_radius: f32,
    pub padding: EdgeInsets,
}

impl Default for ButtonStyle {
    fn default() -> Self {
        Self {
            variant: ButtonVariant::Filled,
            background: Color::hex(0xEA580C), // Vibrant Orange
            text_color: Color::rgba(1.0, 1.0, 1.0, 1.0),
            border_color: Color::TRANSPARENT,
            corner_radius: 10.0,
            padding: EdgeInsets::symmetric(18.0, 10.0),
        }
    }
}

pub struct Button {
    label: String,
    style: ButtonStyle,
    on_click: Option<Callback<()>>,
}

impl Button {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            style: ButtonStyle::default(),
            on_click: None,
        }
    }
    pub fn style(mut self, style: ButtonStyle) -> Self {
        self.style = style;
        self
    }

    pub fn background(mut self, color: Color) -> Self {
        self.style.background = color;
        self
    }
    pub fn corner_radius(mut self, radius: f32) -> Self {
        self.style.corner_radius = radius;
        self
    }
    pub fn padding(mut self, padding: EdgeInsets) -> Self {
        self.style.padding = padding;
        self
    }
    pub fn on_click<F: Fn() + Send + Sync + 'static>(mut self, handler: F) -> Self {
        self.on_click = Some(Callback::new(move |_| handler()));
        self
    }
}

impl View for Button {
    fn build(&self, _ctx: &mut BuildContext) -> ViewNode {
        // A Button builds into a Container containing a Text child
        let text_child = ViewNode::new(NodeKind::Text {
            text: self.label.clone(),
            font_size: 15.0,
            color: self.style.text_color,
        });
        ViewNode::new(NodeKind::Container {
            background: self.style.background,
            corner_radius: self.style.corner_radius,
            padding: self.style.padding,
        })
        .with_child(text_child)
    }
}
