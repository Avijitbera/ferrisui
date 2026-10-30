use bytemuck::{Pod, Zeroable};
use ferrisui_core::color::Color;
use ferrisui_core::geometry::{Rect, Size};

#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable)]

pub struct RectInstance {
    pub rect_pos: [f32; 2],
    pub rect_size: [f32; 2],
    pub color: [f32; 4],
    pub border_color: [f32; 4],
    pub border_width: f32,
    pub corner_radius: f32,
    pub _padding: [f32; 2],
}

impl RectInstance {
    pub fn new(
        rect: Rect,
        color: Color,
        corner_radius: f32,
        border_width: f32,
        border_color: Color,
    ) -> Self {
        Self {
            rect_pos: [rect.origin.x, rect.origin.y],
            rect_size: [rect.size.width, rect.size.height],
            color: color.to_rgba_f32(),
            border_color: border_color.to_rgba_f32(),
            border_width,
            corner_radius,
            _padding: [0.0, 0.0],
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
pub struct WindowUniforms {
    pub screen_size: [f32; 2],
    pub _padding: [f32; 2],
}

impl WindowUniforms {
    pub fn new(size: Size) -> Self {
        Self {
            screen_size: [size.width, size.height],
            _padding: [0.0, 0.0],
        }
    }
}
