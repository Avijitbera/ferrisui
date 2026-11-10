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
use ferrisui_theme::ThemeData;
use std::sync::Arc;

pub struct App<V: View + 'static> {
    root_view: V,
    config: WindowConfig,
    clear_color: Option<Color>,
    theme: ThemeData,
}

impl<V: View + 'static> App<V> {
    pub fn new(root_view: V) -> Self {
        let theme = ThemeData::dark();
        Self {
            root_view,
            config: WindowConfig::default(),
            clear_color: None,
            theme,
        }
    }

    pub fn theme(mut self, theme: ThemeData) -> Self {
        self.theme = theme;
        self
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
        self.clear_color = Some(color);
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
        build_ctx.provide(self.theme.clone());

        let mut cursor_pos = Point::new(-1000.0, -1000.0);
        let mut _is_mouse_down = false;
        let mut last_root_node: Option<ViewNode> = None;
        let root_view = self.root_view;

        let frame_clear_color = self.clear_color.unwrap_or(self.theme.colors.background);

        app.run(move |win, event, _should_exit| match event {
            Event::Mouse(MouseEvent::Moved { position }) => {
                cursor_pos = position;
                win.request_redraw();
            }
            Event::Mouse(MouseEvent::Button {
                button,
                state,
                position,
            }) => {
                if button == MouseButton::Left {
                    _is_mouse_down = state == ElementState::Pressed;
                    if state == ElementState::Pressed {
                        if let Some(ref root) = last_root_node {
                            let click_target = if position.x >= 0.0 && position.y >= 0.0 {
                                position
                            } else {
                                cursor_pos
                            };
                            let handled = dispatch_click(root, click_target);
                            if handled {
                                win.request_redraw();
                            }
                        }
                    }
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
                build_ctx.reset();

                // 1. Declaratively build the virtual view tree
                let mut root_node = root_view.build(&mut build_ctx);

                // 2. Resolve layouts recursively
                let window_bounds = Rect::new(0.0, 0.0, logical.width, logical.height);
                layout_node_recursive(&mut root_node, window_bounds);

                // 3. Render tree recursively to GPU batch
                render_node(&mut renderer, &root_node, cursor_pos);

                // 4. Save laid-out tree for interaction hit-testing
                last_root_node = Some(root_node);

                // 5. Submit GPU frame
                let _ = renderer.render_frame(frame_clear_color);
            }
            _ => {}
        })?;

        Ok(())
    }
}

/// Recursively dispatches a mouse click to the topmost interactive ViewNode under the cursor.
fn dispatch_click(node: &ViewNode, pos: Point) -> bool {
    if !node.bounds.contains(pos) {
        return false;
    }

    // Children in reverse order (topmost z-order first)
    for child in node.children.iter().rev() {
        if dispatch_click(child, pos) {
            return true;
        }
    }

    if let Some(ref cb) = node.on_click {
        cb.emit(());
        return true;
    }

    false
}

/// Computes the preferred (intrinsic) size for a ViewNode and its children.
fn compute_preferred_size(node: &ViewNode) -> Size {
    match &node.kind {
        NodeKind::Text {
            text, font_size, ..
        } => {
            let px = (font_size / 6.0).max(1.5).min(4.0);
            let char_w = px * 3.0;
            let char_gap = px * 1.0;
            let total_w = (text.len() as f32 * (char_w + char_gap) - char_gap).max(20.0);
            let total_h = (px * 5.0).max(*font_size);
            Size::new(total_w + 16.0, total_h + 12.0)
        }
        NodeKind::Container { padding, .. } => {
            if let Some(child) = node.children.first() {
                let child_size = compute_preferred_size(child);
                Size::new(
                    child_size.width + padding.horizontal(),
                    child_size.height + padding.vertical(),
                )
            } else {
                Size::new(40.0 + padding.horizontal(), 40.0 + padding.vertical())
            }
        }
        NodeKind::Flex {
            is_vertical,
            spacing,
        } => {
            let count = node.children.len();
            if count == 0 {
                return Size::ZERO;
            }
            if *is_vertical {
                let mut max_w: f32 = 0.0;
                let mut total_h: f32 = 0.0;
                for (i, child) in node.children.iter().enumerate() {
                    let s = compute_preferred_size(child);
                    max_w = max_w.max(s.width);
                    total_h += s.height;
                    if i + 1 < count {
                        total_h += *spacing;
                    }
                }
                Size::new(max_w, total_h)
            } else {
                let mut total_w: f32 = 0.0;
                let mut max_h: f32 = 0.0;
                for (i, child) in node.children.iter().enumerate() {
                    let s = compute_preferred_size(child);
                    total_w += s.width;
                    max_h = max_h.max(s.height);
                    if i + 1 < count {
                        total_w += *spacing;
                    }
                }
                Size::new(total_w, max_h)
            }
        }
        _ => Size::new(80.0, 36.0),
    }
}

