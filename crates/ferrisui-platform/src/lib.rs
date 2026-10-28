pub mod clipboard;
pub mod cursor;
pub mod event_loop;
pub mod file_dialog;
pub mod monitor;
pub mod system;
pub mod window;

pub mod prelude {
    pub use crate::clipboard::Clipboard;
    pub use crate::cursor::CursorIcon;
    pub use crate::event_loop::PlatformApp;
    pub use crate::file_dialog::FileDialog;
    pub use crate::monitor::MonitorInfo;
    pub use crate::system::{System, ThemePreference};
    pub use crate::window::{Window, WindowConfig};
}
