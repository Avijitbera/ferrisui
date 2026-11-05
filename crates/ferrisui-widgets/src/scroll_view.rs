use ferrisui_core::{
    build_context::BuildContext,
    color::Color,
    geometry::EdgeInsets,
    state::Signal,
    view::View,
    view_node::{NodeKind, ViewNode},
};

pub struct ScrollView<V: View> {
    child: V,
    scroll_offset: Signal<f32>,
    viewport_height: Option<f32>,
    show_scrollbar: bool,
    scrollbar_color: Color,
    scrollbar_width: f32,
    track_color: Color,
}

impl<V: View> ScrollView<V> {
    pub fn new(child: V) -> Self {
        Self {
            child,
            scroll_offset: Signal::new(0.0),
            viewport_height: None,
            show_scrollbar: true,
            scrollbar_color: Color::rgba(1.0, 1.0, 1.0, 0.15),
            scrollbar_width: 6.0,
            track_color: Color::rgba(0.0, 0.0, 0.0, 0.15),
        }
    }

    pub fn scroll_offset(mut self, offset: Signal<f32>) -> Self {
        self.scroll_offset = offset;
        self
    }

    pub fn height(mut self, height: f32) -> Self {
        self.viewport_height = Some(height);
        self
    }

    pub fn show_scrollbar(mut self, show: bool) -> Self {
        self.show_scrollbar = show;
        self
    }

    pub fn scrollbar_color(mut self, color: Color) -> Self {
        self.scrollbar_color = color;
        self
    }

    pub fn scroll_by(&self, delta_y: f32, max_scroll: f32) {
        let current = self.scroll_offset.get();
        let new_offset = (current + delta_y).clamp(0.0, max_scroll.max(0.0));
        self.scroll_offset.set(new_offset);
    }

    pub fn scroll_to_top(&self) {
        self.scroll_offset.set(0.0);
    }
}

impl<V: View> View for ScrollView<V> {
    fn build(&self, ctx: &mut BuildContext) -> ViewNode {
        let current_offset = self.scroll_offset.get();

        let mut viewport_node = ViewNode::new(NodeKind::Container {
            background: Color::TRANSPARENT,
            corner_radius: 0.0,
            padding: EdgeInsets::ZERO,
        });

        let mut row_node = ViewNode::new(NodeKind::Flex {
            is_vertical: false,
            spacing: 4.0,
        });

        let mut content_container = ViewNode::new(NodeKind::Container {
            background: Color::TRANSPARENT,
            corner_radius: 0.0,
            padding: EdgeInsets::only(0.0, current_offset.max(0.0), 0.0, 0.0),
        });

        content_container = content_container.with_child(self.child.build(ctx));
        row_node = row_node.with_child(content_container);

        if self.show_scrollbar {
            let mut track_node = ViewNode::new(NodeKind::Container {
                background: self.track_color,
                corner_radius: self.scrollbar_width / 2.0,
                padding: EdgeInsets::symmetric(self.scrollbar_width / 2.0, 2.0),
            });

            let thumb_node = ViewNode::new(NodeKind::Container {
                background: self.scrollbar_color,
                corner_radius: self.scrollbar_width / 2.0,
                padding: EdgeInsets::symmetric(self.scrollbar_width / 2.0, 10.0),
            });

            track_node = track_node.with_child(thumb_node);
            row_node = row_node.with_child(track_node);
        }

        viewport_node = viewport_node.with_child(row_node);
        viewport_node
    }
}
