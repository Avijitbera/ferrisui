use crate::color_scheme::ColorScheme;
use crate::theme::ThemeData;
use crate::tokens::{ElevationScale, ShapeScale, SpacingScale};
use crate::typography::TypeScale;
use ferrisui_core::color::Color;

pub fn dark_color_schema() -> ColorScheme {
    ColorScheme {
        primary: Color::hex(0x667EEA), // Soft Indigo
        on_primary: Color::hex(0xFFFFFF),
        primary_container: Color::hex(0x2E3856),
        on_primary_container: Color::hex(0xDDE2FF),
        secondary: Color::hex(0x764BA2), // Soft Purple
        on_secondary: Color::hex(0xFFFFFF),
        secondary_container: Color::hex(0x3B2551),
        on_secondary_container: Color::hex(0xEFDCFF),
        tertiary: Color::hex(0x38BDF8), // Tailwind Sky 400
        on_tertiary: Color::hex(0x0F172A),
        surface: Color::hex(0x1A1A2E),

        on_surface: Color::hex(0xE4E4E7),
        surface_variant: Color::hex(0x232338),
        on_surface_variant: Color::hex(0xA1A1AA),
        background: Color::hex(0x0A0A0F), // Near Black
        on_background: Color::hex(0xF4F4F5),
        error: Color::hex(0xEF4444),
        on_error: Color::hex(0xFFFFFF),
        success: Color::hex(0x10B981),
        on_success: Color::hex(0xFFFFFF),
        warning: Color::hex(0xF59E0B),
        on_warning: Color::hex(0x0F172A),
        outline: Color::rgba(1.0, 1.0, 1.0, 0.14),
        outline_variant: Color::rgba(1.0, 1.0, 1.0, 0.08),
    }
}

pub fn default_dark_theme() -> ThemeData {
    let colors = dark_color_schema();
    let typography = TypeScale::generate(colors.on_background);
    ThemeData {
        is_dark: true,
        colors,
        typography,
        spacing: SpacingScale::default(),
        shape: ShapeScale::default(),
        elevation: ElevationScale::default(),
    }
}
