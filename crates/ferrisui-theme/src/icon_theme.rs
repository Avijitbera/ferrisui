use ferrisui_core::color::Color;

#[derive(Debug, Clone, PartialEq)]
pub struct IconTheme {
    pub color: Color,
    pub size: f32,
    pub size_sm: f32,
    pub size_md: f32,
    pub size_lg: f32,
    pub size_xl: f32,

    pub stroke_width: f32,
}

impl IconTheme {
    pub fn dark(text_color: Color) -> Self {
        Self {
            color: text_color,
            size: 24.0,
            size_sm: 16.0,
            size_md: 24.0,
            size_lg: 32.0,
            size_xl: 48.0,
            stroke_width: 2.0,
        }
    }

    pub fn light(text_color: Color) -> Self {
        Self {
            color: text_color,
            size: 24.0,
            size_sm: 16.0,
            size_md: 24.0,
            size_lg: 32.0,
            size_xl: 48.0,
            stroke_width: 2.0,
        }
    }
}
