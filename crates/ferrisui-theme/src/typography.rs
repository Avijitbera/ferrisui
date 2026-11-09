use ferrisui_core::color::Color;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FontWeight {
    Thin = 100,
    Light = 300,
    Regular = 400,
    Medium = 500,
    SemiBold = 600,
    Bold = 700,
    ExtraBold = 800,
    Black = 900,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TextStyle {
    pub size: f32,
    pub weight: FontWeight,
    pub line_height: f32,
    pub letter_spacing: f32,
    pub color: Color,
}

impl TextStyle {
    pub fn new(size: f32, weight: FontWeight, line_height: f32, color: Color) -> Self {
        Self {
            size,
            weight,
            line_height,
            letter_spacing: 0.0,
            color,
        }
    }

    pub fn with_color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TypeScale {
    pub display_large: TextStyle,
    pub display_medium: TextStyle,
    pub display_small: TextStyle,

    pub headline_large: TextStyle,
    pub headline_medium: TextStyle,
    pub headline_small: TextStyle,

    pub title_large: TextStyle,
    pub title_medium: TextStyle,
    pub title_small: TextStyle,

    pub body_large: TextStyle,
    pub body_medium: TextStyle,
    pub body_small: TextStyle,

    pub label_large: TextStyle,
    pub label_medium: TextStyle,
    pub label_small: TextStyle,
}

impl TypeScale {
    pub fn generate(text_color: Color) -> Self {
        Self {
            display_large: TextStyle::new(57.0, FontWeight::Regular, 64.0, text_color),
            display_medium: TextStyle::new(45.0, FontWeight::Regular, 52.0, text_color),
            display_small: TextStyle::new(36.0, FontWeight::Regular, 44.0, text_color),
            headline_large: TextStyle::new(32.0, FontWeight::SemiBold, 40.0, text_color),
            headline_medium: TextStyle::new(28.0, FontWeight::SemiBold, 36.0, text_color),
            headline_small: TextStyle::new(24.0, FontWeight::SemiBold, 32.0, text_color),
            title_large: TextStyle::new(22.0, FontWeight::Medium, 28.0, text_color),
            title_medium: TextStyle::new(16.0, FontWeight::Medium, 24.0, text_color),
            title_small: TextStyle::new(14.0, FontWeight::Medium, 20.0, text_color),
            body_large: TextStyle::new(16.0, FontWeight::Regular, 24.0, text_color),
            body_medium: TextStyle::new(14.0, FontWeight::Regular, 20.0, text_color),
            body_small: TextStyle::new(12.0, FontWeight::Regular, 16.0, text_color),
            label_large: TextStyle::new(14.0, FontWeight::Medium, 20.0, text_color),
            label_medium: TextStyle::new(12.0, FontWeight::Medium, 16.0, text_color),
            label_small: TextStyle::new(11.0, FontWeight::Medium, 16.0, text_color),
        }
    }
}
