/*
    Appellation: configs <module>
    Contributors: FL03 <jo3mccain@icloud.com> (https://github.com)
    Description:
        ... Summary ...
*/

pub trait AppConfig<'a>: Clone + serde::Deserialize<'a> + serde::Serialize {
    fn builder(&mut self) -> crate::DefaultConfigBuilder {
        let mut builder = config::Config::builder();

        builder = builder.add_source(crate::collect_config_files("**/*.config.*", false));
        builder
    }
    fn constructor(&mut self) -> Result<Self, crate::ConfigError> {
        self.builder().build()?.try_deserialize()
    }
}
