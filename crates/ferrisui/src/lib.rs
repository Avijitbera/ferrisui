pub mod app;
pub mod prelude;

pub use app::App;
pub struct FerrisUI;

impl FerrisUI {
    pub fn app<V: ferrisui_core::view::View + 'static>(root_view: V) -> App<V> {
        App::new(root_view)
    }
}
