/*
    Appellation: scope <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use std::path::PathBuf;

/// The [`Scope`] struct is a two-part pathlike structure intendend to be used as a
/// configurable mechanism to defining the context and working directory of a service.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(default, rename_all = "lowercase")
)]
pub struct Scope {
    // The root directory of the service
    pub(crate) context: Option<String>,
    // The directory where all of the assets
    pub(crate) workdir: String,
}

impl Scope {
    pub(crate) const DEFAULT_WORKDIR: &'static str = "dist";

    pub fn new(workdir: impl ToString) -> Self {
        Self {
            context: None,
            workdir: workdir.to_string(),
        }
    }
    /// returns the scope context
    pub fn context(&self) -> Option<&str> {
        self.context.as_deref()
    }
    /// returns the scope workdir
    pub fn workdir(&self) -> &str {
        self.workdir.as_str()
    }
    /// converts the scope into a path
    pub fn as_path(&self) -> PathBuf {
        // initialize a new path
        let mut path = PathBuf::new();
        // include the context, if it exists
        if let Some(context) = self.context() {
            path.push(context)
        }
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
    /// update the context of the scope and return a mutable reference to the current instance
    pub fn set_context<T: ToString>(&mut self, context: T) -> &mut Self {
        self.context = Some(context.to_string());
        self
    }
    /// consumes the current scope to create another with the given context
    pub fn with_context(self, context: impl ToString) -> Self {
        Self {
            context: Some(context.to_string()),
            ..self
        }
    }
    /// update the workdir of the scope and return a mutable reference to the current instance
    pub fn set_workdir<T: ToString>(&mut self, workdir: T) -> &mut Self {
        self.workdir = workdir.to_string();
        self
    }
    /// consumes the current scope to create another with the given workdir
    pub fn with_workdir(self, workdir: impl ToString) -> Self {
        Self {
            workdir: workdir.to_string(),
            ..self
        }
    }
    /// only applies the context if it is not empty
    pub fn set_some_context<C: ToString>(&mut self, rhs: Option<C>) {
        rhs.map(|data| self.set_context(data));
    }
    /// only applies the workdir if it is not empty
    pub fn set_some_workdir(&mut self, rhs: Option<impl ToString>) {
        rhs.map(|data| self.set_workdir(data));
    }
}

impl Default for Scope {
    fn default() -> Self {
        Self {
            context: None,
            workdir: Scope::DEFAULT_WORKDIR.into(),
        }
    }
}

impl core::fmt::Display for Scope {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{path}", path = self.as_path().display())
    }
}

impl core::str::FromStr for Scope {
    type Err = crate::ConfigError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::new(s))
    }
}
