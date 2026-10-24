pub mod build_context;
pub mod callback;
pub mod color;
pub mod event;
pub mod geometry;
pub mod id;
pub mod modifier;
pub mod state;
pub mod view;
pub mod view_node;

pub mod prelude {
    pub use crate::build_context::BuildContext;
    pub use crate::callback::Callback;
    pub use crate::color::Color;
    pub use crate::event::{Event, KeyEvent, MouseButton, MouseEvent};
    pub use crate::geometry::{Corners, EdgeInsets, Point, Rect, Size};
    pub use crate::id::ViewId;
    pub use crate::modifier::ViewModifierExt;
    pub use crate::state::Signal;
    pub use crate::view::{AnyView, View};
    pub use crate::view_node::{NodeKind, ViewNode};
}
