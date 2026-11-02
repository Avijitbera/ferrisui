use ferrisui_core::{
    build_context::BuildContext,
    callback::Callback,
    color::Color,
    geometry::EdgeInsets,
    state::Signal,
    view::View,
    view_node::{NodeKind, ViewNode},
};

#[derive(Debug, Clone)]
pub struct TextInputStyle {
    pub background: Color,
    pub text_color: Color,
    pub placeholder_color: Color,
    pub border_color: Color,
    pub focused_border_color: Color,
    pub cursor_color: Color,
    pub corner_radius: f32,
    pub padding: EdgeInsets,
    pub font_size: f32,
}

impl Default for TextInputStyle {
    fn default() -> Self {
        Self {
            background: Color::hex(0x1E293B),
            text_color: Color::hex(0xF8FAFC),
            placeholder_color: Color::hex(0x64748B),
            border_color: Color::rgba(1.0, 1.0, 1.0, 0.15),
            focused_border_color: Color::hex(0x38BDF8),
            cursor_color: Color::hex(0x38BDF8),
            corner_radius: 8.0,
            padding: EdgeInsets::symmetric(14.0, 12.0),
            font_size: 15.0,
        }
    }
}

pub struct TextInput {
    text: Signal<String>,
    placeholder: String,
    style: TextInputStyle,
    is_focused: Signal<bool>,
    cursor_pos: Signal<usize>,
    on_change: Option<Callback<String>>,
    on_submit: Option<Callback<String>>,
}

impl TextInput {
    pub fn new(text: Signal<String>) -> Self {
        let len = text.get().len();

        Self {
            text,
            placeholder: String::new(),
            style: TextInputStyle::default(),
            is_focused: Signal::new(false),
            cursor_pos: Signal::new(len),
            on_change: None,
            on_submit: None,
        }
    }

    /// Sets placeholder text displayed when the input buffer is empty.
    pub fn placeholder(mut self, placeholder: impl Into<String>) -> Self {
        self.placeholder = placeholder.into();
        self
    }
    /// Overrides default visual style tokens.
    pub fn style(mut self, style: TextInputStyle) -> Self {
        self.style = style;
        self
    }

    /// Sets custom background color.
    pub fn background(mut self, color: Color) -> Self {
        self.style.background = color;
        self
    }
    /// Sets custom corner radius.
    pub fn corner_radius(mut self, radius: f32) -> Self {
        self.style.corner_radius = radius;
        self
    }

    /// Callback fired whenever the text buffer changes.
    pub fn on_change<F: Fn(String) + Send + Sync + 'static>(mut self, handler: F) -> Self {
        self.on_change = Some(Callback::new(handler));
        self
    }
    /// Callback fired when the user presses Enter/Return.
    pub fn on_submit<F: Fn(String) + Send + Sync + 'static>(mut self, handler: F) -> Self {
        self.on_submit = Some(Callback::new(handler));
        self
    }

    /// Inserts a character at the current cursor position.
    pub fn insert_char(&self, ch: char) {
        let mut pos = self.cursor_pos.get();
        self.text.update(|s| {
            if pos > s.len() {
                pos = s.len();
            }
            s.insert(pos, ch);
        });
        self.cursor_pos.set(pos + 1);
        if let Some(ref cb) = self.on_change {
            cb.emit(self.text.get());
        }
    }

    /// Deletes the character immediately before the cursor (Backspace).
    pub fn delete_backward(&self) {
        let pos = self.cursor_pos.get();
        if pos > 0 {
            self.text.update(|s| {
                if pos <= s.len() {
                    s.remove(pos - 1);
                }
            });
            self.cursor_pos.set(pos - 1);
            if let Some(ref cb) = self.on_change {
                cb.emit(self.text.get());
            }
        }
    }
}

impl View for TextInput {
    fn build(&self, _ctx: &mut BuildContext) -> ViewNode {
        let current_text = self.text.get();
        let focused = self.is_focused.get();

        let (display_text, display_color) = if current_text.is_empty() {
            (self.placeholder.clone(), self.style.placeholder_color)
        } else {
            (current_text, self.style.text_color)
        };

        let text_node = ViewNode::new(NodeKind::Text {
            text: display_text,
            font_size: self.style.font_size,
            color: display_color,
        });
        let is_focused_signal = self.is_focused.clone();
        let bg_color = if focused {
            Color::hex(0x0F172A)
        } else {
            self.style.background
        };
        let mut container_node = ViewNode::new(NodeKind::Container {
            background: bg_color,
            corner_radius: self.style.corner_radius,
            padding: self.style.padding,
        })
        .with_child(text_node);

        container_node.on_click = Some(Callback::new(move |_| {
            is_focused_signal.set(true);
        }));
        container_node
    }
}
