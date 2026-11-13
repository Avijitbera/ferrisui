use ferrisui_core::{
    build_context::BuildContext,
    color::Color,
    event::{ElementState, Event, KeyEvent, MouseButton, MouseEvent},
    geometry::{Point, Rect, Size},
    id::ViewId,
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
        let mut focused_id: Option<ViewId> = None;
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
                            let mut clicked_focusable_id: Option<ViewId> = None;
                            let handled =
                                dispatch_click(root, click_target, &mut clicked_focusable_id);

                            if let Some(target_id) = clicked_focusable_id {
                                notify_blur_except(root, target_id);
                                notify_focus(root, target_id);
                                focused_id = Some(target_id);
                            } else {
                                notify_blur_all(root);
                                focused_id = None;
                            }

                            if handled || clicked_focusable_id.is_some() {
                                win.request_redraw();
                            }
                        }
                    }
                    win.request_redraw();
                }
            }
            Event::Key(key_event) => {
                if let Some(ref root) = last_root_node {
                    let handled = dispatch_key(root, focused_id, &key_event);
                    if handled {
                        win.request_redraw();
                    }
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
fn dispatch_click(node: &ViewNode, pos: Point, focused_id: &mut Option<ViewId>) -> bool {
    if !node.bounds.contains(pos) {
        return false;
    }

    // Children in reverse order (topmost z-order first)
    for child in node.children.iter().rev() {
        if dispatch_click(child, pos, focused_id) {
            return true;
        }
    }

    if node.is_focusable || node.on_key.is_some() {
        *focused_id = Some(node.id);
    }

    if let Some(ref cb) = node.on_click {
        cb.emit(());
        return true;
    }

    node.is_focusable
}

/// Dispatches a keyboard event to the focused node, falling back to any node with focus.
fn dispatch_key(node: &ViewNode, focused_id: Option<ViewId>, event: &KeyEvent) -> bool {
    let matches_focus = node.is_focused || (focused_id.is_some() && focused_id == Some(node.id));

    if matches_focus {
        if let Some(ref cb) = node.on_key {
            cb.emit(event.clone());
            return true;
        }
    }

    for child in &node.children {
        if dispatch_key(child, focused_id, event) {
            return true;
        }
    }

    false
}

/// Blurs all currently focused nodes in the view tree.
fn notify_blur_all(node: &ViewNode) {
    if node.is_focused {
        if let Some(ref cb) = node.on_focus_change {
            cb.emit(false);
        }
    }
    for child in &node.children {
        notify_blur_all(child);
    }
}

/// Blurs all currently focused nodes except the newly focused target.
fn notify_blur_except(node: &ViewNode, except_id: ViewId) {
    if node.is_focused && node.id != except_id {
        if let Some(ref cb) = node.on_focus_change {
            cb.emit(false);
        }
    }
    for child in &node.children {
        notify_blur_except(child, except_id);
    }
}

/// Delivers a focus event to the target node.
fn notify_focus(node: &ViewNode, target_id: ViewId) -> bool {
    if node.id == target_id {
        if let Some(ref cb) = node.on_focus_change {
            cb.emit(true);
        }
        return true;
    }
    for child in &node.children {
        if notify_focus(child, target_id) {
            return true;
        }
    }
    false
}

/// Recursively collects all focusable node IDs in tree traversal order.
#[allow(dead_code)]
fn collect_focusables(node: &ViewNode, out: &mut Vec<ViewId>) {
    if node.is_focusable || node.on_key.is_some() {
        out.push(node.id);
    }
    for child in &node.children {
        collect_focusables(child, out);
    }
}

/// Computes the preferred (intrinsic) size for a ViewNode and its children.
fn compute_preferred_size(node: &ViewNode) -> Size {
    if let Some(fixed) = node.fixed_size {
        return fixed;
    }
    match &node.kind {
        NodeKind::Text {
            text, font_size, ..
        } => {
            // Use integer pixel scale so glyphs are always crisp whole-pixel rectangles
            let px = ((font_size / 9.0).max(1.0)).round().max(2.0);
            let char_w = px * 5.0;
            let char_gap = px * 1.5;
            // Count visible characters (not bytes) for accurate width
            let char_count = text.chars().count();
            let total_w = if char_count == 0 {
                6.0
            } else {
                char_count as f32 * (char_w + char_gap) - char_gap
            };
            let total_h = px * 7.0;
            Size::new(total_w + 12.0, total_h + 8.0)
        }
        NodeKind::Container { padding, .. } => {
            if node.is_focusable {
                let fixed_h = 2.0 * 7.0 + 8.0 + padding.vertical();
                return Size::new(260.0, fixed_h);
            }
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
        NodeKind::ZStack => {
            let mut max_w: f32 = 0.0;
            let mut max_h: f32 = 0.0;
            for child in &node.children {
                let s = compute_preferred_size(child);
                max_w = max_w.max(s.width);
                max_h = max_h.max(s.height);
            }
            Size::new(max_w, max_h)
        }
        NodeKind::Grid {
            columns,
            spacing,
            row_spacing,
        } => {
            let count = node.children.len();
            if count == 0 {
                return Size::ZERO;
            }
            let cols = *columns.max(&1);
            let rows = (count + cols - 1) / cols;
            let child_sizes: Vec<Size> = node.children.iter().map(compute_preferred_size).collect();
            let max_cell_w = child_sizes.iter().map(|s| s.width).fold(0.0f32, f32::max);
            let max_cell_h = child_sizes.iter().map(|s| s.height).fold(0.0f32, f32::max);
            let total_w = max_cell_w * cols as f32 + *spacing * (cols - 1) as f32;
            let total_h = max_cell_h * rows as f32 + *row_spacing * (rows.saturating_sub(1)) as f32;
            Size::new(total_w, total_h)
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
                if child.is_text_left_aligned {
                    layout_node_recursive(child, inner_rect);
                } else {
                    let pref = compute_preferred_size(child);
                    let w = if child.flex_grow || matches!(&child.kind, NodeKind::Flex { .. }) {
                        inner_rect.size.width
                    } else {
                        pref.width.min(inner_rect.size.width)
                    };
                    let h = pref.height.min(inner_rect.size.height);
                    let x = if child.flex_grow || matches!(&child.kind, NodeKind::Flex { .. }) {
                        inner_rect.origin.x
                    } else {
                        inner_rect.origin.x + (inner_rect.size.width - w) * 0.5
                    };
                    let y = inner_rect.origin.y + (inner_rect.size.height - h).max(0.0) * 0.5;
                    layout_node_recursive(child, Rect::new(x, y, w, h));
                }
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
                let start_y = bounds.origin.y;
                let mut current_y = start_y;

                for (i, child) in node.children.iter_mut().enumerate() {
                    let pref = child_sizes[i];
                    // Rows, stacks, containers, and flex_grow elements stretch across bounds.size.width
                    let w = match &child.kind {
                        NodeKind::Flex { .. } | NodeKind::Container { .. } => bounds.size.width,
                        _ => {
                            if child.flex_grow {
                                bounds.size.width
                            } else {
                                pref.width.min(bounds.size.width)
                            }
                        }
                    };
                    let h = pref.height;
                    let x = if w >= bounds.size.width {
                        bounds.origin.x
                    } else {
                        bounds.origin.x + (bounds.size.width - w) * 0.5
                    };
                    layout_node_recursive(child, Rect::new(x, current_y, w, h));
                    current_y += h + *spacing;
                }
            } else {
                // HStack: give all remaining space to flex_grow children
                let child_sizes: Vec<Size> =
                    node.children.iter().map(compute_preferred_size).collect();

                let grow_count = node.children.iter().filter(|c| c.flex_grow).count();
                let fixed_w: f32 = node
                    .children
                    .iter()
                    .zip(child_sizes.iter())
                    .filter(|(c, _)| !c.flex_grow)
                    .map(|(_, s)| s.width)
                    .sum::<f32>()
                    + *spacing * (node.children.len().saturating_sub(1)) as f32;
                let leftover = (bounds.size.width - fixed_w).max(0.0);
                let grow_w = if grow_count > 0 {
                    leftover / grow_count as f32
                } else {
                    0.0
                };

                let total_content_w: f32 = child_sizes
                    .iter()
                    .zip(node.children.iter())
                    .map(|(s, c)| if c.flex_grow { grow_w } else { s.width })
                    .sum::<f32>()
                    + *spacing * (node.children.len().saturating_sub(1)) as f32;
                let start_x = if grow_count > 0 {
                    bounds.origin.x
                } else {
                    bounds.origin.x + (bounds.size.width - total_content_w).max(0.0) * 0.5
                };
                let mut current_x = start_x;

                for (i, child) in node.children.iter_mut().enumerate() {
                    let w = if child.flex_grow {
                        grow_w
                    } else {
                        child_sizes[i].width
                    };
                    let h = child_sizes[i].height.min(bounds.size.height);
                    let y = bounds.origin.y + (bounds.size.height - h) * 0.5;
                    layout_node_recursive(child, Rect::new(current_x, y, w, h));
                    current_x += w + *spacing;
                }
            }
        }
        NodeKind::Grid {
            columns,
            spacing,
            row_spacing,
        } => {
            let count = node.children.len();
            if count == 0 {
                return;
            }
            let cols = *columns.max(&1);
            let total_h_spacing = *spacing * (cols - 1) as f32;
            let col_w = ((bounds.size.width - total_h_spacing) / cols as f32).max(1.0);
            for (i, child) in node.children.iter_mut().enumerate() {
                let col = i % cols;
                let row = i / cols;
                let pref = compute_preferred_size(child);
                // Default to square cell aspect ratio if preferred height is small
                let cell_h = if pref.height > 10.0 {
                    pref.height
                } else {
                    col_w
                };
                let x = bounds.origin.x + col as f32 * (col_w + *spacing);
                let y = bounds.origin.y + row as f32 * (cell_h + *row_spacing);
                layout_node_recursive(child, Rect::new(x, y, col_w, cell_h));
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

            // Focused input field — bright sky-blue ring, no hover-brighten
            if node.is_focused {
                border_color = Color::hex(0x38BDF8); // Sky 400
                border_width = 2.0;
            } else if is_hovered {
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
            draw_text_bitmap(
                renderer,
                text,
                node.bounds,
                *font_size,
                *color,
                node.is_text_left_aligned,
            );
        }
        _ => {}
    }

    for child in &node.children {
        render_node(renderer, child, cursor);
    }
}

/// Renders crisp, readable text glyphs using a built-in 5x7 ASCII vector font.
fn draw_text_bitmap(
    renderer: &mut Renderer,
    text: &str,
    bounds: Rect,
    font_size: f32,
    color: Color,
    align_left: bool,
) {
    if text.is_empty() {
        return;
    }

    // Integer pixel scale — prevents sub-pixel fractional squares that look blurry/uneven
    let px = ((font_size / 9.0).max(1.0)).round().max(2.0);
    let char_w = px * 5.0;
    let char_gap = px * 1.5;
    let char_h = px * 7.0;
    let chars: Vec<char> = text.chars().collect();
    let total_w = if chars.is_empty() {
        0.0
    } else {
        chars.len() as f32 * (char_w + char_gap) - char_gap
    };

    let min_x = (bounds.origin.x + 4.0).round();
    let max_x = (bounds.origin.x + bounds.size.width - 4.0).round();

    let start_x = if align_left {
        let avail_w = (bounds.size.width - 16.0).max(0.0);
        if total_w > avail_w {
            (bounds.origin.x + 8.0 - (total_w - avail_w)).round()
        } else {
            (bounds.origin.x + 8.0).round()
        }
    } else {
        (bounds.origin.x + (bounds.size.width - total_w).max(0.0) * 0.5).round()
    };
    // Snap start to integer coords so glyphs land on exact pixel boundaries
    let start_x = start_x.round();
    let start_y = (bounds.origin.y + (bounds.size.height - char_h).max(0.0) * 0.5).round();

    let mut cur_x = start_x;
    for ch in chars {
        if cur_x + char_w >= min_x && cur_x <= max_x {
            let glyph = get_glyph_5x7(ch);
            for col in 0..5usize {
                let rx = (cur_x + col as f32 * px).round();
                if rx >= min_x && rx + px <= max_x {
                    let col_byte = glyph[col];
                    for row in 0..7usize {
                        if (col_byte & (1 << row)) != 0 {
                            let ry = (start_y + row as f32 * px).round();
                            renderer.draw_rect(
                                Rect::new(rx, ry, px, px),
                                color,
                                0.0,
                                0.0,
                                Color::TRANSPARENT,
                            );
                        }
                    }
                }
            }
        }
        cur_x += char_w + char_gap;
    }
}

/// Complete 5x7 standard ASCII font table (characters 32..=126).
const FONT_5X7: [[u8; 5]; 95] = [
    [0x00, 0x00, 0x00, 0x00, 0x00], // ' ' (32)
    [0x00, 0x00, 0x5F, 0x00, 0x00], // '!' (33)
    [0x00, 0x07, 0x00, 0x07, 0x00], // '"' (34)
    [0x14, 0x7F, 0x14, 0x7F, 0x14], // '#' (35)
    [0x24, 0x2A, 0x7F, 0x2A, 0x12], // '$' (36)
    [0x23, 0x13, 0x08, 0x64, 0x62], // '%' (37)
    [0x36, 0x49, 0x55, 0x22, 0x50], // '&' (38)
    [0x00, 0x05, 0x03, 0x00, 0x00], // '\'' (39)
    [0x00, 0x1C, 0x22, 0x41, 0x00], // '(' (40)
    [0x00, 0x41, 0x22, 0x1C, 0x00], // ')' (41)
    [0x14, 0x08, 0x3E, 0x08, 0x14], // '*' (42)
    [0x08, 0x08, 0x3E, 0x08, 0x08], // '+' (43)
    [0x00, 0x50, 0x30, 0x00, 0x00], // ',' (44)
    [0x08, 0x08, 0x08, 0x08, 0x08], // '-' (45)
    [0x00, 0x60, 0x60, 0x00, 0x00], // '.' (46)
    [0x20, 0x10, 0x08, 0x04, 0x02], // '/' (47)
    [0x3E, 0x51, 0x49, 0x45, 0x3E], // '0' (48)
    [0x00, 0x42, 0x7F, 0x40, 0x00], // '1' (49)
    [0x42, 0x61, 0x51, 0x49, 0x46], // '2' (50)
    [0x21, 0x41, 0x45, 0x4B, 0x31], // '3' (51)
    [0x18, 0x14, 0x12, 0x7F, 0x10], // '4' (52)
    [0x27, 0x45, 0x45, 0x45, 0x39], // '5' (53)
    [0x3C, 0x4A, 0x49, 0x49, 0x30], // '6' (54)
    [0x01, 0x71, 0x09, 0x05, 0x03], // '7' (55)
    [0x36, 0x49, 0x49, 0x49, 0x36], // '8' (56)
    [0x06, 0x49, 0x49, 0x29, 0x1E], // '9' (57)
    [0x00, 0x36, 0x36, 0x00, 0x00], // ':' (58)
    [0x00, 0x56, 0x36, 0x00, 0x00], // ';' (59)
    [0x08, 0x14, 0x22, 0x41, 0x00], // '<' (60)
    [0x14, 0x14, 0x14, 0x14, 0x14], // '=' (61)
    [0x00, 0x41, 0x22, 0x14, 0x08], // '>' (62)
    [0x02, 0x01, 0x51, 0x09, 0x06], // '?' (63)
    [0x32, 0x49, 0x79, 0x41, 0x3E], // '@' (64)
    [0x7E, 0x11, 0x11, 0x11, 0x7E], // 'A' (65)
    [0x7F, 0x49, 0x49, 0x49, 0x36], // 'B' (66)
    [0x3E, 0x41, 0x41, 0x41, 0x22], // 'C' (67)
    [0x7F, 0x41, 0x41, 0x22, 0x1C], // 'D' (68)
    [0x7F, 0x49, 0x49, 0x49, 0x41], // 'E' (69)
    [0x7F, 0x09, 0x09, 0x09, 0x01], // 'F' (70)
    [0x3E, 0x41, 0x49, 0x49, 0x7A], // 'G' (71)
    [0x7F, 0x08, 0x08, 0x08, 0x7F], // 'H' (72)
    [0x00, 0x41, 0x7F, 0x41, 0x00], // 'I' (73)
    [0x20, 0x40, 0x41, 0x3F, 0x01], // 'J' (74)
    [0x7F, 0x08, 0x14, 0x22, 0x41], // 'K' (75)
    [0x7F, 0x40, 0x40, 0x40, 0x40], // 'L' (76)
    [0x7F, 0x02, 0x0C, 0x02, 0x7F], // 'M' (77)
    [0x7F, 0x04, 0x08, 0x10, 0x7F], // 'N' (78)
    [0x3E, 0x41, 0x41, 0x41, 0x3E], // 'O' (79)
    [0x7F, 0x09, 0x09, 0x09, 0x06], // 'P' (80)
    [0x3E, 0x41, 0x51, 0x21, 0x5E], // 'Q' (81)
    [0x7F, 0x09, 0x19, 0x29, 0x46], // 'R' (82)
    [0x46, 0x49, 0x49, 0x49, 0x31], // 'S' (83)
    [0x01, 0x01, 0x7F, 0x01, 0x01], // 'T' (84)
    [0x3F, 0x40, 0x40, 0x40, 0x3F], // 'U' (85)
    [0x1F, 0x20, 0x40, 0x20, 0x1F], // 'V' (86)
    [0x7F, 0x20, 0x18, 0x20, 0x7F], // 'W' (87)
    [0x63, 0x14, 0x08, 0x14, 0x63], // 'X' (88)
    [0x07, 0x08, 0x70, 0x08, 0x07], // 'Y' (89)
    [0x61, 0x51, 0x49, 0x45, 0x43], // 'Z' (90)
    [0x00, 0x7F, 0x41, 0x41, 0x00], // '[' (91)
    [0x02, 0x04, 0x08, 0x10, 0x20], // '\\' (92)
    [0x00, 0x41, 0x41, 0x7F, 0x00], // ']' (93)
    [0x04, 0x02, 0x01, 0x02, 0x04], // '^' (94)
    [0x40, 0x40, 0x40, 0x40, 0x40], // '_' (95)
    [0x00, 0x01, 0x02, 0x04, 0x00], // '`' (96)
    [0x20, 0x54, 0x54, 0x54, 0x78], // 'a' (97)
    [0x7F, 0x48, 0x44, 0x44, 0x38], // 'b' (98)
    [0x38, 0x44, 0x44, 0x44, 0x20], // 'c' (99)
    [0x38, 0x44, 0x44, 0x48, 0x7F], // 'd' (100)
    [0x38, 0x54, 0x54, 0x54, 0x18], // 'e' (101)
    [0x08, 0x7E, 0x09, 0x01, 0x02], // 'f' (102)
    [0x0C, 0x52, 0x52, 0x52, 0x3E], // 'g' (103)
    [0x7F, 0x08, 0x04, 0x04, 0x78], // 'h' (104)
    [0x00, 0x44, 0x7D, 0x40, 0x00], // 'i' (105)
    [0x20, 0x40, 0x44, 0x3D, 0x00], // 'j' (106)
    [0x7F, 0x10, 0x28, 0x44, 0x00], // 'k' (107)
    [0x00, 0x41, 0x7F, 0x40, 0x00], // 'l' (108)
    [0x7C, 0x04, 0x18, 0x04, 0x78], // 'm' (109)
    [0x7C, 0x08, 0x04, 0x04, 0x78], // 'n' (110)
    [0x38, 0x44, 0x44, 0x44, 0x38], // 'o' (111)
    [0x7C, 0x14, 0x14, 0x14, 0x08], // 'p' (112)
    [0x08, 0x14, 0x14, 0x18, 0x7C], // 'q' (113)
    [0x7C, 0x08, 0x04, 0x04, 0x08], // 'r' (114)
    [0x48, 0x54, 0x54, 0x54, 0x20], // 's' (115)
    [0x04, 0x3F, 0x44, 0x40, 0x20], // 't' (116)
    [0x3C, 0x40, 0x40, 0x20, 0x7C], // 'u' (117)
    [0x1C, 0x20, 0x40, 0x20, 0x1C], // 'v' (118)
    [0x3C, 0x40, 0x30, 0x40, 0x3C], // 'w' (119)
    [0x44, 0x28, 0x10, 0x28, 0x44], // 'x' (120)
    [0x0C, 0x50, 0x50, 0x50, 0x3C], // 'y' (121)
    [0x44, 0x64, 0x54, 0x4C, 0x44], // 'z' (122)
    [0x00, 0x08, 0x36, 0x41, 0x00], // '{' (123)
    [0x00, 0x00, 0x7F, 0x00, 0x00], // '|' (124)
    [0x00, 0x41, 0x36, 0x08, 0x00], // '}' (125)
    [0x08, 0x04, 0x08, 0x10, 0x08], // '~' (126)
];

/// Returns the 5x7 column bytes for any ASCII or common unicode character.
fn get_glyph_5x7(c: char) -> [u8; 5] {
    let code = c as u32;
    if (32..=126).contains(&code) {
        FONT_5X7[(code - 32) as usize]
    } else {
        match c {
            '✓' => [0x00, 0x20, 0x40, 0x38, 0x04],
            '✕' | '×' => [0x63, 0x14, 0x08, 0x14, 0x63],
            '•' => [0x00, 0x1C, 0x1C, 0x1C, 0x00],
            _ => [0x7F, 0x41, 0x41, 0x41, 0x7F], // Hollow box fallback
        }
    }
}
