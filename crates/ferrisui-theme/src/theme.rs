use crate::color_scheme::ColorScheme;
use crate::custom::ThemeBuilder;
use crate::dark::default_dark_theme;
use crate::icon_theme::IconTheme;
use crate::light::default_light_theme;
use crate::tokens::{ElevationScale, ShapeScale, SpacingScale};
use crate::typography::TypeScale;
use ferrisui_core::build_context::BuildContext;
use std::sync::LazyLock;

#[derive(Debug, Clone, PartialEq)]
pub struct ThemeData {
    pub is_dark: bool,
    pub colors: ColorScheme,
    pub typography: TypeScale,
    pub spacing: SpacingScale,
    pub shape: ShapeScale,
    pub elevation: ElevationScale,
    pub icons: IconTheme,
}

impl ThemeData {
    /// Built-in sleek dark theme (Default).
    pub fn dark() -> Self {
        default_dark_theme()
    }
    /// Built-in crisp light theme.
    pub fn light() -> Self {
        default_light_theme()
    }
    /// Start a fluent custom theme builder.
    pub fn custom() -> ThemeBuilder {
        ThemeBuilder::new()
    }
}
impl Default for ThemeData {
    fn default() -> Self {
        Self::dark()
    }
}

static DEFAULT_DARK_THEME: LazyLock<ThemeData> = LazyLock::new(ThemeData::dark);

/// Extension trait enabling ergonomic `.theme()` and `.set_theme()` on `BuildContext`.
pub trait ThemeContextExt {
    /// Accesses the active theme from the context, falling back to the default dark theme.
    fn theme(&self) -> &ThemeData;

    /// Sets or overrides the active theme in the build context.
    fn set_theme(&mut self, theme: ThemeData);
}

impl ThemeContextExt for BuildContext {
    fn theme(&self) -> &ThemeData {
        self.get::<ThemeData>()
            .unwrap_or_else(|| &DEFAULT_DARK_THEME)
    }

    fn set_theme(&mut self, theme: ThemeData) {
        self.provide(theme);
    }
}
