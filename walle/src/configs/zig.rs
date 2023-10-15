use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
#[serde(default)]
pub struct ZigConfig<'a> {
    pub symbol: &'a str,
    pub disabled: bool,
    pub detect_extensions: Vec<&'a str>,
    pub detect_files: Vec<&'a str>,
    pub detect_folders: Vec<&'a str>,
}

impl<'a> Default for ZigConfig<'a> {
    fn default() -> Self {
        Self {
            symbol: "↯",
            disabled: false,
            detect_extensions: vec!["zig"],
            detect_files: vec!["build.zig"],
            detect_folders: vec![],
        }
    }
}
