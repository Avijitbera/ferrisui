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
}
