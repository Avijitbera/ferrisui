use ferrisui_core::geometry::{Rect, Size};

use crate::constraints::BoxConstraints;

#[derive(Debug, Clone, PartialEq)]
pub struct LayoutResult {
    pub size: Size,
    pub children_bounds: Vec<Rect>,
}

impl LayoutResult {
    pub fn new(size: Size, children_bounds: Vec<Rect>) -> Self {
        Self {
            size,
            children_bounds,
        }
    }

    pub fn leaf(size: Size) -> Self {
        Self {
            size,
            children_bounds: Vec::new(),
        }
    }
}

pub trait Measurable {
    fn measure(&self, constraints: BoxConstraints) -> Size;
    fn layout(&mut self, constraints: BoxConstraints) -> LayoutResult;
}