/// Recursively computes exact bounds for each node, centering elements within flex stacks and containers.
fn layout_node_recursive(node: &mut ViewNode, bounds: Rect) {
    node.bounds = bounds;

    match &node.kind {
        NodeKind::Container { padding, .. } => {
            let inner_rect = bounds.inset(*padding);
            for child in &mut node.children {
                let pref = compute_preferred_size(child);
                let w = pref.width.min(inner_rect.size.width);
                let h = pref.height.min(inner_rect.size.height);
                let x = inner_rect.origin.x + (inner_rect.size.width - w) * 0.5;
                let y = inner_rect.origin.y + (inner_rect.size.height - h) * 0.5;
                layout_node_recursive(child, Rect::new(x, y, w, h));
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
                let child_sizes: Vec<Size> =
                    node.children.iter().map(compute_preferred_size).collect();
                let total_content_h: f32 = child_sizes.iter().map(|s| s.height).sum::<f32>()
                    + *spacing * (count - 1) as f32;
                let start_y =
                    bounds.origin.y + (bounds.size.height - total_content_h).max(0.0) * 0.5;
                let mut current_y = start_y;

                for (i, child) in node.children.iter_mut().enumerate() {
                    let pref = child_sizes[i];
                    let w = pref.width.min(bounds.size.width);
                    let h = pref.height;
                    let x = bounds.origin.x + (bounds.size.width - w) * 0.5;
                    layout_node_recursive(child, Rect::new(x, current_y, w, h));
                    current_y += h + *spacing;
                }
            } else {
                let child_sizes: Vec<Size> =
                    node.children.iter().map(compute_preferred_size).collect();
                let total_content_w: f32 = child_sizes.iter().map(|s| s.width).sum::<f32>()
                    + *spacing * (count - 1) as f32;
                let start_x =
                    bounds.origin.x + (bounds.size.width - total_content_w).max(0.0) * 0.5;
                let mut current_x = start_x;

                for (i, child) in node.children.iter_mut().enumerate() {
                    let pref = child_sizes[i];
                    let w = pref.width;
                    let h = pref.height.min(bounds.size.height);
                    let y = bounds.origin.y + (bounds.size.height - h) * 0.5;
                    layout_node_recursive(child, Rect::new(current_x, y, w, h));
                    current_x += w + *spacing;
                }
            }
        }
        _ => {
            for child in &mut node.children {
                layout_node_recursive(child, bounds);
            }
        }
    }
}

/// Renders a ViewNode and its children to the GPU batch.
fn render_node(renderer: &mut Renderer, node: &ViewNode, cursor: Point) {
    match &node.kind {
        NodeKind::Container {
            background,
            corner_radius,
            ..
        } => {
            let is_interactive = node.on_click.is_some();
            let is_hovered = is_interactive && node.bounds.contains(cursor);

            let mut bg = *background;
            let mut border_color = Color::rgba(1.0, 1.0, 1.0, 0.12);
            let mut border_width = if is_interactive { 1.0 } else { 0.0 };

            if is_hovered {
                border_color = Color::hex(0x38BDF8); // Tailwind Sky 400
                border_width = 1.5;
                // Brighten interactive container on hover
                bg = Color::rgba(
                    (bg.r * 1.15).min(1.0),
                    (bg.g * 1.15).min(1.0),
                    (bg.b * 1.15).min(1.0),
                    bg.a,
                );
            }

            if bg.a > 0.0 || border_width > 0.0 {
                renderer.draw_rect(node.bounds, bg, *corner_radius, border_width, border_color);
            }
        }
        NodeKind::Text {
            text,
            font_size,
            color,
        } => {
            draw_text_bitmap(renderer, text, node.bounds, *font_size, *color);
        }
        _ => {}
    }

    for child in &node.children {
        render_node(renderer, child, cursor);
    }
}

