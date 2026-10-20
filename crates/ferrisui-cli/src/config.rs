use serde::{Deserialize, Serialize};

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
