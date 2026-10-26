use crate::constraints::BoxConstraints;
use crate::layout_node::LayoutResult;
use ferrisui_core::geometry::{Point, Rect, Size};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FlexDirection {
    Horizontal,
    Vertical,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MainAxisAlignment {
    Start,
    Center,
    End,
    SpaceBetween,
    SpaceAround,
    SpaceEvenly,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CrossAxisAlignment {
    Start,
    Center,
    End,
    Stretch,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FlexItem {
    pub flex_factor: f32,
    pub preferred_size: Size,
}

pub struct FlexSolver {
    pub direction: FlexDirection,
    pub spacing: f32,
    pub main_alignment: MainAxisAlignment,
    pub cross_alignment: CrossAxisAlignment,
}

impl FlexSolver {
    pub fn new(direction: FlexDirection) -> Self {
        Self {
            direction,
            spacing: 0.0,
            main_alignment: MainAxisAlignment::Start,
            cross_alignment: CrossAxisAlignment::Start,
        }
    }

    pub fn solve(&self, constraints: BoxConstraints, items: &[FlexItem]) -> LayoutResult {
        let is_horiz = self.direction == FlexDirection::Horizontal;
        let n = items.len();
        if n == 0 {
            return LayoutResult::leaf(constraints.constrain(Size::ZERO));
        }

        let total_spacing = self.spacing * (n - 1) as f32;
        let max_main = if is_horiz {
            constraints.max_width
        } else {
            constraints.max_height
        };
        let min_main = if is_horiz {
            constraints.min_width
        } else {
            constraints.min_height
        };

        let mut fixed_main_sum = 0.0f32;
        let mut total_flex = 0.0f32;
        let mut max_cross: f32 = 0.0;

        for item in items {
            if item.flex_factor > 0.0 {
                total_flex += item.flex_factor;
            } else {
                let main_dim = if is_horiz {
                    item.preferred_size.width
                } else {
                    item.preferred_size.height
                };
                let corss_dim = if is_horiz {
                    item.preferred_size.height
                } else {
                    item.preferred_size.width
                };
                fixed_main_sum += main_dim;
                max_cross = max_cross.max(corss_dim);
            }
        }

        let available_for_flex = (max_main - fixed_main_sum - total_spacing).max(0.0);
        let mut item_sizes: Vec<Size> = Vec::with_capacity(n);

        for item in items {
            if item.flex_factor > 0.0 {
                let main_allocated = (item.flex_factor / total_flex) * available_for_flex;
                let size = if is_horiz {
                    Size::new(main_allocated, item.preferred_size.height)
                } else {
                    Size::new(item.preferred_size.width, main_allocated)
                };
                item_sizes.push(size);
                let cross_dim = if is_horiz { size.height } else { size.width };
                max_cross = max_cross.max(cross_dim);
            } else {
                item_sizes.push(item.preferred_size);
            }
        }

        let total_main_used: f32 = item_sizes
            .iter()
            .map(|s| if is_horiz { s.width } else { s.height })
            .sum::<f32>()
            + total_spacing;
        let final_main = total_main_used.clamp(min_main, max_main);
        let final_cross = if is_horiz {
            constraints.constrain_height(max_cross)
        } else {
            constraints.constrain_width(max_cross)
        };

        let free_space = (final_main - total_main_used).max(0.0);

        let (mut current_main, space_between) = match self.main_alignment {
            MainAxisAlignment::Start => (0.0, self.spacing),
            MainAxisAlignment::End => (free_space, self.spacing),
            MainAxisAlignment::Center => (free_space / 2.0, self.spacing),
            MainAxisAlignment::SpaceBetween => {
                let step = if n > 1 {
                    free_space / (n - 1) as f32
                } else {
                    0.0
                };
                (0.0, self.spacing + step)
            }
            MainAxisAlignment::SpaceAround => {
                let step = free_space / n as f32;
                (step / 2.0, self.spacing + step)
            }
            MainAxisAlignment::SpaceEvenly => {
                let step = free_space / (n + 1) as f32;
                (step, self.spacing + step)
            }
        };

        let mut children_bounds = Vec::with_capacity(n);

        for size in &item_sizes {
            let (child_main, child_cross) = if is_horiz {
                (size.width, size.height)
            } else {
                (size.height, size.width)
            };
            let current_cross = match self.cross_alignment {
                CrossAxisAlignment::Start => 0.0,
                CrossAxisAlignment::Center => (final_cross - child_cross).max(0.0) / 2.0,
                CrossAxisAlignment::End => (final_cross - child_cross).max(0.0),
                CrossAxisAlignment::Stretch => 0.0,
            };

            let (origin_x, origin_y, width, height) = if is_horiz {
                (current_main, current_cross, child_main, child_cross)
            } else {
                (current_cross, current_main, child_cross, child_main)
            };
            children_bounds.push(Rect::new(origin_x, origin_y, width, height));
            current_main += child_main + space_between;
        }
        let container_size = if is_horiz {
            Size::new(final_main, final_cross)
        } else {
            Size::new(final_cross, final_main)
        };

        LayoutResult::new(container_size, children_bounds)
    }
}
