use crate::alignment::Alignment;
use crate::constraints::BoxConstraints;
use crate::flex::{CrossAxisAlignment, FlexDirection, FlexItem, FlexSolver, MainAxisAlignment};
use crate::layout_node::LayoutResult;
use ferrisui_core::geometry::{Rect, Size};

pub fn compute_vstack(
    constraints: BoxConstraints,
    spacing: f32,
    children_sizes: &[Size],
    main_align: MainAxisAlignment,
    cross_align: CrossAxisAlignment,
) -> LayoutResult {
    let mut solver = FlexSolver::new(FlexDirection::Vertical);
    solver.spacing = spacing;
    solver.main_alignment = main_align;
    solver.cross_alignment = cross_align;

    let items: Vec<FlexItem> = children_sizes
        .iter()
        .map(|&s| FlexItem {
            flex_factor: 0.0,
            preferred_size: s,
        })
        .collect();

    solver.solve(constraints, &items)
}

pub fn compute_hstack(
    constraints: BoxConstraints,
    spacing: f32,
    children_sizes: &[Size],
    main_align: MainAxisAlignment,
    cross_align: CrossAxisAlignment,
) -> LayoutResult {
    let mut solver = FlexSolver::new(FlexDirection::Horizontal);

    solver.spacing = spacing;
    solver.main_alignment = main_align;
    solver.cross_alignment = cross_align;

    let items: Vec<FlexItem> = children_sizes
        .iter()
        .map(|&s| FlexItem {
            flex_factor: 0.0,
            preferred_size: s,
        })
        .collect();
    solver.solve(constraints, &items)
}

pub fn compute_zstack(
    constraints: BoxConstraints,
    children_sizes: &[Size],
    alignment: Alignment,
) -> LayoutResult {
    let mut max_width: f32 = 0.0;
    let mut max_height: f32 = 0.0;

    for &size in children_sizes {
        max_width = max_width.max(size.width);
        max_height = max_height.max(size.height);
    }

    let final_size = constraints.constrain(Size::new(max_width, max_height));
    let mut children_bounds = Vec::with_capacity(children_sizes.len());

    for &child_size in children_sizes {
        let pos = alignment.align(child_size, final_size);
        children_bounds.push(Rect::from_origin_size(pos, child_size));
    }

    LayoutResult::new(final_size, children_bounds)
}
