/*
    Appellation: logger <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#[doc(inline)]
pub use self::tracing::*;

pub(crate) mod tracing;

use crate::LogLevel;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(default, rename_all = "snake_case")
)]
pub struct LoggerConfig {
    pub level: LogLevel,
    pub name: String,
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub tracing: TracingConfig,
}

impl LoggerConfig {
    const LOGGER_PREFIX: &str = "app";

    pub fn new() -> Self {
        Self::from_name(Self::LOGGER_PREFIX)
    }

    pub fn from_name<T: ToString>(name: T) -> Self {
        Self {
            level: LogLevel::default(),
            name: name.to_string(),
            tracing: TracingConfig::new(),
        }
    }

    pub fn from_level(level: LogLevel) -> Self {
        Self {
            level,
            name: Self::LOGGER_PREFIX.to_string(),
            tracing: TracingConfig::new(),
        }
    }
    /// returns a copy of the configured [`level`](LogLevel)
    pub const fn level(&self) -> LogLevel {
        self.level
    }
    /// returns a mutable reference to the configured [`level`](LogLevel)
    pub const fn level_mut(&mut self) -> &mut LogLevel {
        &mut self.level
    }
    /// returns a reference to the current logger name
    pub const fn name(&self) -> &String {
        &self.name
    }
    /// returns a mutable reference to the current logger name
    pub const fn name_mut(&mut self) -> &mut String {
        &mut self.name
    }
    /// returns a copy of the current [`tracing`](TracingConfig) schema
    pub const fn tracing(&self) -> TracingConfig {
        self.tracing
    }
    /// returns a mutable reference to the current tracing configuration
    pub const fn tracing_mut(&mut self) -> &mut TracingConfig {
        &mut self.tracing
    }
    /// update the current logger level and return a mutable reference
    pub fn set_level(&mut self, level: LogLevel) -> &mut Self {
        self.level = level;
        self
    }
    /// update the current logger name and return a mutable reference
    pub fn set_name<T: ToString>(&mut self, name: T) -> &mut Self {
        self.name = name.to_string();
        self
    }
    /// update the current tracing schema and return a mutable reference
    pub fn set_tracing(&mut self, tracing: TracingConfig) -> &mut Self {
        self.tracing = tracing;
        self
    }
    /// consumes the current instance to create another with the given logger level
    pub fn with_level(self, level: LogLevel) -> Self {
        Self { level, ..self }
    }
    /// consumes the current instance to create another with the given logger name
    pub fn with_name<T: ToString>(self, name: T) -> Self {
        Self {
            name: name.to_string(),
            ..self
        }
    }
    /// consumes the current instance to create another with the given tracing schema
    pub fn with_tracing(self, tracing: TracingConfig) -> Self {
        Self { tracing, ..self }
    }

    #[cfg(feature = "tracing")]
    pub fn init_tracing(&self) {
        self.tracing().init_tracing(self.level, Some(&self.name));
    }
}

impl Default for LoggerConfig {
    fn default() -> Self {
        Self::new()
    }
}

/*
 ************* Implementations *************
*/
impl core::fmt::Display for LoggerConfig {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        #[cfg(feature = "json")]
        {
            f.write_str(&serde_json::to_string(self).unwrap())
        }
        #[cfg(not(feature = "json"))]
        {
            write!(f, "{{ level: {}, name: {} }}", self.level, self.name)
        }
    }
}

unsafe impl Send for LoggerConfig {}

unsafe impl Sync for LoggerConfig {}
