use crate::constraints::BoxConstraints;
use crate::layout_node::LayoutResult;
use ferrisui_core::geometry::{Rect, Size};

pub struct GridLayout {
    pub columns: usize,
    pub spacing: f32,
    pub row_spacing: f32,
}
impl GridLayout {
    pub fn new(columns: usize, spacing: f32) -> Self {
        Self {
            columns: columns.max(1),
            spacing,
            row_spacing: spacing,
        }
    }

    pub fn solve(&self, constraints: BoxConstraints, children_sizes: &[Size]) -> LayoutResult {
        let n = children_sizes.len();
        if n == 0 {
            return LayoutResult::leaf(constraints.constrain(Size::ZERO));
        }
        let cols = self.columns;
        let total_col_spacing = self.spacing * (cols - 1) as f32;
        let col_width = ((constraints.max_width - total_col_spacing) / cols as f32).max(0.0);
        let rows = (n + cols - 1) / cols;
        let mut row_heights = vec![0.0f32; rows];
        for (idx, size) in children_sizes.iter().enumerate() {
            let r = idx / cols;
            row_heights[r] = row_heights[r].max(size.height);
        }
        let mut current_y = 0.0f32;
        let mut children_bounds = Vec::with_capacity(n);
        for (r, &row_h) in row_heights.iter().enumerate() {
            for c in 0..cols {
                let idx = r * cols + c;
                if idx >= n {
                    break;
                }
                let x = c as f32 * (col_width + self.spacing);
                children_bounds.push(Rect::new(x, current_y, col_width, row_h));
            }
            current_y += row_h + self.row_spacing;
        }
        let total_height = (current_y - self.row_spacing).max(0.0);
        let final_size = constraints.constrain(Size::new(constraints.max_width, total_height));
        LayoutResult::new(final_size, children_bounds)
    }
}