/// Renders crisp text glyphs using a built-in GPU 3x5 vector pixel font.
fn draw_text_bitmap(
    renderer: &mut Renderer,
    text: &str,
    bounds: Rect,
    font_size: f32,
    color: Color,
) {
    if text.is_empty() {
        return;
    }

    let px = (font_size / 6.0).max(1.5).min(4.0);
    let char_w = px * 3.0;
    let char_gap = px * 1.0;
    let char_h = px * 5.0;
    let total_w = (text.len() as f32 * (char_w + char_gap) - char_gap).max(1.0);

    let start_x = bounds.origin.x + (bounds.size.width - total_w).max(0.0) * 0.5;
    let start_y = bounds.origin.y + (bounds.size.height - char_h).max(0.0) * 0.5;

    let mut cur_x = start_x;
    for ch in text.chars() {
        let glyph = get_glyph_3x5(ch);
        for row in 0..5 {
            for col in 0..3 {
                let bit = row * 3 + col;
                if (glyph & (1 << bit)) != 0 {
                    let rx = cur_x + col as f32 * px;
                    let ry = start_y + row as f32 * px;
                    renderer.draw_rect(
                        Rect::new(rx, ry, px * 0.95, px * 0.95),
                        color,
                        0.4,
                        0.0,
                        Color::TRANSPARENT,
                    );
                }
            }
        }
        cur_x += char_w + char_gap;
    }
}

const fn pack_rows(r: [u16; 5]) -> u16 {
    r[0] | (r[1] << 3) | (r[2] << 6) | (r[3] << 9) | (r[4] << 12)
}

/// 3x5 compact bitmap glyph dictionary for standard ASCII characters.
fn get_glyph_3x5(c: char) -> u16 {
    let uc = c.to_ascii_uppercase();
    match uc {
        '0' => pack_rows([7, 5, 5, 5, 7]),
        '1' => pack_rows([2, 6, 2, 2, 7]),
        '2' => pack_rows([7, 1, 7, 4, 7]),
        '3' => pack_rows([7, 1, 7, 1, 7]),
        '4' => pack_rows([5, 5, 7, 1, 1]),
        '5' => pack_rows([7, 4, 7, 1, 7]),
        '6' => pack_rows([7, 4, 7, 5, 7]),
        '7' => pack_rows([7, 1, 2, 2, 2]),
        '8' => pack_rows([7, 5, 7, 5, 7]),
        '9' => pack_rows([7, 5, 7, 1, 7]),

        'A' => pack_rows([7, 5, 7, 5, 5]),
        'B' => pack_rows([6, 5, 6, 5, 6]),
        'C' => pack_rows([7, 4, 4, 4, 7]),
        'D' => pack_rows([6, 5, 5, 5, 6]),
        'E' => pack_rows([7, 4, 7, 4, 7]),
        'F' => pack_rows([7, 4, 6, 4, 4]),
        'G' => pack_rows([7, 4, 5, 5, 7]),
        'H' => pack_rows([5, 5, 7, 5, 5]),
        'I' => pack_rows([7, 2, 2, 2, 7]),
        'J' => pack_rows([1, 1, 1, 5, 7]),
        'K' => pack_rows([5, 5, 6, 5, 5]),
        'L' => pack_rows([4, 4, 4, 4, 7]),
        'M' => pack_rows([5, 7, 5, 5, 5]),
        'N' => pack_rows([5, 7, 7, 5, 5]),
        'O' => pack_rows([7, 5, 5, 5, 7]),
        'P' => pack_rows([7, 5, 7, 4, 4]),
        'Q' => pack_rows([7, 5, 5, 7, 3]),
        'R' => pack_rows([7, 5, 6, 5, 5]),
        'S' => pack_rows([7, 4, 7, 1, 7]),
        'T' => pack_rows([7, 2, 2, 2, 2]),
        'U' => pack_rows([5, 5, 5, 5, 7]),
        'V' => pack_rows([5, 5, 5, 5, 2]),
        'W' => pack_rows([5, 5, 5, 7, 5]),
        'X' => pack_rows([5, 5, 2, 5, 5]),
        'Y' => pack_rows([5, 5, 2, 2, 2]),
        'Z' => pack_rows([7, 1, 2, 4, 7]),

        '+' => pack_rows([0, 2, 7, 2, 0]),
        '-' => pack_rows([0, 0, 7, 0, 0]),
        ':' => pack_rows([0, 2, 0, 2, 0]),
        '!' => pack_rows([2, 2, 2, 0, 2]),
        '?' => pack_rows([7, 1, 2, 0, 2]),
        '.' => pack_rows([0, 0, 0, 0, 2]),
        ',' => pack_rows([0, 0, 0, 2, 4]),
        '(' => pack_rows([2, 4, 4, 4, 2]),
        ')' => pack_rows([2, 1, 1, 1, 2]),
        '=' => pack_rows([0, 7, 0, 7, 0]),
        '/' => pack_rows([1, 1, 2, 4, 4]),
        '_' => pack_rows([0, 0, 0, 0, 7]),
        ' ' => 0,
        _ => pack_rows([7, 5, 5, 5, 7]), // Fallback box
    }
}
