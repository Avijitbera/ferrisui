use ferrisui_core::geometry::EdgeInsets;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ShapeScale {
    pub none: f32,
    pub xs: f32,
    pub sm: f32,
    pub md: f32,
    pub lg: f32,
    pub xl: f32,
    pub full: f32,
}

impl Default for ShapeScale {
    fn default() -> Self {
        Self {
            none: 0.0,
            sm: 6.0,
            xs: 4.0,
            md: 8.0,
            lg: 12.0,
            xl: 16.0,
            full: 9999.0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SpacingScale {
    pub none: f32,
    pub xxs: f32,
    pub xs: f32,
    pub sm: f32,
    pub md: f32,
    pub lg: f32,
    pub xl: f32,
    pub xxl: f32,
}

impl SpacingScale {
    pub fn inset_all(&self, val: f32) -> EdgeInsets {
        EdgeInsets::all(val)
    }

    pub fn inset_symmetric(&self, horizontal: f32, vertical: f32) -> EdgeInsets {
        EdgeInsets::symmetric(vertical, horizontal)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Elevation {
    pub y_offset: f32,
    pub blur_radius: f32,
    pub spread: f32,
    pub opacity: f32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ElevationScale {
    pub level0: Elevation,
    pub level1: Elevation,
    pub level2: Elevation,
    pub level3: Elevation,
    pub level4: Elevation,
    pub level5: Elevation,
}

impl Default for ElevationScale {
    fn default() -> Self {
        Self {
            level0: Elevation {
                y_offset: 0.0,
                blur_radius: 0.0,
                spread: 0.0,
                opacity: 0.0,
            },
            level1: Elevation {
                y_offset: 1.0,
                blur_radius: 3.0,
                spread: 0.0,
                opacity: 0.15,
            },
            level2: Elevation {
                y_offset: 2.0,
                blur_radius: 6.0,
                spread: 0.0,
                opacity: 0.18,
            },
            level3: Elevation {
                y_offset: 4.0,
                blur_radius: 12.0,
                spread: 0.0,
                opacity: 0.20,
            },
            level4: Elevation {
                y_offset: 8.0,
                blur_radius: 18.0,
                spread: 0.0,
                opacity: 0.22,
            },
            level5: Elevation {
                y_offset: 12.0,
                blur_radius: 24.0,
                spread: 0.0,
                opacity: 0.25,
            },
        }
    }
}
