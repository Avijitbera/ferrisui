use ferrisui::prelude::*;

struct CounterApp {
    count: Signal<i32>,
}

impl CounterApp {
    pub fn new() -> Self {
        Self {
            count: Signal::new(0),
        }
    }
}

impl View for CounterApp {
    fn build(&self, _ctx: &mut BuildContext) -> ViewNode {
        let count = self.count.clone();
        let val = count.get();

        let count_dec = count.clone();
        let count_inc = count.clone();
        let count_reset = count.clone();

        Container::new()
            .background(Color::hex(0x0B0F19))
            .child(
                VStack::new()
                    .spacing(24.0)
                    .child(
                        Text::new("FERRISUI REACTIVE COUNTER")
                            .font_size(22.0)
                            .color(Color::hex(0x38BDF8)),
                    )
                    .child(
                        Container::new()
                            .background(Color::hex(0x1E293B))
                            .padding(EdgeInsets::symmetric(32.0, 16.0))
                            .child(
                                Text::new(format!("COUNT: {}", val))
                                    .font_size(36.0)
                                    .color(if val > 0 {
                                        Color::hex(0x4ADE80)
                                    } else if val < 0 {
                                        Color::hex(0xF87171)
                                    } else {
                                        Color::hex(0xE2E8F0)
                                    }),
                            ),
                    )
                    .child(
                        HStack::new()
                            .spacing(16.0)
                            .child(
                                Button::new("- DECREMENT")
                                    .style(ButtonStyle::outlined())
                                    .on_click(move || {
                                        let next = count_dec.update(|n| {
                                            *n -= 1;
                                            *n
                                        });
                                        println!("🔻 Decremented count to: {}", next);
                                    }),
                            )
                            .child(
                                Button::new("+ INCREMENT")
                                    .style(ButtonStyle::filled())
                                    .on_click(move || {
                                        let next = count_inc.update(|n| {
                                            *n += 1;
                                            *n
                                        });
                                        println!("🔺 Incremented count to: {}", next);
                                    }),
                            ),
                    )
                    .child(
                        Button::new("RESET")
                            .style(ButtonStyle::text())
                            .on_click(move || {
                                count_reset.set(0);
                                println!("🔄 Counter reset to 0");
                            }),
                    ),
            )
            .build(_ctx)
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🦀 Starting FerrisUI Interactive Counter Application...");

    FerrisUI::app(CounterApp::new())
        .title("FerrisUI - Interactive Counter")
        .size(600.0, 480.0)
        .clear_color(Color::hex(0x0B0F19))
        .run()
}
