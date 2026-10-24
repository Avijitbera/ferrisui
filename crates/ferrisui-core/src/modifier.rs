use crate::build_context::BuildContext;
use crate::color::Color;
use crate::geometry::EdgeInsets;
use crate::view::View;
use crate::view_node::{NodeKind, ViewNode};

pub trait ViewModifierExt: View + Sized + 'static {
    fn padding(self, insets: EdgeInsets) -> ContainerModifier<Self> {
        ContainerModifier {
            child: self,
            background: Color::TRANSPARENT,
            corner_radius: 0.0,
            padding: insets,
        }
    }

    fn background(self, color: Color) -> ContainerModifier<Self> {
        ContainerModifier {
            child: self,
            background: color,
            corner_radius: 0.0,
            padding: EdgeInsets::ZERO,
        }
    }
}

impl<V: View + Sized + 'static> ViewModifierExt for V {}

pub struct ContainerModifier<V: View> {
    child: V,
    background: Color,
    corner_radius: f32,
    padding: EdgeInsets,
}

impl<V: View> View for ContainerModifier<V> {
    fn build(&self, ctx: &mut BuildContext) -> ViewNode {
        let child_node = self.child.build(ctx);
        ViewNode::new(NodeKind::Container {
            background: self.background,
            corner_radius: self.corner_radius,
            padding: self.padding,
        })
        .with_child(child_node)
    }
}
