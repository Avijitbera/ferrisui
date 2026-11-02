use ferrisui::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🦀 Starting FerrisUI Hello World application...");

    FerrisUI::app(
        Container::new()
            .background(Color::hex(0x0F172A))
            .child(
                VStack::new()
                    .spacing(16.0)
                    .child(
                        Text::new("HELLO, FERRISUI!")
                            .font_size(28.0)
                            .color(Color::hex(0x38BDF8)),
                    )
                    .child(
                        Text::new("A MODERN DECLARATIVE UI FRAMEWORK FOR RUST")
                            .font_size(14.0)
                            .color(Color::hex(0x94A3B8)),
                    )
                    .child(
                        Button::new("GET STARTED")
                            .style(ButtonStyle::filled())
                            .on_click(|| {
                                println!("🚀 Get Started button clicked!");
                            }),
                    ),
            ),
    )
    .title("FerrisUI - Hello World")
    .size(640.0, 420.0)
    .clear_color(Color::hex(0x0F172A))
    .run()
}
