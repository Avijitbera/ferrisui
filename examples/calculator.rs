// In examples/calculator.rs

use ferrisui::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Op {
    Add,
    Subtract,
    Multiply,
    Divide,
}

#[derive(Clone)]
struct CalculatorApp {
    display: Signal<String>,
    pending_value: Signal<Option<f64>>,
    pending_op: Signal<Option<Op>>,
    clear_on_next: Signal<bool>,
}

impl CalculatorApp {
    pub fn new() -> Self {
        Self {
            display: Signal::new("0".to_string()),
            pending_value: Signal::new(None),
            pending_op: Signal::new(None),
            clear_on_next: Signal::new(false),
        }
    }

    fn push_digit(&self, digit: &str) {
        let mut curr = self.display.get();
        if self.clear_on_next.get() || curr == "0" {
            curr = String::new();
            self.clear_on_next.set(false);
        }
        if curr.len() < 10 {
            curr.push_str(digit);
            self.display.set(curr);
        }
    }

    fn push_dot(&self) {
        let mut curr = self.display.get();
        if self.clear_on_next.get() {
            curr = "0".to_string();
            self.clear_on_next.set(false);
        }
        if !curr.contains('.') {
            curr.push('.');
            self.display.set(curr);
        }
    }

    fn clear_all(&self) {
        self.display.set("0".to_string());
        self.pending_value.set(None);
        self.pending_op.set(None);
        self.clear_on_next.set(false);
    }

    fn toggle_sign(&self) {
        let val: f64 = self.display.get().parse().unwrap_or(0.0);
        if val != 0.0 {
            self.display.set(Self::format_result(-val));
        }
    }

    fn apply_percent(&self) {
        let val: f64 = self.display.get().parse().unwrap_or(0.0);
        self.display.set(Self::format_result(val / 100.0));
        self.clear_on_next.set(true);
    }

    fn set_operator(&self, op: Op) {
        let current_num: f64 = self.display.get().parse().unwrap_or(0.0);
        if let (Some(prev), Some(pending)) = (self.pending_value.get(), self.pending_op.get()) {
            if !self.clear_on_next.get() {
                let res = Self::calculate(prev, current_num, pending);
                self.display.set(Self::format_result(res));
                self.pending_value.set(Some(res));
            }
        } else {
            self.pending_value.set(Some(current_num));
        }
        self.pending_op.set(Some(op));
        self.clear_on_next.set(true);
    }

    fn equals(&self) {
        if let (Some(prev), Some(op)) = (self.pending_value.get(), self.pending_op.get()) {
            let current_num: f64 = self.display.get().parse().unwrap_or(0.0);
            let res = Self::calculate(prev, current_num, op);
            self.display.set(Self::format_result(res));
            self.pending_value.set(None);
            self.pending_op.set(None);
            self.clear_on_next.set(true);
        }
    }

    fn calculate(a: f64, b: f64, op: Op) -> f64 {
        match op {
            Op::Add => a + b,
            Op::Subtract => a - b,
            Op::Multiply => a * b,
            Op::Divide => {
                if b == 0.0 {
                    0.0
                } else {
                    a / b
                }
            }
        }
    }

    fn format_result(val: f64) -> String {
        if val.is_infinite() || val.is_nan() {
            return "Error".to_string();
        }
        let s = format!("{:.6}", val);
        let trimmed = s.trim_end_matches('0').trim_end_matches('.');
        if trimmed.is_empty() {
            "0".to_string()
        } else {
            trimmed.to_string()
        }
    }
}

