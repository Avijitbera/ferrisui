use crate::color_scheme::ColorScheme;
use crate::theme::ThemeData;
use crate::tokens::{ElevationScale, ShapeScale, SpacingScale};
use crate::typography::TypeScale;
use ferrisui_core::color::Color;

pub fn light_color_schema() -> ColorScheme {
    ColorScheme {
        primary: Color::hex(0x4F46E5),
        on_primary: Color::hex(0xFFFFFF),
        primary_container: Color::hex(0xEEF2FF),
        on_primary_container: Color::hex(0x312E81),
        secondary: Color::hex(0x7C3AED),
        on_secondary: Color::hex(0xFFFFFF),
        secondary_container: Color::hex(0xF5F3FF),
        on_secondary_container: Color::hex(0x4C1D95),
        tertiary: Color::hex(0x0284C7), // Sky 600
        on_tertiary: Color::hex(0xFFFFFF),
        surface: Color::hex(0xFFFFFF), // Pure White
        on_surface: Color::hex(0x18181B),
        surface_variant: Color::hex(0xF4F4F5),
        on_surface_variant: Color::hex(0x71717A),
        background: Color::hex(0xFAFAFA), // Crisp Light Gray
        on_background: Color::hex(0x09090B),
        error: Color::hex(0xDC2626),
        on_error: Color::hex(0xFFFFFF),
        success: Color::hex(0x059669),
        on_success: Color::hex(0xFFFFFF),
        warning: Color::hex(0xD97706),
        on_warning: Color::hex(0xFFFFFF),
        outline: Color::rgba(0.0, 0.0, 0.0, 0.12),
        outline_variant: Color::rgba(0.0, 0.0, 0.0, 0.06),
    }
}

pub fn default_light_theme() -> ThemeData {
    let colors = light_color_scheme();
    let typography = TypeScale::generate(colors.on_background);
    ThemeData {
        is_dark: false,
        colors,
        typography,
        spacing: SpacingScale::default(),
        shape: ShapeScale::default(),
        elevation: ElevationScale::default(),
    }
}
