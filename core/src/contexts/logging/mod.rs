/*
    Appellation: logging <module>
    Contributors: FL03 <jo3mccain@icloud.com> (https://gitlab.com/FL03)
    Description:
        ... Summary ...
*/
pub use self::{logger::Logger, utils::logger_from_env};

mod logger;

pub trait LoggerSpec {
    fn level(&self) -> String;
    fn setup(&self) -> &Self {
        logger_from_env(Some(self.level().as_str()));
        self
    }
}

pub(crate) mod utils {
    pub fn logger_from_env(level: Option<&str>) {
        let env_var = "RUST_LOG";
        let level = match level {
            Some(v) => v,
            None => "info",
        };
        if std::env::var_os(env_var).is_none() {
            std::env::set_var(env_var, level)
        }
        tracing_subscriber::fmt::init()
    }
}
