use ferrisui_core::geometry::{Point, Size};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Alignment {
    pub x: f32,
    pub y: f32,
}

impl Default for Alignment {
    fn default() -> Self {
        Self::CENTER
    }
}

impl Alignment {
    pub const TOP_LEADING: Self = Self::new(-1.0, -1.0);
    pub const TOP_CENTER: Self = Self::new(0.0, -1.0);
    pub const TOP_TRAILING: Self = Self::new(1.0, -1.0);

    pub const CENTER_LEADING: Self = Self::new(-1.0, 0.0);
    pub const CENTER: Self = Self::new(0.0, 0.0);
    pub const CENTER_TRAILING: Self = Self::new(1.0, 0.0);
    pub const BOTTOM_LOADING: Self = Self::new(-1.0, 1.0);
    pub const BOTTOM_CENTER: Self = Self::new(0.0, 1.0);
    pub const BOTTOM_TRAILING: Self = Self::new(1.0, 1.0);

    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    /// Computes the Point offset for positioning a child of `child_size` within `parent_size`.
    pub fn align(&self, child_size: Size, parent_size: Size) -> Point {
        let free_w = (parent_size.width - child_size.width).max(0.0);
        let free_h = (parent_size.height - child_size.height).max(0.0);
        Point::new(
            (free_w / 2.0) * (self.x + 1.0),
            (free_h / 2.0) * (self.y + 1.0),
        )
    }
}
