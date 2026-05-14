/*
    Appellation: tracing <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(default, rename_all = "snake_case")
)]
pub struct TracingConfig {
    pub(crate) ansi: bool,
    pub(crate) file: bool,
    pub(crate) line_number: bool,
    pub(crate) target: bool,
    pub(crate) timer: bool,
    pub(crate) thread_ids: bool,
    pub(crate) thread_names: bool,
}

impl TracingConfig {
    pub fn new() -> Self {
        Self {
            ansi: true,
            file: false,
            line_number: false,
            target: true,
            timer: true,
            thread_ids: false,
            thread_names: false,
        }
    }
    /// Set the ansi toggle
    pub fn ansi(self, ansi: bool) -> Self {
        Self { ansi, ..self }
    }
    /// toggle if the ansi colors should be used in the tracing output; (default: false)
    pub fn file(self, file: bool) -> Self {
        Self { file, ..self }
    }
    /// toggle if the file name should be included in the tracing output; (default: false)
    pub fn line_number(self, line_number: bool) -> Self {
        Self {
            line_number,
            ..self
        }
    }
    /// toggle if the target should be included in the tracing output; (default: true)
    pub fn target(self, target: bool) -> Self {
        Self { target, ..self }
    }
    /// toggle if the timer should be included in the tracing output; (default: true)
    pub fn timer(self, timer: bool) -> Self {
        Self { timer, ..self }
    }
    /// toggle if the thread ids should be included in the tracing output; (default: false)
    pub fn thread_ids(self, thread_ids: bool) -> Self {
        Self { thread_ids, ..self }
    }
    /// toggle if the thread names should be included in the tracing output; (default: false)
    pub fn thread_names(self, thread_names: bool) -> Self {
        Self {
            thread_names,
            ..self
        }
    }

    pub fn get(&self, name: &str) -> bool {
        match name {
            "ansi" => self.ansi,
            "file" => self.file,
            "line_number" => self.line_number,
            "target" => self.target,
            "timer" => self.timer,
            "thread_ids" => self.thread_ids,
            "thread_names" => self.thread_names,
            _ => panic!("unknown tracing config option"),
        }
    }
    /// returns true if the ansi toggle is set
    pub const fn get_ansi(&self) -> bool {
        self.ansi
    }
    /// returns true if the file toggle is set
    pub const fn get_file(&self) -> bool {
        self.file
    }
    /// returns true if the line number is toggled
    pub const fn get_line_number(&self) -> bool {
        self.line_number
    }
    /// returns true if the target is toggled
    pub const fn get_target(&self) -> bool {
        self.target
    }
    /// returns true if the thread ids are toggled
    pub const fn get_thread_ids(&self) -> bool {
        self.thread_ids
    }
    /// returns true if the thread names are toggled
    pub const fn get_thread_names(&self) -> bool {
        self.thread_names
    }

    #[cfg(feature = "tracing-subscriber")]
    /// Initialize the tracer with the given name
    pub fn init_tracing(self, level: crate::LogLevel, name: Option<&str>) -> Self {
        use tracing_subscriber::{filter::EnvFilter, util::SubscriberInitExt};
        let name = name.unwrap_or(env!("CARGO_PKG_NAME"));
        let filter = EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| format!("{name}={level},tower_http={level}").into());
        // deconstruct the config
        let Self {
            ansi,
            file,
            line_number,
            target,
            thread_ids,
            thread_names,
            ..
        } = self;
        // initialize the tracing subscriber
        tracing_subscriber::fmt()
            .compact()
            .with_ansi(ansi)
            .with_env_filter(filter)
            .with_file(file)
            .with_line_number(line_number)
            .with_max_level(level.as_tracing_level())
            .with_target(target)
            .with_thread_ids(thread_ids)
            .with_thread_names(thread_names)
            .with_timer(tracing_subscriber::fmt::time::uptime())
            .finish()
            .init();
        // trace the success
        tracing::trace!("successfully initialized the tracing subscriber");
        self
    }
}

impl Default for TracingConfig {
    fn default() -> Self {
        Self::new()
    }
}

impl core::fmt::Display for TracingConfig {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        #[cfg(feature = "json")]
        {
            f.write_str(&serde_json::to_string(self).unwrap())
        }
        #[cfg(not(feature = "json"))]
        {
            write!(
                f,
                "{{ ansi: {}, file: {}, line_number: {}, target: {}, timer: {}, thread_ids: {}, thread_names: {} }}",
                self.ansi,
                self.file,
                self.line_number,
                self.target,
                self.timer,
                self.thread_ids,
                self.thread_names
            )
        }
    }
}
