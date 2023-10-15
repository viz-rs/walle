use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
#[serde(default)]
pub struct RustConfig<'a> {
    pub symbol: &'a str,
    pub disabled: bool,
    pub detect_extensions: Vec<&'a str>,
    pub detect_files: Vec<&'a str>,
    pub detect_folders: Vec<&'a str>,
}

impl<'a> Default for RustConfig<'a> {
    fn default() -> Self {
        Self {
            symbol: "🦀",
            disabled: false,
            detect_extensions: vec!["rs"],
            detect_files: vec!["Cargo.toml", "rust-toolchain.toml"],
            detect_folders: vec!["target"],
        }
    }
}
