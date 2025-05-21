/*
    Appellation: scope <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use std::path::PathBuf;

const DEFAULT_WORKDIR: &str = "dist";

fn _default_context() -> Option<String> {
    Some(env!("CARGO_MANIFEST_DIR").to_string())
}

/// [Scope] stores critical information regarding the applications current position within
/// the filesystem. The context is considered to be the current working directory of the
/// application while the workdir is used to point to the directory where all of the assets
/// are stored.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(default, rename_all = "snake_case")
)]
pub struct Scope {
    // The root directory of the service
    pub(crate) context: Option<String>,
    // The directory where all of the assets
    pub(crate) workdir: String,
}

impl Scope {
    pub fn from_workdir<T: ToString>(workdir: T) -> Self {
        Self {
            context: _default_context(),
            workdir: workdir.to_string(),
        }
    }
    /// returns a reference to the context of the scope
    pub fn context(&self) -> Option<&String> {
        self.context.as_ref()
    }
    /// returns a reference to the workdir of the scope
    pub fn workdir(&self) -> &str {
        &self.workdir
    }
    /// update the context and return a mutable reference to the instance for chaining purposes
    pub fn set_context(&mut self, context: impl ToString) -> &mut Self {
        self.context = Some(context.to_string());
        self
    }
    /// update the workdir and return a mutable reference to the instance for chaining purposes
    pub fn set_workdir(&mut self, workdir: impl ToString) -> &mut Self {
        self.workdir = workdir.to_string();
        self
    }
    /// optionally apply the given context to the scope; this means that if None is passed,
    /// nothing happens so the previous state of the context is preserved.
    pub fn set_some_context(&mut self, rhs: Option<impl ToString>) {
        rhs.map(|data| self.context = Some(data.to_string()));
    }
    /// optionally apply the given workdir to the scope; this means that if None is passed,
    /// nothing happens so the previous state of the workdir is preserved.
    pub fn set_some_workdir(&mut self, rhs: Option<impl ToString>) {
        rhs.map(|data| self.workdir = data.to_string());
    }
    /// consumes the current instance to return another using the given context
    pub fn with_context(self, context: impl ToString) -> Self {
        Self {
            context: Some(context.to_string()),
            ..self
        }
    }
    /// consumes the current instance to return another using the given workdir
    pub fn with_workdir(self, workdir: impl ToString) -> Self {
        Self {
            workdir: workdir.to_string(),
            ..self
        }
    }
    /// converts the scope into a path
    pub fn as_path(&self) -> PathBuf {
        // initialize a new path
        let mut path = PathBuf::new();
        // include the context, if it exists
        self.context().clone().map(|context| path.push(context));
        // add the workdir
        path.push(self.workdir());
        // ensure the path is a directory
        debug_assert!(path.is_dir());
        // return the path
        path
    }
    /// converts the scope into a string
    pub fn as_path_str(&self) -> String {
        self.as_path().display().to_string()
    }
    /// sets the current working directory to the scope
    pub fn set_cwd(&self) {
        std::env::set_current_dir(self.as_path()).unwrap();
    }
}

impl Default for Scope {
    fn default() -> Self {
        Self {
            context: _default_context(),
            workdir: DEFAULT_WORKDIR.to_string(),
        }
    }
}

impl core::fmt::Display for Scope {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{path}", path = self.as_path().display())
    }
}

impl core::str::FromStr for Scope {
    type Err = std::io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::from_workdir(s))
    }
}
