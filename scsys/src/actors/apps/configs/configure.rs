/*
    Appellation: configuration <module>
    Contributors: FL03 <jo3mccain@icloud.com> (https://gitlab.com/FL03)
*/
use crate::core::{collect_config_files, DefaultConfigBuilder};

use config::{Config, ConfigError};

pub trait AppConfig<'a>: Clone + serde::Deserialize<'a> + serde::Serialize {
    fn builder(&mut self) -> DefaultConfigBuilder {
        let mut builder = Config::builder();

        builder = builder.add_source(collect_config_files("**/*.config.*", false));
        builder
    }
    fn constructor(&mut self) -> Result<Self, ConfigError> {
        self.builder().build()?.try_deserialize()
    }
}
