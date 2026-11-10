use ferrisui_core::color::Color;

#[derive(Debug, Clone, PartialEq)]
pub struct ColorScheme {
    pub primary: Color,
    pub on_primary: Color,
    pub primary_container: Color,
    pub on_primary_container: Color,

    pub secondary: Color,
    pub on_secondary: Color,
    pub secondary_container: Color,
    pub on_secondary_container: Color,

    pub tertiary: Color,
    pub on_tertiary: Color,

    pub surface: Color,
    pub on_surface: Color,

    pub surface_variant: Color,
    pub on_surface_variant: Color,

    pub background: Color,
    pub on_background: Color,

    pub error: Color,
    pub on_error: Color,
    pub success: Color,
    pub on_success: Color,
    pub warning: Color,
    pub on_warning: Color,

    pub outline: Color,
    pub outline_variant: Color,
}
