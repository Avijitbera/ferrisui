use ferrisui_core::geometry::{Point, Size};

#[derive(Debug, Clone)]
pub struct MonitorInfo {
    pub name: Option<String>,
    pub size: Size,
    pub position: Point,

    pub scale_factor: f64,
    pub refresh_rate_millihertz: Option<u32>,
}
