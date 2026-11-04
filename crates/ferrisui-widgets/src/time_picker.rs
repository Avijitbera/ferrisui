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
pub struct Time {
    pub hour: u32,   // 0 - 23
    pub minute: u32, // 0 - 59
    pub is_24h: bool,
}

impl Time {
    pub fn new(hour: u32, minute: u32) -> Self {
        Self {
            hour: hour % 24,
            minute: minute % 60,
            is_24h: false,
        }
    }

    pub fn format(&self) -> String {
        if self.is_24h {
            format!("{:02}:{:02}", self.hour, self.minute)
        } else {
            let (h12, am_pm) = if self.hour == 0 {
                (12, "AM")
            } else if self.hour < 12 {
                (self.hour, "AM")
            } else if self.hour == 12 {
                (12, "PM")
            } else {
                (self.hour - 12, "PM")
            };
            format!("{:02}:{:02} {}", h12, self.minute, am_pm)
        }
    }
}

impl Default for Time {
    fn default() -> Self {
        Self {
            hour: 10,
            minute: 30,
            is_24h: false,
        }
    }
}

pub struct TimePicker {
    selected_time: Signal<Time>,
    on_change: Option<Callback<Time>>,
    accent_color: Color,
    background: Color,
}

impl TimePicker {
    pub fn new(selected_time: Signal<Time>) -> Self {
        Self {
            selected_time,
            on_change: None,
            accent_color: Color::hex(0x38BDF8),
            background: Color::hex(0x1E293B),
        }
    }

    pub fn on_change<F>(mut self, handler: F) -> Self
    where
        F: Fn(Time) + Send + Sync + 'static,
    {
        self.on_change = Some(Callback::new(handler));
        self
    }
}

