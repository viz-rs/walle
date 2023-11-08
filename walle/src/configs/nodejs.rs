use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
#[serde(default)]
pub struct NodejsConfig<'a> {
    pub symbol: &'a str,
    pub disabled: bool,
    pub detect_extensions: Vec<&'a str>,
    pub detect_files: Vec<&'a str>,
    pub detect_folders: Vec<&'a str>,
}

impl<'a> Default for NodejsConfig<'a> {
    fn default() -> Self {
        Self {
            symbol: "",
            disabled: false,
            detect_extensions: vec!["js", "mjs", "cjs", "ts", "mts", "cts"],
            detect_files: vec!["package.json", ".node-version", ".nvmrc"],
            detect_folders: vec!["node_modules"],
        }
    }
}
