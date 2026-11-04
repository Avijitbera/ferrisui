use ferrisui_core::{
    build_context::BuildContext,
    callback::Callback,
    color::Color,
    geometry::EdgeInsets,
    state::Signal,
    view::View,
    view_node::{NodeKind, ViewNode},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Date {
    pub year: i32,
    pub month: u32,
    pub day: u32,
}

impl Date {
    pub fn new(year: i32, month: u32, day: u32) -> Self {
        Self { year, month, day }
    }

    pub fn is_leap_year(year: i32) -> bool {
        (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
    }

    pub fn days_in_month(year: i32, month: u32) -> u32 {
        match month {
            1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
            4 | 6 | 9 | 11 => 30,
            2 => {
                if Self::is_leap_year(year) {
                    29
                } else {
                    28
                }
            }
            _ => 30,
        }
    }

    /// Returns day of week for the 1st of the month: 0 = Mon, 1 = Tue, ..., 6 = Sun.
    pub fn first_day_weekday(year: i32, month: u32) -> u32 {
        // Tomohiko Sakamoto's algorithm (0 = Sun, 1 = Mon ... 6 = Sat)
        let t = [0, 3, 2, 5, 0, 3, 5, 1, 4, 6, 2, 4];
        let y = if month < 3 { year - 1 } else { year };
        let sun_0 = (y + y / 4 - y / 100 + y / 400 + t[(month - 1) as usize] + 1) % 7;
        // Shift to Monday = 0
        ((sun_0 + 6) % 7) as u32
    }

    pub fn month_name(month: u32) -> &'static str {
        match month {
            1 => "January",
            2 => "February",
            3 => "March",
            4 => "April",
            5 => "May",
            6 => "June",
            7 => "July",
            8 => "August",
            9 => "September",
            10 => "October",
            11 => "November",
            12 => "December",
            _ => "Unknown",
        }
    }

    pub fn format(&self) -> String {
        format!("{:04}-{:02}-{:02}", self.year, self.month, self.day)
    }
}

impl Default for Date {
    fn default() -> Self {
        Self {
            year: 2026,
            month: 11,
            day: 4,
        }
    }
}

pub struct DatePicker {
    selected_date: Signal<Date>,
    viewed_year: Signal<i32>,
    viewed_month: Signal<u32>,
    on_select: Option<Callback<Date>>,
    accent_color: Color,
    background: Color,
}

impl DatePicker {
    pub fn new(selected_date: Signal<Date>) -> Self {
        let initial = selected_date.get();
        Self {
            selected_date,
            viewed_year: Signal::new(initial.year),
            viewed_month: Signal::new(initial.month),
            on_select: None,
            accent_color: Color::hex(0x38BDF8), // Sky 400
            background: Color::hex(0x1E293B),   // Slate 800 Card
        }
    }
    pub fn accent_color(mut self, color: Color) -> Self {
        self.accent_color = color;
        self
    }
    pub fn background(mut self, color: Color) -> Self {
        self.background = color;
        self
    }

    pub fn on_select<F>(mut self, handler: F) -> Self
    where
        F: Fn(Date) + Send + Sync + 'static,
    {
        self.on_select = Some(Callback::new(handler));
        self
    }
}

impl View for DatePicker {
    fn build(&self, ctx: &mut BuildContext) -> ViewNode {
        let cur_year = self.viewed_year.get();
        let cur_month = self.viewed_month.get();
        let selected = self.selected_date.get();
        // Outer Card Container
        let mut card = ViewNode::new(NodeKind::Container {
            background: self.background,
            corner_radius: 12.0,
            padding: EdgeInsets::all(16.0),
        });

        let mut vbox = ViewNode::new(NodeKind::Flex {
            is_vertical: true,
            spacing: 12.0,
        });
        // 1. Navigation Header: [<] Month YYYY [>]
        let mut nav_row = ViewNode::new(NodeKind::Flex {
            is_vertical: false,
            spacing: 16.0,
        });

        // Prev month button
        let ym_y = self.viewed_year.clone();
        let ym_m = self.viewed_month.clone();
        let mut prev_btn = ViewNode::new(NodeKind::Container {
            background: Color::rgba(1.0, 1.0, 1.0, 0.08),
            corner_radius: 6.0,
            padding: EdgeInsets::symmetric(10.0, 6.0),
        });
        prev_btn.on_click = Some(Callback::new(move |_| {
            let m = ym_m.get();
            if m == 1 {
                ym_m.set(12);
                ym_y.set(ym_y.get() - 1);
            } else {
                ym_m.set(m - 1);
            }
        }));
        prev_btn = prev_btn.with_child(ViewNode::new(NodeKind::Text {
            text: "<".into(),
            font_size: 14.0,
            color: Color::WHITE,
        }));
        nav_row = nav_row.with_child(prev_btn);
        // Month & Year Label
        nav_row = nav_row.with_child(ViewNode::new(NodeKind::Text {
            text: format!("{} {}", Date::month_name(cur_month), cur_year),
            font_size: 16.0,
            color: Color::WHITE,
        }));

        let ny_y = self.viewed_year.clone();
        let ny_m = self.viewed_month.clone();
        let mut next_btn = ViewNode::new(NodeKind::Container {
            background: Color::rgba(1.0, 1.0, 1.0, 0.08),
            corner_radius: 6.0,
            padding: EdgeInsets::symmetric(10.0, 6.0),
        });
        next_btn.on_click = Some(Callback::new(move |_| {
            let m = ny_m.get();
            if m == 12 {
                ny_m.set(1);
                ny_y.set(ny_y.get() + 1);
            } else {
                ny_m.set(m + 1);
            }
        }));
        next_btn = next_btn.with_child(ViewNode::new(NodeKind::Text {
            text: ">".into(),
            font_size: 14.0,
            color: Color::WHITE,
        }));

        nav_row = nav_row.with_child(next_btn);
        vbox = vbox.with_child(nav_row);
        // 2. Weekday Header Row: Mo Tu We Th Fr Sa Su
        let mut weekday_row = ViewNode::new(NodeKind::Flex {
            is_vertical: false,
            spacing: 10.0,
        });

        for &day_abbr in &["Mo", "Tu", "We", "Th", "Fr", "Sa", "Su"] {
            weekday_row = weekday_row.with_child(ViewNode::new(NodeKind::Text {
                text: day_abbr.into(),
                font_size: 12.0,
                color: Color::hex(0x64748B),
            }));
        }

        vbox = vbox.with_child(weekday_row);
        // 3. Days Grid: 7 columns, offset by first_day_weekday
        let first_weekday = Date::first_day_weekday(cur_year, cur_month);
        let total_days = Date::days_in_month(cur_year, cur_month);
        let mut current_day = 1u32;
        let mut cell_idx = 0u32;

        while current_day <= total_days {
            let mut week_row = ViewNode::new(NodeKind::Flex {
                is_vertical: false,
                spacing: 6.0,
            });

            for _col in 0..7 {
                if cell_idx < first_weekday || current_day > total_days {
                    // Empty spacer cell
                    let empty_cell = ViewNode::new(NodeKind::Container {
                        background: Color::TRANSPARENT,
                        corner_radius: 4.0,
                        padding: EdgeInsets::symmetric(10.0, 6.0),
                    });
                    week_row = week_row.with_child(empty_cell);
                } else {
                    let d = current_day;
                    let is_selected = selected.year == cur_year
                        && selected.month == cur_month
                        && selected.day == d;
                    let bg = if is_selected {
                        self.accent_color
                    } else {
                        Color::rgba(1.0, 1.0, 1.0, 0.04)
                    };
                    let text_color = if is_selected {
                        Color::hex(0x0F172A)
                    } else {
                        Color::WHITE
                    };

                    let mut day_cell = ViewNode::new(NodeKind::Container {
                        background: bg,
                        corner_radius: 6.0,
                        padding: EdgeInsets::symmetric(8.0, 6.0),
                    });
                    let sel_sig = self.selected_date.clone();
                    let cb = self.on_select.clone();

                    day_cell.on_click = Some(Callback::new(move |_| {
                        let new_date = Date::new(cur_year, cur_month, d);
                        sel_sig.set(new_date);
                        if let Some(ref callback) = cb {
                            callback.emit(new_date);
                        }
                    }));

                    day_cell = day_cell.with_child(ViewNode::new(NodeKind::Text {
                        text: format!("{:2}", d),
                        font_size: 12.0,
                        color: text_color,
                    }));

                    week_row = week_row.with_child(day_cell);
                    current_day += 1;
                }

                cell_idx += 1;
            }
            vbox = vbox.with_child(week_row);
        }

        card = card.with_child(vbox);
        card
    }
}
