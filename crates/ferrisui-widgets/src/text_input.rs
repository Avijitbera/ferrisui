use ferrisui_core::{
    build_context::BuildContext,
    callback::Callback,
    color::Color,
    event::{ElementState, Key, KeyEvent},
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
    is_focused: Option<Signal<bool>>,
    cursor_pos: Option<Signal<usize>>,
    on_change: Option<Callback<String>>,
    on_submit: Option<Callback<String>>,
}

impl TextInput {
    pub fn new(text: Signal<String>) -> Self {
        Self {
            text,
            placeholder: String::new(),
            style: TextInputStyle::default(),
            is_focused: None,
            cursor_pos: None,
            on_change: None,
            on_submit: None,
        }
    }

    /// Explicitly binds or sets an external focus signal.
    pub fn focused(mut self, focused: Signal<bool>) -> Self {
        self.is_focused = Some(focused);
        self
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
        let mut pos = self
            .cursor_pos
            .as_ref()
            .map(|c| c.get())
            .unwrap_or_else(|| self.text.get().len());
        self.text.update(|s| {
            if pos > s.len() {
                pos = s.len();
            }
            s.insert(pos, ch);
        });
        if let Some(ref c) = self.cursor_pos {
            c.set(pos + 1);
        }
        if let Some(ref cb) = self.on_change {
            cb.emit(self.text.get());
        }
    }

    /// Deletes the character immediately before the cursor (Backspace).
    pub fn delete_backward(&self) {
        let pos = self
            .cursor_pos
            .as_ref()
            .map(|c| c.get())
            .unwrap_or_else(|| self.text.get().len());
        if pos > 0 {
            self.text.update(|s| {
                if pos <= s.len() {
                    s.remove(pos - 1);
                }
            });
            if let Some(ref c) = self.cursor_pos {
                c.set(pos - 1);
            }
            if let Some(ref cb) = self.on_change {
                cb.emit(self.text.get());
            }
        }
    }
}

impl View for TextInput {
    fn build(&self, ctx: &mut BuildContext) -> ViewNode {
        let is_focused_sig = match &self.is_focused {
            Some(sig) => sig.clone(),
            None => ctx.state(false),
        };
        let cursor_pos_sig = match &self.cursor_pos {
            Some(sig) => sig.clone(),
            None => ctx.state(self.text.get().len()),
        };

        let current_text = self.text.get();
        let focused = is_focused_sig.get();

        let (display_text, display_color) = if current_text.is_empty() {
            if focused {
                (format!("| {}", self.placeholder), self.style.placeholder_color)
            } else {
                (self.placeholder.clone(), self.style.placeholder_color)
            }
        } else if focused {
            let pos = cursor_pos_sig.get().min(current_text.len());
            let mut with_cursor = current_text.clone();
            with_cursor.insert(pos, '|');
            (with_cursor, self.style.text_color)
        } else {
            (current_text, self.style.text_color)
        };

        let text_node = ViewNode::new(NodeKind::Text {
            text: display_text,
            font_size: self.style.font_size,
            color: display_color,
        })
        .with_text_left_aligned(true);

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
        .with_child(text_node)
        .with_focusable(true)
        .with_focused(focused)
        .with_flex_grow(true);

        let is_focused_for_click = is_focused_sig.clone();
        container_node.on_click = Some(Callback::new(move |_| {
            is_focused_for_click.set(true);
        }));

        let is_focused_for_change = is_focused_sig.clone();
        container_node.on_focus_change = Some(Callback::new(move |f: bool| {
            is_focused_for_change.set(f);
        }));

        let text_sig = self.text.clone();
        let cursor_pos = cursor_pos_sig.clone();
        let on_change = self.on_change.clone();
        let on_submit = self.on_submit.clone();
        let is_focused_for_key = is_focused_sig.clone();

        container_node.on_key = Some(Callback::new(move |event: KeyEvent| {
            if !is_focused_for_key.get() {
                return;
            }
            match event {
                KeyEvent::Key { key, state, .. } => {
                    if state != ElementState::Pressed {
                        return;
                    }
                    match key {
                        Key::Enter => {
                            cursor_pos.set(0);
                            if let Some(ref cb) = on_submit {
                                cb.emit(text_sig.get());
                            }
                        }
                        Key::Backspace => {
                            let mut pos = cursor_pos.get();
                            text_sig.update(|s| {
                                if pos > s.len() {
                                    pos = s.len();
                                }
                                if pos > 0 && pos <= s.len() {
                                    s.remove(pos - 1);
                                    pos -= 1;
                                }
                            });
                            cursor_pos.set(pos);
                            if let Some(ref cb) = on_change {
                                cb.emit(text_sig.get());
                            }
                        }
                        Key::ArrowLeft => {
                            let pos = cursor_pos.get();
                            if pos > 0 {
                                cursor_pos.set(pos - 1);
                            }
                        }
                        Key::ArrowRight => {
                            let pos = cursor_pos.get();
                            let len = text_sig.get().len();
                            if pos < len {
                                cursor_pos.set(pos + 1);
                            }
                        }
                        Key::Space => {
                            let mut pos = cursor_pos.get();
                            text_sig.update(|s| {
                                if pos > s.len() {
                                    pos = s.len();
                                }
                                s.insert(pos, ' ');
                            });
                            cursor_pos.set(pos + 1);
                            if let Some(ref cb) = on_change {
                                cb.emit(text_sig.get());
                            }
                        }
                        Key::Character(s) => {
                            let mut pos = cursor_pos.get();
                            text_sig.update(|buf| {
                                for ch in s.chars() {
                                    if !ch.is_control() {
                                        if pos > buf.len() {
                                            pos = buf.len();
                                        }
                                        buf.insert(pos, ch);
                                        pos += 1;
                                    }
                                }
                            });
                            cursor_pos.set(pos);
                            if let Some(ref cb) = on_change {
                                cb.emit(text_sig.get());
                            }
                        }
                        _ => {}
                    }
                }
                KeyEvent::Character(ch) => {
                    if !ch.is_control() {
                        let mut pos = cursor_pos.get();
                        text_sig.update(|buf| {
                            if pos > buf.len() {
                                pos = buf.len();
                            }
                            buf.insert(pos, ch);
                        });
                        cursor_pos.set(pos + 1);
                        if let Some(ref cb) = on_change {
                            cb.emit(text_sig.get());
                        }
                    }
                }
            }
        }));

        container_node
    }
}
