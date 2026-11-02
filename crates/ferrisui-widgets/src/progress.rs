use ferrisui_core::build_context::BuildContext;
use ferrisui_core::color::Color;
use ferrisui_core::geometry::EdgeInsets;
use ferrisui_core::view::View;
use ferrisui_core::view_node::{NodeKind, ViewNode};
pub struct Progress {
    pub progress: f32, // 0.0 .. 1.0
    track_color: Color,
    fill_color: Color,
    height: f32,
    corner_radius: f32,
}

impl Progress {
    pub fn new(progress: f32) -> Self {
        Self {
            progress: progress.clamp(0.0, 1.0),
            track_color: Color::rgba(1.0, 1.0, 1.0, 0.10),
            fill_color: Color::hex(0x6366F1),
            height: 8.0,
            corner_radius: 4.0,
        }
    }

    pub fn color(mut self, color: Color) -> Self {
        self.fill_color = color;
        self
    }
    pub fn track_color(mut self, color: Color) -> Self {
        self.track_color = color;
        self
    }
    pub fn height(mut self, height: f32) -> Self {
        self.height = height;
        self
    }
}

impl View for Progress {
    fn build(&self, _ctx: &mut BuildContext) -> ViewNode {
        // Outer track container
        ViewNode::new(NodeKind::Container {
            background: self.track_color,
            corner_radius: self.corner_radius,
            padding: EdgeInsets::ZERO,
        })
        .with_child(
            // Inner filled progress container
            ViewNode::new(NodeKind::Container {
                background: self.fill_color,
                corner_radius: self.corner_radius,
                padding: EdgeInsets::ZERO,
            }),
        )
    }
}
