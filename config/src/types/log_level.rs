/*
    Appellation: log_level <module>
    Contrib: @FL03
*/
#[derive(
    Clone,
    Copy,
    Debug,
    Default,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    strum::AsRefStr,
    strum::Display,
    strum::EnumCount,
    strum::EnumIs,
    strum::EnumIter,
    strum::EnumString,
    strum::VariantNames,
)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(rename_all = "lowercase")
)]
#[strum(serialize_all = "lowercase")]
pub enum LogLevel {
    /// the debug designation (rank 1) often used for development
    Debug = 1,
    /// the error designation (rank 4) is used for errors that are not fatal to the program
    Error = 4,
    /// the info designation (rank 2) is used for useful information
    #[default]
    Info = 2,
    /// the trace designation (rank 0) is used for very low-level (often extremely verbose)
    /// information
    Trace = 0,
    /// The warn designation (rank 3)
    Warn = 3,
    /// the off designation (rank -1) is used to turn off logging
    Off = -1,
}

impl LogLevel {
    pub fn from_isize(level: isize) -> Self {
        match level % 5 {
            0 => Self::Trace,
            1 => Self::Debug,
            2 => Self::Info,
            3 => Self::Warn,
            4 => Self::Error,
            lvl if lvl < 0 => Self::Off,
            _ => unreachable!(
                "modular arithmetic error; accepted values are between, but not including, the range of: (+/- 5)"
            ),
        }
    }
    /// returns true if the log-level is not [`Off`](LogLevel::Off)
    pub fn is_enabled(&self) -> bool {
        match self {
            Self::Off => false,
            _ => true,
        }
    }

    pub fn as_log_string<T>(&self, name: T) -> String
    where
        T: core::fmt::Display,
    {
        format!("{name}={lvl}", lvl = self.as_ref())
    }
}

unsafe impl Send for LogLevel {}

unsafe impl Sync for LogLevel {}

impl From<isize> for LogLevel {
    fn from(level: isize) -> Self {
        Self::from_isize(level)
    }
}

impl From<LogLevel> for isize {
    fn from(level: LogLevel) -> Self {
        level as isize
    }
}

#[cfg(feature = "config")]
impl From<LogLevel> for config::Value {
    fn from(level: LogLevel) -> Self {
        level.to_string().into()
    }
}

#[cfg(feature = "tracing")]
mod impl_tracing {
    use super::LogLevel;
    use tracing::Level;

    impl LogLevel {
        pub fn from_tracing(level: Level) -> Self {
            match level {
                Level::DEBUG => Self::Debug,
                Level::ERROR => Self::Error,
                Level::INFO => Self::Info,
                Level::TRACE => Self::Trace,
                Level::WARN => Self::Warn,
            }
        }
        /// convert the configured log level into a tracing [level](tracing::Level)
        pub fn as_tracing_level(&self) -> Option<Level> {
            match self {
                Self::Debug => Some(Level::DEBUG),
                Self::Error => Some(Level::ERROR),
                Self::Info => Some(Level::INFO),
                Self::Trace => Some(Level::TRACE),
                Self::Warn => Some(Level::WARN),
                Self::Off => None,
            }
        }
    }

    impl From<Level> for LogLevel {
        fn from(level: Level) -> Self {
            Self::from_tracing(level)
        }
    }

    impl From<LogLevel> for Level {
        fn from(level: LogLevel) -> Self {
            level.as_tracing_level().unwrap_or(Level::INFO)
        }
    }
}
