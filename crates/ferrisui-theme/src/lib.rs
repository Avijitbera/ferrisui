pub mod color_scheme;
pub mod custom;
pub mod dark;
pub mod light;
pub mod theme;
pub mod tokens;
pub mod typography;

pub use color_scheme::ColorScheme;
pub use custom::ThemeBuilder;
pub use theme::{ThemeContextExt, ThemeData};
pub use tokens::{Elevation, ElevationScale, ShapeScale, SpacingScale};
pub use typography::{FontWeight, TextStyle, TypeScale};

pub mod prelude {
    pub use crate::color_scheme::ColorScheme;
    pub use crate::custom::ThemeBuilder;
    pub use crate::theme::{ThemeContextExt, ThemeData};
    pub use crate::tokens::{Elevation, ElevationScale, ShapeScale, SpacingScale};
    pub use crate::typography::{FontWeight, TextStyle, TypeScale};
}
