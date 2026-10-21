use std::fmt::{self, write};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TemplateKind {
    Counter,
    Minimal,
    Media,
}

impl fmt::Display for TemplateKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TemplateKind::Counter => write!(f, "Counter (State, Button, and Layout)"),
            TemplateKind::Minimal => write!(f, "Minimal (Basic App Structure)"),
            TemplateKind::Media => write!(f, "Media Player (Video surface & player HUD)"),
        }
    }
}

impl TemplateKind {
    pub fn parse_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "counter" => Some(TemplateKind::Counter),
            "minimal" => Some(TemplateKind::Minimal),
            "media" => Some(TemplateKind::Media),
            _ => None,
        }
    }

    pub fn generate_main_rs(&self, app_name: &str) -> String {
        match self {
            TemplateKind::Counter => format!(
                r#"//! {app_name} — Built with FerrisUI
use ferrisui::prelude::*;
fn main() {{
    FerrisUI::app()
        .window_title("{app_name}")
        .window_size(800.0, 600.0)
        .run(|ctx| {{
            // Reactive state signal
            let count = ctx.use_signal(0);
            VStack::new()
                .spacing(20.0)
                .alignment(Alignment::Center)
                .child(
                    Text::new("🚀 Welcome to FerrisUI!")
                        .font_size(28.0)
                        .bold()
                )
                .child(
                    Text::new(format!("Current Count: {{}}", count.get()))
                        .font_size(20.0)
                )
                .child(
                    HStack::new()
                        .spacing(12.0)
                        .child(
                            Button::new("➖ Decrement")
                                .on_click(move || count.update(|n| n - 1))
                        )
                        .child(
                            Button::new("➕ Increment")
                                .on_click(move || count.update(|n| n + 1))
                        )
                )
        }});
}}
"#
            ),
            TemplateKind::Minimal => format!(
                r#"//! {app_name} — Minimal FerrisUI App
use ferrisui::prelude::*;
fn main() {{
    FerrisUI::app()
        .window_title("{app_name}")
        .window_size(640.0, 480.0)
        .run(|_ctx| {{
            Center::new(
                Text::new("Hello, FerrisUI!")
                    .font_size(24.0)
            )
        }});
}}
"#
            ),
            TemplateKind::Media => format!(
                r#"//! {app_name} — Media Player Template
use ferrisui::prelude::*;
fn main() {{
    FerrisUI::app()
        .window_title("{app_name} - Media Player")
        .window_size(1024.0, 640.0)
        .run(|ctx| {{
            let is_playing = ctx.use_signal(false);
            ZStack::new()
                .child(
                    Container::new()
                        .background(Color::BLACK)
                        .child(Text::new("Video Viewport (Hardware Decoded)").color(Color::WHITE))
                )
                .child(
                    VStack::new()
                        .alignment(Alignment::BottomLeading)
                        .padding(16.0)
                        .child(
                            Button::new(if is_playing.get() {{ "⏸ Pause" }} else {{ "▶ Play" }})
                                .on_click(move || is_playing.update(|p| !p))
                        )
                )
        }});
}}
"#
            ),
        }
    }

    pub fn generate_cargo_toml(&self, app_name: &str) -> String {
        format!(
            r#"[package]
name = "{app_name}"
version = "0.1.0"
edition = "2021"
[dependencies]
# Pull in the framework facade
ferrisui = {{ path = "../crates/ferrisui" }}
"#
        )
    }
    pub fn generate_gitignore() -> String {
        r#"/target
**/*.rs.bk
Cargo.lock
.DS_Store
"#
        .to_string()
    }
}
