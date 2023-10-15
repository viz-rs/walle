use serde::{de::Error as SerdeError, Deserialize};
use toml::{de::Error, Value};

pub trait ModuleConfig<'a, E>
where
    Self: Default,
    E: SerdeError,
{
    fn from_config(config: Value) -> Result<Self, E>;

    fn load(config: Value) -> Self {
        match Self::from_config(config) {
            Ok(config) => config,
            Err(e) => {
                // log::warn!("Failed to load config value: {}", e);
                Self::default()
            }
        }
    }

    fn try_load(config: Option<Value>) -> Self {
        config.map(Into::into).map(Self::load).unwrap_or_default()
    }
}

impl<'a, T: Deserialize<'a> + Default> ModuleConfig<'a, Error> for T {
    fn from_config(config: Value) -> Result<Self, Error> {
        T::deserialize(config)
    }
}