impl View for TimePicker {
    fn build(&self, _ctx: &mut BuildContext) -> ViewNode {
        let current = self.selected_time.get();
        let mut card = ViewNode::new(NodeKind::Container {
            background: self.background,
            corner_radius: 12.0,
            padding: EdgeInsets::all(16.0),
        });
        let mut root_vbox = ViewNode::new(NodeKind::Flex {
            is_vertical: true,
            spacing: 12.0,
        });
        // Title
        root_vbox = root_vbox.with_child(ViewNode::new(NodeKind::Text {
            text: "Select Time".into(),
            font_size: 15.0,
            color: Color::hex(0x94A3B8),
        }));
        let mut time_row = ViewNode::new(NodeKind::Flex {
            is_vertical: false,
            spacing: 10.0,
        });
        // 1. Hour Spinner Block
        let sig_h_up = self.selected_time.clone();
        let cb_h_up = self.on_change.clone();
        let sig_h_dn = self.selected_time.clone();
        let cb_h_dn = self.on_change.clone();
        let mut hour_col = ViewNode::new(NodeKind::Flex {
            is_vertical: true,
            spacing: 4.0,
        });
        // Hour Up Button
        let mut h_up = ViewNode::new(NodeKind::Container {
            background: Color::rgba(1.0, 1.0, 1.0, 0.08),
            corner_radius: 4.0,
            padding: EdgeInsets::symmetric(14.0, 4.0),
        });
        h_up.on_click = Some(Callback::new(move |_| {
            let mut t = sig_h_up.get();
            t.hour = (t.hour + 1) % 24;
            sig_h_up.set(t);
            if let Some(ref cb) = cb_h_up {
                cb.emit(t);
            }
        }));
        h_up = h_up.with_child(ViewNode::new(NodeKind::Text {
            text: "+".into(),
            font_size: 14.0,
            color: Color::WHITE,
        }));
        hour_col = hour_col.with_child(h_up);
        // Hour Value Display
        let (display_hour, am_pm_str) = if current.is_24h {
            (current.hour, "")
        } else if current.hour == 0 {
            (12, "AM")
        } else if current.hour < 12 {
            (current.hour, "AM")
        } else if current.hour == 12 {
            (12, "PM")
        } else {
            (current.hour - 12, "PM")
        };
        let mut hour_val = ViewNode::new(NodeKind::Container {
            background: Color::rgba(0.0, 0.0, 0.0, 0.25),
            corner_radius: 6.0,
            padding: EdgeInsets::symmetric(12.0, 8.0),
        });
        hour_val = hour_val.with_child(ViewNode::new(NodeKind::Text {
            text: format!("{:02}", display_hour),
            font_size: 20.0,
            color: self.accent_color,
        }));
        hour_col = hour_col.with_child(hour_val);
        // Hour Down Button
        let mut h_dn = ViewNode::new(NodeKind::Container {
            background: Color::rgba(1.0, 1.0, 1.0, 0.08),
            corner_radius: 4.0,
            padding: EdgeInsets::symmetric(14.0, 4.0),
        });
        h_dn.on_click = Some(Callback::new(move |_| {
            let mut t = sig_h_dn.get();
            t.hour = if t.hour == 0 { 23 } else { t.hour - 1 };
            sig_h_dn.set(t);
            if let Some(ref cb) = cb_h_dn {
                cb.emit(t);
            }
        }));
        h_dn = h_dn.with_child(ViewNode::new(NodeKind::Text {
            text: "-".into(),
            font_size: 14.0,
            color: Color::WHITE,
        }));
        hour_col = hour_col.with_child(h_dn);
        time_row = time_row.with_child(hour_col);
        // Colon Separator
        time_row = time_row.with_child(ViewNode::new(NodeKind::Text {
            text: ":".into(),
            font_size: 22.0,
            color: Color::WHITE,
        }));
        // 2. Minute Spinner Block
        let sig_m_up = self.selected_time.clone();
        let cb_m_up = self.on_change.clone();
        let sig_m_dn = self.selected_time.clone();
        let cb_m_dn = self.on_change.clone();
        let mut min_col = ViewNode::new(NodeKind::Flex {
            is_vertical: true,
            spacing: 4.0,
        });
        let mut m_up = ViewNode::new(NodeKind::Container {
            background: Color::rgba(1.0, 1.0, 1.0, 0.08),
            corner_radius: 4.0,
            padding: EdgeInsets::symmetric(14.0, 4.0),
        });
        m_up.on_click = Some(Callback::new(move |_| {
            let mut t = sig_m_up.get();
            t.minute = (t.minute + 1) % 60;
            sig_m_up.set(t);
            if let Some(ref cb) = cb_m_up {
                cb.emit(t);
            }
        }));
        m_up = m_up.with_child(ViewNode::new(NodeKind::Text {
            text: "+".into(),
            font_size: 14.0,
            color: Color::WHITE,
        }));
        min_col = min_col.with_child(m_up);
        let mut min_val = ViewNode::new(NodeKind::Container {
            background: Color::rgba(0.0, 0.0, 0.0, 0.25),
            corner_radius: 6.0,
            padding: EdgeInsets::symmetric(12.0, 8.0),
        });
        min_val = min_val.with_child(ViewNode::new(NodeKind::Text {
            text: format!("{:02}", current.minute),
            font_size: 20.0,
            color: self.accent_color,
        }));
        min_col = min_col.with_child(min_val);
        let mut m_dn = ViewNode::new(NodeKind::Container {
            background: Color::rgba(1.0, 1.0, 1.0, 0.08),
            corner_radius: 4.0,
            padding: EdgeInsets::symmetric(14.0, 4.0),
        });
        m_dn.on_click = Some(Callback::new(move |_| {
            let mut t = sig_m_dn.get();
            t.minute = if t.minute == 0 { 59 } else { t.minute - 1 };
            sig_m_dn.set(t);
            if let Some(ref cb) = cb_m_dn {
                cb.emit(t);
            }
        }));
        m_dn = m_dn.with_child(ViewNode::new(NodeKind::Text {
            text: "-".into(),
            font_size: 14.0,
            color: Color::WHITE,
        }));
        min_col = min_col.with_child(m_dn);
        time_row = time_row.with_child(min_col);
        // 3. AM / PM Toggle Pill (for 12h format)
        if !current.is_24h {
            let sig_ampm = self.selected_time.clone();
            let cb_ampm = self.on_change.clone();
            let mut ampm_btn = ViewNode::new(NodeKind::Container {
                background: Color::rgba(1.0, 1.0, 1.0, 0.12),
                corner_radius: 6.0,
                padding: EdgeInsets::symmetric(10.0, 10.0),
            });
            ampm_btn.on_click = Some(Callback::new(move |_| {
                let mut t = sig_ampm.get();
                // Flip AM <-> PM by +/- 12 hours
                if t.hour < 12 {
                    t.hour += 12;
                } else {
                    t.hour -= 12;
                }
                sig_ampm.set(t);
                if let Some(ref cb) = cb_ampm {
                    cb.emit(t);
                }
            }));
            ampm_btn = ampm_btn.with_child(ViewNode::new(NodeKind::Text {
                text: am_pm_str.into(),
                font_size: 14.0,
                color: Color::WHITE,
            }));
            time_row = time_row.with_child(ampm_btn);
        }
        root_vbox = root_vbox.with_child(time_row);
        card = card.with_child(root_vbox);
        card
    }
}