impl View for CalculatorApp {
    fn build(&self, _ctx: &mut BuildContext) -> ViewNode {
        let display_text = self.display.get();

        // 1. Digital Display Screen
        let display_box = Container::new()
            .background(Color::hex(0x18181B)) // Zinc 900
            .corner_radius(12.0)
            .padding(EdgeInsets::symmetric(24.0, 16.0))
            .child(
                HStack::new().child(Spacer::new()).child(
                    Text::new(display_text)
                        .font_size(36.0)
                        .color(Color::hex(0xF4F4F5)), // Zinc 100
                ),
            );

        // 2. Button helpers
        let make_btn = |label: &'static str,
                        bg: Color,
                        text_c: Color,
                        on_click: Box<dyn Fn() + Send + Sync>| {
            let mut style = ButtonStyle::filled();
            style.text_color = text_c;
            Button::new(label)
                .style(style)
                .background(bg)
                .corner_radius(10.0)
                .padding(EdgeInsets::symmetric(16.0, 8.0))
                .on_click(move || on_click())
        };

        let app = self.clone();

        // 3. Grid of Calculator buttons (4 columns)
        let num_bg = Color::hex(0x27272A); // Zinc 800
        let op_bg = Color::hex(0xF97316); // Orange 500
        let func_bg = Color::hex(0x3F3F46); // Zinc 700
        let text_white = Color::hex(0xFAFAFA);

        let grid = Grid::new(4)
            .gap(10.0)
            // Row 1
            .child(make_btn("C", func_bg, text_white, {
                let a = app.clone();
                Box::new(move || a.clear_all())
            }))
            .child(make_btn("±", func_bg, text_white, {
                let a = app.clone();
                Box::new(move || a.toggle_sign())
            }))
            .child(make_btn("%", func_bg, text_white, {
                let a = app.clone();
                Box::new(move || a.apply_percent())
            }))
            .child(make_btn("÷", op_bg, text_white, {
                let a = app.clone();
                Box::new(move || a.set_operator(Op::Divide))
            }))
            // Row 2
            .child(make_btn("7", num_bg, text_white, {
                let a = app.clone();
                Box::new(move || a.push_digit("7"))
            }))
            .child(make_btn("8", num_bg, text_white, {
                let a = app.clone();
                Box::new(move || a.push_digit("8"))
            }))
            .child(make_btn("9", num_bg, text_white, {
                let a = app.clone();
                Box::new(move || a.push_digit("9"))
            }))
            .child(make_btn("×", op_bg, text_white, {
                let a = app.clone();
                Box::new(move || a.set_operator(Op::Multiply))
            }))
            // Row 3
            .child(make_btn("4", num_bg, text_white, {
                let a = app.clone();
                Box::new(move || a.push_digit("4"))
            }))
            .child(make_btn("5", num_bg, text_white, {
                let a = app.clone();
                Box::new(move || a.push_digit("5"))
            }))
            .child(make_btn("6", num_bg, text_white, {
                let a = app.clone();
                Box::new(move || a.push_digit("6"))
            }))
            .child(make_btn("-", op_bg, text_white, {
                let a = app.clone();
                Box::new(move || a.set_operator(Op::Subtract))
            }))
            // Row 4
            .child(make_btn("1", num_bg, text_white, {
                let a = app.clone();
                Box::new(move || a.push_digit("1"))
            }))
            .child(make_btn("2", num_bg, text_white, {
                let a = app.clone();
                Box::new(move || a.push_digit("2"))
            }))
            .child(make_btn("3", num_bg, text_white, {
                let a = app.clone();
                Box::new(move || a.push_digit("3"))
            }))
            .child(make_btn("+", op_bg, text_white, {
                let a = app.clone();
                Box::new(move || a.set_operator(Op::Add))
            }))
            // Row 5
            .child(make_btn("0", num_bg, text_white, {
                let a = app.clone();
                Box::new(move || a.push_digit("0"))
            }))
            .child(make_btn("00", num_bg, text_white, {
                let a = app.clone();
                Box::new(move || a.push_digit("00"))
            }))
            .child(make_btn(".", num_bg, text_white, {
                let a = app.clone();
                Box::new(move || a.push_dot())
            }))
            .child(make_btn("=", op_bg, text_white, {
                let a = app.clone();
                Box::new(move || a.equals())
            }));

        // 4. Main Calculator Container
        Container::new()
            .background(Color::hex(0x09090B)) // Deep dark black
            .padding(EdgeInsets::all(20.0))
            .child(
                VStack::new()
                    .spacing(16.0)
                    .child(
                        Text::new("FERRISUI CALCULATOR")
                            .font_size(12.0)
                            .color(Color::hex(0x71717A)),
                    )
                    .child(display_box)
                    .child(grid),
            )
            .build(_ctx)
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Launching FerrisUI Calculator...");

    FerrisUI::app(CalculatorApp::new())
        .title("FerrisUI Calculator")
        .size(360.0, 560.0)
        .run()
}
