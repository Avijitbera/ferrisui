use crate::build_context::BuildContext;
use crate::view_node::ViewNode;

pub trait View {
    fn build(&self, ctx: &mut BuildContext) -> ViewNode;
}

pub struct AnyView {
    inner: Box<dyn View>,
}

impl AnyView {
    pub fn new<V: View + 'static>(view: V) -> Self {
        Self {
            inner: Box::new(view),
        }
    }
}

impl View for AnyView {
    fn build(&self, ctx: &mut BuildContext) -> ViewNode {
        self.inner.build(ctx)
    }
}

impl View for () {
    fn build(&self, _ctx: &mut BuildContext) -> ViewNode {
        ViewNode::new(crate::view_node::NodeKind::Empty)
    }
}
