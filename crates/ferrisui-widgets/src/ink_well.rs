use std::time::Instant;

use ferrisui_core::{
    build_context::BuildContext,
    callback::Callback,
    color::Color,
    geometry::{EdgeInsets, Point},
    state::Signal,
    view::View,
    view_node::{NodeKind, ViewNode},
};

#[derive(Debug, Clone, Copy)]
pub struct RippleState {
    pub origin: Point,
    pub progress: f32,
    pub start_time: Option<Instant>,
    pub is_animating: bool,
}

impl Default for RippleState {
    fn default() -> Self {
        Self {
            origin: Point::new(0.0, 0.0),
            progress: 0.0,
            start_time: None,
            is_animating: false,
        }
    }
}

pub struct InkWell<V: View> {
    child: V,
    on_click: Option<Callback<()>>,
    ripple_color: Color,
    hover_color: Color,
    background: Color,
    corner_radius: f32,
    padding: EdgeInsets,
    ripple_state: Signal<RippleState>,
    is_hovered: Signal<bool>,
    duration_ms: u64,
}

impl<V: View> InkWell<V> {
    pub fn new(child: V) -> Self {
        Self {
            child,
            on_click: None,
            ripple_color: Color::rgba(1.0, 1.0, 1.0, 0.20),
            hover_color: Color::rgba(1.0, 1.0, 1.0, 0.08),
            background: Color::TRANSPARENT,
            corner_radius: 8.0,
            padding: EdgeInsets::all(8.0),
            ripple_state: Signal::new(RippleState::default()),
            is_hovered: Signal::new(false),
            duration_ms: 350,
        }
    }

    pub fn on_click<F>(mut self, handler: F) -> Self
    where
        F: Fn() + Send + Sync + 'static,
    {
        let ripple_signal = self.ripple_state.clone();
        self.on_click = Some(Callback::new(move |_| {
            ripple_signal.set(RippleState {
                origin: Point::new(0.0, 0.0),
                progress: 0.1,
                start_time: Some(Instant::now()),
                is_animating: true,
            });
            handler();
        }));
        self
    }
    /// Custom color for the expanding ink splash ripple.
    pub fn ripple_color(mut self, color: Color) -> Self {
        self.ripple_color = color;
        self
    }
    /// Highlight overlay color when hovered with cursor.
    pub fn hover_color(mut self, color: Color) -> Self {
        self.hover_color = color;
        self
    }
    /// Background color of the underlying surface.
    pub fn background(mut self, color: Color) -> Self {
        self.background = color;
        self
    }
    /// Corner radius for clipping the surface and ripple edges.
    pub fn corner_radius(mut self, radius: f32) -> Self {
        self.corner_radius = radius;
        self
    }

    /// Internal padding around the child content.
    pub fn padding(mut self, padding: EdgeInsets) -> Self {
        self.padding = padding;
        self
    }
    /// Ripple animation duration in milliseconds (default: 350ms).
    pub fn duration_ms(mut self, duration: u64) -> Self {
        self.duration_ms = duration;
        self
    }
    /// Triggers a ripple explicitly from a specific click coordinate `pos`.
    pub fn trigger_ripple_at(&self, pos: Point) {
        self.ripple_state.set(RippleState {
            origin: pos,
            progress: 0.05,
            start_time: Some(Instant::now()),
            is_animating: true,
        });
    }
}

impl<V: View> View for InkWell<V> {
    fn build(&self, ctx: &mut BuildContext) -> ViewNode {
        let ripple = self.ripple_state.get();
        let hovered = self.is_hovered.get();

        let effective_bg = if hovered {
            Color::rgba(
                self.background.r * 0.9 + self.hover_color.r * 0.1,
                self.background.g * 0.9 + self.hover_color.g * 0.1,
                self.background.b * 0.9 + self.hover_color.b * 0.1,
                (self.background.a + self.hover_color.a).min(1.0),
            )
        } else {
            self.background
        };

        let mut outer_node = ViewNode::new(NodeKind::Container {
            background: effective_bg,
            corner_radius: self.corner_radius,
            padding: self.padding,
        });

        outer_node.on_click = self.on_click.clone();

        let child_node = self.child.build(ctx);
        outer_node = outer_node.with_child(child_node);

        if ripple.is_animating {
            if let Some(start) = ripple.start_time {
                let elapsed = start.elapsed().as_millis() as f32;
                let current_progress = (elapsed / self.duration_ms as f32).min(1.0);

                let alpha_fade = (1.0 - current_progress) * self.ripple_color.a;

                let animated_ripple_color = Color::rgba(
                    self.ripple_color.r,
                    self.ripple_color.g,
                    self.ripple_color.b,
                    alpha_fade,
                );

                let ripple_overlay = ViewNode::new(NodeKind::Container {
                    background: animated_ripple_color,
                    corner_radius: self.corner_radius,
                    padding: EdgeInsets::ZERO,
                });

                outer_node = outer_node.with_child(ripple_overlay);

                if current_progress >= 1.0 {
                    self.ripple_state.set(RippleState {
                        is_animating: false,
                        progress: 1.0,
                        start_time: None,
                        ..ripple
                    })
                }
            }
        }
        outer_node
    }
}
