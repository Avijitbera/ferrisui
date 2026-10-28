use crate::window::{Window, WindowConfig};
use ferrisui_core::event::{ElementState, Event, KeyEvent, Modifiers, MouseButton, MouseEvent};
use ferrisui_core::geometry::Point;
use winit::event::{
    ElementState as WinitElementState, Event as WinitEvent, MouseButton as WinitMouseButton,
    WindowEvent as WinitWindowEvent,
};
use winit::event_loop::{ControlFlow, EventLoop};

pub struct PlatformApp {
    event_loop: Option<EventLoop<()>>,
    window: Option<Window>,
}

impl PlatformApp {
    pub fn new(config: WindowConfig) -> Result<Self, Box<dyn std::error::Error>> {
        let event_loop = EventLoop::new()?;
        let window = Window::new(&event_loop, config)?;
        Ok(Self {
            event_loop: Some(event_loop),
            window: Some(window),
        })
    }

    pub fn window(&self) -> &Window {
        self.window.as_ref().expect("Window initialized")
    }

    pub fn run<F>(mut self, mut handler: F) -> Result<(), Box<dyn std::error::Error>>
    where
        F: FnMut(&Window, Event, &mut bool) + 'static,
    {
        let event_loop = self.event_loop.take().expect("EventLoop available");
        let window = self.window.take().expect("Window available");

        let mut mouse_position = Point::ZERO;
        let mut modifiers = Modifiers::default();

        event_loop.run(move |event, window_target| {
            window_target.set_control_flow(ControlFlow::Wait);
            let mut should_exit = false;

            match event {
                WinitEvent::WindowEvent {
                    event: win_event, ..
                } => match win_event {
                    WinitWindowEvent::CloseRequested => {
                        should_exit = true;
                    }
                    WinitWindowEvent::Resized(_) => {
                        let scale = window.scale_factor();
                        handler(&window, Event::ScaleFactorChanged(scale), &mut should_exit);
                        window.request_redraw();
                    }
                    WinitWindowEvent::ScaleFactorChanged { scale_factor, .. } => {
                        handler(
                            &window,
                            Event::ScaleFactorChanged(scale_factor as f32),
                            &mut should_exit,
                        );
                        window.request_redraw();
                    }
                    WinitWindowEvent::CursorMoved { position, .. } => {
                        let scale = window.scale_factor();
                        mouse_position =
                            Point::new((position.x as f32) / scale, (position.y as f32) / scale);
                        handler(
                            &window,
                            Event::Mouse(MouseEvent::Moved {
                                position: mouse_position,
                            }),
                            &mut should_exit,
                        );
                    }

                    WinitWindowEvent::MouseInput { state, button, .. } => {
                        let btn = match button {
                            WinitMouseButton::Left => MouseButton::Left,
                            WinitMouseButton::Right => MouseButton::Right,
                            WinitMouseButton::Middle => MouseButton::Middle,
                            WinitMouseButton::Other(id) => MouseButton::Other(id),
                            _ => MouseButton::Left,
                        };
                        let st = match state {
                            WinitElementState::Pressed => ElementState::Pressed,
                            WinitElementState::Released => ElementState::Released,
                        };

                        handler(
                            &window,
                            Event::Mouse(MouseEvent::Button {
                                position: mouse_position,
                                button: btn,
                                state: st,
                            }),
                            &mut should_exit,
                        );
                    }

                    WinitWindowEvent::MouseWheel { delta, .. } => {
                        let (dx, dy) = match delta {
                            winit::event::MouseScrollDelta::LineDelta(x, y) => (x * 20.0, y * 20.0),
                            winit::event::MouseScrollDelta::PixelDelta(pos) => {
                                (pos.x as f32, pos.y as f32)
                            }
                        };

                        handler(
                            &window,
                            Event::Mouse(MouseEvent::Wheel {
                                delta_x: dx,
                                delta_y: dy,
                            }),
                            &mut should_exit,
                        );
                    }

                    WinitWindowEvent::KeyboardInput {
                        event: key_event, ..
                    } => {
                        let st = match key_event.state {
                            WinitElementState::Pressed => ElementState::Pressed,
                            WinitElementState::Released => ElementState::Released,
                        };

                        handler(
                            &window,
                            Event::Key(KeyEvent::Key {
                                key_code: 0,
                                state: st,
                                modifiers,
                            }),
                            &mut should_exit,
                        );
                    }

                    WinitWindowEvent::ModifiersChanged(new_mods) => {
                        modifiers.shift = new_mods.state().shift_key();
                        modifiers.ctrl = new_mods.state().control_key();
                        modifiers.alt = new_mods.state().alt_key();
                        modifiers.meta = new_mods.state().super_key();
                    }
                    WinitWindowEvent::Focused(focused) => {
                        handler(&window, Event::WindowFocused(focused), &mut should_exit);
                    }
                    WinitWindowEvent::RedrawRequested => {}
                    _ => {}
                },
                _ => {}
            }
            if should_exit {
                window_target.exit();
            }
        })?;
        Ok(())
    }
}
