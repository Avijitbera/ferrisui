use ferrisui_core::{
    build_context::BuildContext,
    view::{AnyView, View},
    view_node::{NodeKind, ViewNode},
};

pub struct Grid {
    columns: usize,
    spacing: f32,
    row_spacing: f32,
    children: Vec<AnyView>,
}

impl Grid {
    pub fn new(columns: usize) -> Self {
        Self {
            columns: columns.max(1),
            spacing: 8.0,
            row_spacing: 8.0,
            children: Vec::new(),
        }
    }

    pub fn gap(mut self, gap: f32) -> Self {
        self.spacing = gap;
        self.row_spacing = gap;
        self
    }

    pub fn spacing(mut self, row_spacing: f32) -> Self {
        self.row_spacing = row_spacing;
        self
    }

    pub fn row_spacing(mut self, row_spacing: f32) -> Self {
        self.row_spacing = row_spacing;
        self
    }

    pub fn child<V: View + 'static>(mut self, child: V) -> Self {
        self.children.push(AnyView::new(child));
        self
    }

    pub fn children(mut self, children: impl IntoIterator<Item = AnyView>) -> Self {
        self.children.extend(children);
        self
    }
}

impl View for Grid {
    fn build(&self, ctx: &mut BuildContext) -> ViewNode {
        let mut node = ViewNode::new(NodeKind::Grid {
            columns: self.columns,
            spacing: self.spacing,
            row_spacing: self.row_spacing,
        })
        .with_flex_grow(true);

        for child in &self.children {
            node = node.with_child(child.build(ctx));
        }
        node
    }
}
