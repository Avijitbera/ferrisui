use ferrisui_core::color::Color;
use ferrisui_core::event::{Event, MouseEvent};
use ferrisui_core::geometry::{Point, Rect, Size};
use ferrisui_platform::event_loop::PlatformApp;
use ferrisui_platform::window::WindowConfig;
use ferrisui_render::{GpuContext, Renderer};
use std::sync::Arc;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = WindowConfig {
        title: "FerrisUI - GPU Render Test".to_string(),
        size: Size::new(900.0, 600.0),
        min_size: Some(Size::new(400.0, 300.0)),
        max_size: None,
        resizable: true,
        decorations: true,
        transparent: false,
    };

    let app = PlatformApp::new(config)?;
    let window = Arc::new(app.window().clone());
    let (phys_w, phys_h) = window.physical_size();

    let gpu_context = pollster::block_on(GpuContext::new(window.clone(), phys_w, phys_h))?;
    let mut renderer = Renderer::new(gpu_context);

    // Set uniform screen size to logical size matching cursor coordinates
    let logical_size = window.inner_size();
    renderer.resize_logical(logical_size.width, logical_size.height);

    let mut cursor_pos = Point::new(-1000.0, -1000.0);

    println!("Window opened successfully! Move mouse to test interactive GPU rendering.");

    app.run(move |win, event, _should_exit| match event {
        Event::Mouse(MouseEvent::Moved { position }) => {
            cursor_pos = position;
            win.request_redraw();
        }
        Event::Resized { width, height } => {
            renderer.resize(width, height);
            let logical = win.inner_size();
            renderer.resize_logical(logical.width, logical.height);
            win.request_redraw();
        }
        Event::RedrawRequested => {
            let win_size = win.inner_size();
            let w = win_size.width;
            let h = win_size.height;
            let bg_color = Color::hex(0x0F172A);

            let card_rect = Rect::new(w * 0.5 - 200.0, h * 0.5 - 150.0, 400.0, 300.0);

            let is_hovered = card_rect.contains(cursor_pos);
            let card_color = if is_hovered {
                Color::rgba(0.12, 0.16, 0.23, 0.95)
            } else {
                Color::rgba(0.12, 0.16, 0.23, 0.70)
            };

            let border_color = if is_hovered {
                Color::hex(0x38BDF8)
            } else {
                Color::rgba(1.0, 1.0, 1.0, 0.15)
            };

            renderer.draw_rect(card_rect, card_color, 24.0, 2.0, border_color);

            let btn_rect = Rect::new(w * 0.5 - 90.0, h * 0.5 + 40.0, 180.0, 46.0);

            let btn_hovered = btn_rect.contains(cursor_pos);
            let btn_color = if btn_hovered {
                Color::hex(0xF97316)
            } else {
                Color::hex(0xEA580C)
            };

            renderer.draw_rect(btn_rect, btn_color, 12.0, 0.0, Color::TRANSPARENT);

            let _ = renderer.render_frame(bg_color);
        }

        _ => {}
    })?;
    Ok(())
}
