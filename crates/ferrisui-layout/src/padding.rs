use crate::constraints::BoxConstraints;
use crate::layout_node::LayoutResult;
use ferrisui_core::geometry::{EdgeInsets, Rect, Size};

pub fn resolve_padding(
    constraints: BoxConstraints,
    insets: EdgeInsets,
    child_size: Option<Size>,
) -> LayoutResult {
    let horizontal = insets.left + insets.right;
    let vertical = insets.top + insets.bottom;
    match child_size {
        Some(child_sz) => {
            let total_size = constraints.constrain(Size::new(
                child_sz.width + horizontal,
                child_sz.height + vertical,
            ));
            let child_rect = Rect::new(insets.left, insets.top, child_sz.width, child_sz.height);
            LayoutResult::new(total_size, vec![child_rect])
        }
        None => {
            let total_size = constraints.constrain(Size::new(horizontal, vertical));
            LayoutResult::leaf(total_size)
        }
    }
}
