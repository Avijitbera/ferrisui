pub mod app;
pub mod prelude;

pub use app::App;
pub use ferrisui_core as core;
pub use ferrisui_layout as layout;
pub use ferrisui_platform as platform;
pub use ferrisui_render as render;
pub use ferrisui_theme as theme;
pub use ferrisui_widgets as widgets;

pub struct FerrisUI;

impl FerrisUI {
    pub fn app<V: ferrisui_core::view::View + 'static>(root_view: V) -> App<V> {
        App::new(root_view)
    }
}

