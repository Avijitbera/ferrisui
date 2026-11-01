use ferrisui_core::build_context::BuildContext;
use ferrisui_core::view::{AnyView, View};
use ferrisui_core::view_node::{NodeKind, ViewNode};

pub struct VStack {
    spacing: f32,
    children: Vec<AnyView>,
}

impl VStack {
    pub fn new() -> Self {
        Self {
            spacing: 8.0,
            children: Vec::new(),
        }
    }

    pub fn spacing(mut self, spacing: f32) -> Self {
        self.spacing = spacing;
        self
    }

    pub fn child<V: View + 'static>(mut self, child: V) -> Self {
        self.children.push(AnyView::new(child));
        self
    }
}

impl View for VStack {
    fn build(&self, ctx: &mut BuildContext) -> ViewNode {
        let mut node = ViewNode::new(NodeKind::Flex {
            is_vertical: true,
            spacing: self.spacing,
        });
        for child in &self.children {
            node = node.with_child(child.build(ctx))
        }
        node
    }
}

pub struct HStack {
    spacing: f32,
    children: Vec<AnyView>,
}

impl HStack {
    pub fn new() -> Self {
        Self {
            spacing: 8.0,
            children: Vec::new(),
        }
    }

    pub fn spacing(mut self, spacing: f32) -> Self {
        self.spacing = spacing;
        self
    }

    pub fn child<V: View + 'static>(mut self, child: V) -> Self {
        self.children.push(AnyView::new(child));
        self
    }
}

impl View for HStack {
    fn build(&self, ctx: &mut BuildContext) -> ViewNode {
        let mut node = ViewNode::new(NodeKind::Flex {
            is_vertical: false,
            spacing: self.spacing,
        });
        for child in &self.children {
            node = node.with_child(child.build(ctx));
        }
        node
    }
}
