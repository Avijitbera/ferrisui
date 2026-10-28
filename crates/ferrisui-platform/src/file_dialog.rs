use std::path::PathBuf;

pub struct FileDialog;

impl FileDialog {
    pub fn pick_file(title: Option<&str>) -> Option<PathBuf> {
        #[cfg(feature = "file-dialog")]
        {
            let mut dialog = rfd::FileDialog::new();
            if let Some(t) = title {
                dialog = dialog.set_title(t);
            }
            dialog.pick_file()
        }
        #[cfg(not(feature = "file-dialog"))]
        {
            let _ = title;
            None
        }
    }

    pub fn save_file(title: Option<&str>, default_name: Option<&str>) -> Option<PathBuf> {
        #[cfg(feature = "file-dialog")]
        {
            let mut dialog = rfd::FileDialog::new();
            if let Some(t) = title {
                dialog = dialog.set_title(t);
            }
            if let Some(n) = default_name {
                dialog = dialog.set_file_name(n);
            }
            dialog.save_file()
        }
        #[cfg(not(feature = "file-dialog"))]
        {
            let _ = (title, default_name);
            None
        }
    }

    pub fn pick_folder(title: Option<&str>) -> Option<PathBuf> {
        #[cfg(feature = "file-dialog")]
        {
            let mut dialog = rfd::FileDialog::new();
            if let Some(t) = title {
                dialog = dialog.set_title(t);
            }
            dialog.pick_folder()
        }
        #[cfg(not(feature = "file-dialog"))]
        {
            let _ = title;
            None
        }
    }
}
