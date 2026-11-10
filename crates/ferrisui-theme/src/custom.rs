use crate::color_scheme::ColorScheme;
use crate::dark::dark_color_scheme;
use crate::theme::ThemeData;
use crate::tokens::{ElevationScale, ShapeScale, SpacingScale};
use crate::typography::TypeScale;
use ferrisui_core::color::Color;

pub struct ThemeBuilder {
    is_dark: bool,
    colors: ColorScheme,
    spacing: SpacingScale,
    shape: ShapeScale,
    elevation: ElevationScale,
}

impl ThemeBuilder {
    pub fn new() -> Self {
        Self {
            is_dark: true,
            colors: dark_color_scheme(),
            spacing: SpacingScale::default(),
            shape: ShapeScale::default(),
            elevation: ElevationScale::default(),
        }
    }

    pub fn dark_mode(mut self, is_dark: bool) -> Self {
        self.is_dark = is_dark;
        self
    }

    pub fn primary(mut self, color: Color) -> Self {
        self.colors.primary = color;
        self
    }

    pub fn secondary(mut self, color: Color) -> Self {
        self.colors.secondary = color;
        self
    }
    pub fn background(mut self, color: Color) -> Self {
        self.colors.background = color;
        self
    }
    pub fn surface(mut self, color: Color) -> Self {
        self.colors.surface = color;
        self
    }
    pub fn text_color(mut self, color: Color) -> Self {
        self.colors.on_background = color;
        self.colors.on_surface = color;
        self
    }

    pub fn shape(mut self, shape: ShapeScale) -> Self {
        self.shape = shape;
        self
    }
    pub fn spacing(mut self, spacing: SpacingScale) -> Self {
        self.spacing = spacing;
        self
    }
    pub fn build(self) -> ThemeData {
        let typography = TypeScale::generate(self.colors.on_background);
        ThemeData {
            is_dark: self.is_dark,
            colors: self.colors,
            typography,
            spacing: self.spacing,
            shape: self.shape,
            elevation: self.elevation,
        }
    }
}
