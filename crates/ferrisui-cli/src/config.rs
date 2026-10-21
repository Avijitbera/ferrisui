use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WindowConfig {
    pub title: String,
    pub width: u32,
    pub height: u32,
    pub resizable: bool,
    pub transparent: bool,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AppManifest {
    pub package: PackageConfig,
    pub window: WindowConfig,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PackageConfig {
    pub name: String,
    pub version: String,
    pub identifier: String,
}

impl Default for AppManifest {
    fn default() -> Self {
        Self {
            package: PackageConfig {
                name: "ferrisui_app".to_string(),
                version: "0.1.0".to_string(),
                identifier: "com.example.ferrisui".to_string(),
            },
            window: WindowConfig {
                title: "FerrisUI App".to_string(),
                width: 900,
                height: 600,
                resizable: true,
                transparent: false,
            },
        }
    }
}

impl AppManifest {
    pub fn load_from_dir(dir: &Path) -> anyhow::Result<Self> {
        let manifest_path = dir.join("FerrisUI.toml");
        if !manifest_path.exists() {
            return Ok(Self::default());
        }
        let content = std::fs::read_to_string(&manifest_path)?;
        let manifest: AppManifest = toml::from_str(&content)?;
        Ok(manifest)
    }

    pub fn to_toml_string(&self) -> anyhow::Result<String> {
        Ok(toml::to_string_pretty(self)?)
    }
}
