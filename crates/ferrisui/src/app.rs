use ferrisui_core::{
    build_context::BuildContext,
    color::Color,
    event::{ElementState, Event, MouseButton, MouseEvent},
    geometry::{Point, Rect, Size},
    view::View,
    view_node::{NodeKind, ViewNode},
};
use ferrisui_platform::{event_loop::PlatformApp, window::WindowConfig};
use ferrisui_render::{GpuContext, Renderer};
use std::sync::Arc;

pub struct App<V: View + 'static> {
    root_view: V,
    config: WindowConfig,
    clear_color: Color,
}

impl<V: View + 'static> App<V> {
    pub fn new(root_view: V) -> Self {
        Self {
            root_view,
            config: WindowConfig::default(),
            clear_color: Color::hex(0x0F172A),
        }
    }

    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.config.title = title.into();
        self
    }

    pub fn size(mut self, width: f32, height: f32) -> Self {
        self.config.size = Size::new(width, height);
        self
    }

    pub fn clear_color(mut self, color: Color) -> Self {
        self.clear_color = color;
        self
    }

    pub fn run(self) -> Result<(), Box<dyn std::error::Error>> {
        let app = PlatformApp::new(self.config)?;

        let window = Arc::new(app.window().clone());
        let (phys_w, phys_h) = window.physical_size();

        let gpu_context = pollster::block_on(GpuContext::new(window.clone(), phys_w, phys_h))?;

        let mut renderer = Renderer::new(gpu_context);
        let logical = window.inner_size();
        renderer.resize_logical(logical.width, logical.height);

        let mut build_ctx = BuildContext::new();
        let mut cursor_pos = Point::new(-1000.0, -1000.0);
        let mut _is_mouse_down = false;
        let root_view = self.root_view;
        let clear_color = self.clear_color;

        app.run(move |win, event, _should_exit| match event {
            Event::Mouse(MouseEvent::Moved { position }) => {
                cursor_pos = position;
                win.request_redraw();
            }
            Event::Mouse(MouseEvent::Button { button, state, .. }) => {
                if button == MouseButton::Left {
                    _is_mouse_down = state == ElementState::Pressed;
                    win.request_redraw();
                }
            }
            Event::Resized { width, height } => {
                renderer.resize(width, height);
                let logical = win.inner_size();
                renderer.resize_logical(logical.width, logical.height);
                win.request_redraw();
            }
            Event::RedrawRequested => {
                let logical = win.inner_size();
                // 1. Declaratively build the virtual view tree
                let mut root_node = root_view.build(&mut build_ctx);
                // 2. Resolve layouts (root node takes full window size)
                root_node.bounds = Rect::new(0.0, 0.0, logical.width, logical.height);
                layout_node(&mut root_node);
                // 3. Render tree recursively to GPU batch
                render_node(&mut renderer, &root_node, cursor_pos);
                // 4. Submit GPU pass
                let _ = renderer.render_frame(clear_color);
            }
            _ => {}
        })?;

        Ok(())
    }
}

fn layout_node(node: &mut ViewNode) {
    match &node.kind {
        NodeKind::Container { padding, .. } => {
            let inner_rect = node.bounds.inset(*padding);
            for child in &mut node.children {
                child.bounds = inner_rect;
                layout_node(child);
            }
        }
        NodeKind::Flex {
            is_vertical,
            spacing,
        } => {
            let count = node.children.len();
            if count == 0 {
                return;
            }

            if *is_vertical {
                let total_spacing = *spacing * (count - 1) as f32;
                let child_h = (node.bounds.size.height - total_spacing).max(0.0) / count as f32;
                let mut current_y = node.bounds.origin.y;

                for child in &mut node.children {
                    child.bounds = Rect::new(
                        node.bounds.origin.x,
                        current_y,
                        node.bounds.size.width,
                        child_h,
                    );
                    layout_node(child);
                    current_y += child_h + *spacing;
                }
            } else {
                let total_spacing = *spacing * (count - 1) as f32;
                let child_w = (node.bounds.size.width - total_spacing).max(0.0) / count as f32;
                let mut current_x = node.bounds.origin.x;
                for child in &mut node.children {
                    child.bounds = Rect::new(
                        current_x,
                        node.bounds.origin.y,
                        child_w,
                        node.bounds.size.height,
                    );
                    layout_node(child);
                    current_x += child_w + *spacing;
                }
            }
        }
        _ => {
            for child in &mut node.children {
                layout_node(child);
            }
        }
    }
}

fn render_node(renderer: &mut Renderer, node: &ViewNode, cursor: Point) {
    match &node.kind {
        NodeKind::Container {
            background,
            corner_radius,
            ..
        } => {
            let is_hovered = node.bounds.contains(cursor);
            let border_color = if is_hovered {
                Color::hex(0x38BDF8)
            } else {
                Color::rgba(1.0, 1.0, 1.0, 0.12)
            };

            renderer.draw_rect(node.bounds, *background, *corner_radius, 1.5, border_color);
        }
        _ => {}
    }
    for child in &node.children {
        render_node(renderer, child, cursor);
    }
}
