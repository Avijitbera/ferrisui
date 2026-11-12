use crate::callback::Callback;
use crate::color::Color;
use crate::event::KeyEvent;
use crate::geometry::{EdgeInsets, Rect, Size};
use crate::id::ViewId;

#[derive(Debug, Clone)]
pub enum NodeKind {
    Empty,
    Text {
        text: String,
        font_size: f32,
        color: Color,
    },
    Container {
        background: Color,
        corner_radius: f32,
        padding: EdgeInsets,
    },
    Flex {
        is_vertical: bool,
        spacing: f32,
    },
    ZStack,
    Grid {
        columns: usize,
        spacing: f32,
        row_spacing: f32,
    },
}

#[derive(Debug, Clone)]
pub struct ViewNode {
    pub id: ViewId,
    pub kind: NodeKind,
    pub bounds: Rect,
    pub children: Vec<ViewNode>,
    pub on_click: Option<Callback<()>>,
    pub is_focusable: bool,
    pub is_focused: bool,
    pub is_text_left_aligned: bool,
    pub flex_grow: bool,
    /// When set, `compute_preferred_size` returns this directly — ignores children.
    pub fixed_size: Option<Size>,
    pub on_key: Option<Callback<KeyEvent>>,
    pub on_focus_change: Option<Callback<bool>>,
}

impl ViewNode {
    pub fn new(kind: NodeKind) -> Self {
        Self {
            id: ViewId::next(),
            kind,
            bounds: Rect::ZERO,
            children: Vec::new(),
            on_click: None,
            is_focusable: false,
            is_focused: false,
            is_text_left_aligned: false,
            flex_grow: false,
            fixed_size: None,
            on_key: None,
            on_focus_change: None,
        }
    }

    pub fn with_flex_grow(mut self, grow: bool) -> Self {
        self.flex_grow = grow;
        self
    }

    pub fn with_fixed_size(mut self, size: Size) -> Self {
        self.fixed_size = Some(size);
        self
    }

    pub fn with_child(mut self, child: ViewNode) -> Self {
        self.children.push(child);
        self
    }

    pub fn with_on_click(mut self, on_click: Option<Callback<()>>) -> Self {
        self.on_click = on_click;
        self
    }

    pub fn with_focusable(mut self, focusable: bool) -> Self {
        self.is_focusable = focusable;
        self
    }

    pub fn with_focused(mut self, focused: bool) -> Self {
        self.is_focused = focused;
        self
    }

    pub fn with_text_left_aligned(mut self, left_aligned: bool) -> Self {
        self.is_text_left_aligned = left_aligned;
        self
    }

    pub fn with_on_key(mut self, on_key: Option<Callback<KeyEvent>>) -> Self {
        self.on_key = on_key;
        self
    }

    pub fn with_on_focus_change(mut self, on_focus_change: Option<Callback<bool>>) -> Self {
        self.on_focus_change = on_focus_change;
        self
    }
}
