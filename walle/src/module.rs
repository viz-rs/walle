pub struct Module<'a> {
    pub config: Option<&'a toml::Value>,

    name: String,
    description: String,
}

impl<'a> Module<'a> {
    pub fn new(name: &str, desc: &str, config: Option<&'a toml::Value>) -> Self {
        Self {
            name: name.to_string(),
            description: desc.to_string(),
            config,
        }
    }
}
