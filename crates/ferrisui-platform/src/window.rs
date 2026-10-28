use crate::cursor::CursorIcon;
use ferrisui_core::geometry::Size;
use raw_window_handle::{HasDisplayHandle, HasWindowHandle};
use std::sync::Arc;
use winit::dpi::LogicalSize;
use winit::event_loop::EventLoopWindowTarget;
use winit::window::{Window as WinitWindow, WindowBuilder};

#[derive(Debug, Clone)]
pub struct WindowConfig {
    pub title: String,
    pub size: Size,
    pub min_size: Option<Size>,
    pub max_size: Option<Size>,
    pub resizable: bool,
    pub decorations: bool,
    pub transparent: bool,
}

impl Default for WindowConfig {
    fn default() -> Self {
        Self {
            title: "FerrisUI Application".to_string(),
            size: Size::new(800.0, 600.0),
            min_size: Some(Size::new(320.0, 240.0)),
            max_size: None,
            resizable: true,
            decorations: true,
            transparent: false,
        }
    }
}

#[derive(Clone)]
pub struct Window {
    inner: Arc<WinitWindow>,
}

impl Window {
    pub fn new<T>(
        event_loop: &EventLoopWindowTarget<T>,
        config: WindowConfig,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let mut builder = WindowBuilder::new()
            .with_title(config.title)
            .with_inner_size(LogicalSize::new(config.size.width, config.size.height))
            .with_resizable(config.resizable)
            .with_decorations(config.decorations)
            .with_transparent(config.transparent);

        if let Some(min) = config.min_size {
            builder = builder.with_min_inner_size(LogicalSize::new(min.width, min.height));
        }

        let winit_window = builder.build(event_loop)?;
        Ok(Self {
            inner: Arc::new(winit_window),
        })
    }

    pub fn raw_window(&self) -> &WinitWindow {
        &self.inner
    }

    pub fn inner_size(&self) -> Size {
        let size = self.inner.inner_size();
        let scale = self.scale_factor();
        Size::new(size.width as f32 / scale, size.height as f32 / scale)
    }

    pub fn physical_size(&self) -> (u32, u32) {
        let size = self.inner.inner_size();
        (size.width, size.height)
    }

    pub fn scale_factor(&self) -> f32 {
        self.inner.scale_factor() as f32
    }

    pub fn set_title(&self, title: &str) {
        self.inner.set_title(title);
    }

    pub fn set_cursor(&self, icon: CursorIcon) {
        self.inner.set_cursor_icon(icon.to_winit());
    }

    pub fn request_redraw(&self) {
        self.inner.request_redraw();
    }
}

impl HasDisplayHandle for Window {
    fn display_handle(
        &self,
    ) -> Result<raw_window_handle::DisplayHandle<'_>, raw_window_handle::HandleError> {
        self.inner.display_handle()
    }
}

impl HasWindowHandle for Window {
    fn window_handle(
        &self,
    ) -> Result<raw_window_handle::WindowHandle<'_>, raw_window_handle::HandleError> {
        self.inner.window_handle()
    }
}
