use ferrisui_core::{
    build_context::BuildContext,
    callback::Callback,
    color::Color,
    geometry::EdgeInsets,
    state::Signal,
    view::View,
    view_node::{NodeKind, ViewNode},
};

#[derive(Debug, Clone, PartialEq)]
pub struct DropdownItem<T: Clone + PartialEq> {
    pub value: T,
    pub label: String,
}

impl<T: Clone + PartialEq> DropdownItem<T> {
    pub fn new(value: T, label: impl Into<String>) -> Self {
        Self {
            value,
            label: label.into(),
        }
    }
}

pub struct Dropdown<T: Clone + PartialEq + Send + Sync + 'static> {
    items: Vec<DropdownItem<T>>,
    selected: Signal<Option<T>>,
    is_open: Signal<bool>,
    placeholder: String,
    on_change: Option<Callback<T>>,
    background: Color,
    menu_background: Color,
    accent_color: Color,
    text_color: Color,
    placeholder_color: Color,
    corner_radius: f32,
    padding: EdgeInsets,
}

impl<T: Clone + PartialEq + Send + Sync + 'static> Dropdown<T> {
    pub fn new(selected: Signal<Option<T>>) -> Self {
        Self {
            items: Vec::new(),
            selected,
            is_open: Signal::new(false),
            placeholder: String::from("Select an option..."),
            on_change: None,
            background: Color::hex(0x1E293B),
            menu_background: Color::hex(0x0F172A),
            accent_color: Color::hex(0x38BDF8),
            text_color: Color::WHITE,
            placeholder_color: Color::hex(0x64748B),
            corner_radius: 8.0,
            padding: EdgeInsets::symmetric(14.0, 10.0),
        }
    }

    pub fn item(mut self, value: T, label: impl Into<String>) -> Self {
        self.items.push(DropdownItem::new(value, label));
        self
    }

    pub fn items(mut self, items: impl IntoIterator<Item = DropdownItem<T>>) -> Self {
        self.items.extend(items);
        self
    }

    pub fn accent_color(mut self, color: Color) -> Self {
        self.accent_color = color;
        self
    }

    pub fn background(mut self, color: Color) -> Self {
        self.background = color;
        self
    }

    pub fn corner_radius(mut self, radius: f32) -> Self {
        self.corner_radius = radius;
        self
    }
}

impl<T: Clone + PartialEq + Send + Sync + 'static> View for Dropdown<T> {
    fn build(&self, _ctx: &mut BuildContext) -> ViewNode {
        let current_selected = self.selected.get();
        let open = self.is_open.get();

        let selected_label = current_selected.as_ref().and_then(|val| {
            self.items
                .iter()
                .find(|it| &it.value == val)
                .map(|it| it.label.clone())
        });

        let mut column = ViewNode::new(NodeKind::Flex {
            is_vertical: true,
            spacing: 4.0,
        });

        let mut trigger_node = ViewNode::new(NodeKind::Container {
            background: self.background,
            corner_radius: self.corner_radius,
            padding: self.padding,
        });

        let toggle_sig = self.is_open.clone();
        trigger_node.on_click = Some(Callback::new(move |_| {
            toggle_sig.set(!toggle_sig.get());
        }));

        let mut trigger_row = ViewNode::new(NodeKind::Flex {
            is_vertical: false,
            spacing: 12.0,
        });

        let (label_text, label_color) = match selected_label {
            Some(lbl) => (lbl, self.text_color),
            None => (self.placeholder.clone(), self.placeholder_color),
        };

        trigger_row = trigger_row.with_child(ViewNode::new(NodeKind::Text {
            text: label_text,
            font_size: 14.0,
            color: label_color,
        }));

        let arrow_char = if open { "▲" } else { "▼" };

        trigger_row = trigger_row.with_child(ViewNode::new(NodeKind::Text {
            text: arrow_char.into(),
            font_size: 11.0,
            color: self.placeholder_color,
        }));

        trigger_node = trigger_node.with_child(trigger_row);
        column = column.with_child(trigger_node);

        if open {
            let mut menu_card = ViewNode::new(NodeKind::Container {
                background: self.menu_background,
                corner_radius: self.corner_radius,
                padding: EdgeInsets::symmetric(4.0, 4.0),
            });

            let mut menu_vbox = ViewNode::new(NodeKind::Flex {
                is_vertical: true,
                spacing: 2.0,
            });

            for item in &self.items {
                let is_item_selected = current_selected.as_ref() == Some(&item.value);

                let item_bg = if is_item_selected {
                    Color::rgba(0.22, 0.74, 0.97, 0.15)
                } else {
                    Color::TRANSPARENT
                };

                let item_text_color = if is_item_selected {
                    self.accent_color
                } else {
                    self.text_color
                };

                let mut item_node = ViewNode::new(NodeKind::Container {
                    background: item_bg,
                    corner_radius: self.corner_radius - 2.0,
                    padding: EdgeInsets::symmetric(10.0, 8.0),
                });

                let sel_sig = self.selected.clone();
                let open_sig = self.is_open.clone();
                let cb = self.on_change.clone();
                let val_to_set = item.value.clone();

                item_node.on_click = Some(Callback::new(move |_| {
                    sel_sig.set(Some(val_to_set.clone()));
                    open_sig.set(false);
                    if let Some(ref callback) = cb {
                        callback.emit(val_to_set.clone());
                    }
                }));

                let item_text = if is_item_selected {
                    format!("✓ {}", item.label)
                } else {
                    format!("  {}", item.label)
                };
                item_node = item_node.with_child(ViewNode::new(NodeKind::Text {
                    text: item_text,
                    font_size: 13.0,
                    color: item_text_color,
                }));

                menu_vbox = menu_vbox.with_child(item_node);
            }
            menu_card = menu_card.with_child(menu_vbox);
            column = column.with_child(menu_card);
        }

        column
    }
}
