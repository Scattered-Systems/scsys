/*
    Appellation: configuration <module>
    Contributors: FL03 <jo3mccain@icloud.com> (https://gitlab.com/FL03)
*/
use crate::core::{collect_config_files, ConfigResult, DefaultConfigBuilder};
use config::{Config, Environment};

pub trait AppConfig<'a>: Clone + serde::Deserialize<'a> + serde::Serialize {
    fn build(
        &self,
        env: Option<&str>,
        pattern: Option<&str>,
        required: Option<bool>,
    ) -> ConfigResult<Self> {
        let mut builder = self.builder();
        match env {
            Some(v) => {
                builder = builder.add_source(Environment::default().separator(v));
            }
            None => builder = builder.add_source(Environment::default().separator("__")),
        }

        match pattern {
            Some(pat) => match required {
                Some(req) => {
                    builder = builder.add_source(collect_config_files(pat, req));
                }
                None => {
                    builder = builder.add_source(collect_config_files(pat, false));
                }
            },
            None => {}
        }
        builder = builder.add_source(collect_config_files("**/*.config.*", false));

        builder.build()?.try_deserialize()
    }
    fn builder(&self) -> DefaultConfigBuilder {
        Config::builder()
    }
}
