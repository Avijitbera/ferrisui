#[derive(Default)]
pub struct Clipboard;

impl Clipboard {
    pub fn new() -> Self {
        Self
    }

    pub fn get_text(&self) -> Option<String> {
        #[cfg(feature = "clipboard")]
        {
            arboard::Clipboard::new().ok()?.get_text().ok()
        }

        #[cfg(not(feature = "clipboard"))]
        {
            None
        }
    }

    pub fn set_text(&self, text: &str) -> bool {
        #[cfg(feature = "clipboard")]
        {
            if let Ok(mut cb) = arboard::Clipboard::new() {
                return cb.set_text(text).is_ok();
            }
            false
        }
        #[cfg(not(feature = "clipboard"))]
        {
            let _ = text;
            false
        }
    }
}
