/*
    Appellation: loggers <module>
    Contributors: FL03 <jo3mccain@icloud.com> (https://github.com)
    Description:
        ... Summary ...
*/

#[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Logger {
    pub level: String,
}

impl Logger {
    pub fn new(level: String) -> Self {
        Self { level }
    }

    pub fn from<T: std::string::ToString>(level: T) -> Self {
        Self::new(level.to_string())
    }

    pub fn setup(&self) {
        if std::env::var_os("RUST_LOG").is_none() {
            std::env::set_var("RUST_LOG", self.level.as_str());
        }

        tracing_subscriber::fmt::init();
    }
}
