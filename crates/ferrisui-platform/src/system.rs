#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ThemePreference {
    #[default]
    Dark,
    Light,
}

pub struct System;

impl System {
    pub fn theme_preference() -> ThemePreference {
        ThemePreference::Dark
    }

    pub fn os_name() -> &'static str {
        std::env::consts::OS
    }

    pub fn arch() -> &'static str {
        std::env::consts::ARCH
    }
}
