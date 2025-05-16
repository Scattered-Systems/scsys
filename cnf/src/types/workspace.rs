/*
    Appellation: workspace <config>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use super::Scope;
use std::path::PathBuf;

const _WORKDIR: &str = "dist";
const _APP_NAME: &str = env!("CARGO_PKG_NAME");
const _ARTIFACTS_DIR: &str = "artifacts";
const _BUILD_DIR: &str = "build";

fn _application() -> String {
    std::env::current_exe()
        .map(|path| path.display().to_string())
        .unwrap_or(_APP_NAME.to_string())
}

fn _application_option() -> Option<String> {
    Some(_application())
}

fn _artifacts() -> String {
    _ARTIFACTS_DIR.to_string()
}

fn _default_scope() -> Scope {
    Scope::from_workdir(_WORKDIR)
}

fn _default_context() -> Option<String> {
    Some(".".to_string())
}

fn _default_workdir() -> PathBuf {
    std::env::current_dir().unwrap_or(_WORKDIR.into())
}

/// [Scope] is a structure containing all of the information required for the service to operate.
#[derive(
    Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(default)]
pub struct WorkspaceConfig {
    /// the path to the executable
    #[serde(default = "_application")]
    pub(crate) application: String,
    /// the path to the directory used to store any artifacts
    #[serde(default = "_artifacts")]
    pub(crate) artifacts: String,
    /// a path to another build-script
    pub(crate) build: Option<String>,
    // The root directory of the service
    #[serde(default = "_default_workdir")]
    pub(crate) workdir: PathBuf,
}

impl WorkspaceConfig {
    pub fn new<T>(workdir: T) -> Self
    where
        PathBuf: From<T>,
    {
        Self {
            application: _application(),
            artifacts: _artifacts(),
            build: None,
            workdir: workdir.into(),
        }
    }
    #[inline]
    /// returns a reference to the application name of the workspace
    pub fn application(&self) -> &str {
        &self.application
    }
    #[inline]
    /// returns a reference to the artifacts directory of the workspace
    pub fn artifacts(&self) -> &str {
        &self.artifacts
    }
    /// returns a reference to the workdir of the workspace
    pub const fn workdir(&self) -> &PathBuf {
        &self.workdir
    }
    /// change the current directory to the workspace
    pub fn set_current_dir(&self) {
        let path = self.workdir();
        debug_assert!(self.is_workdir_valid());
        #[cfg(feature = "tracing")]
        tracing::info!("setting current directory to: {p}", p = path.display());
        std::env::set_current_dir(path).unwrap();
    }
    /// set the working directory of the scope
    pub fn set_workdir<T>(&mut self, workdir: T)
    where
        PathBuf: From<T>,
    {
        self.workdir = workdir.into();
    }
    /// if the workdir is set, set it to the given workdir
    pub fn set_workdir_option<T>(&mut self, workdir: Option<T>)
    where
        PathBuf: From<T>,
    {
        workdir.map(|w| self.set_workdir(w));
    }
    /// check if the workdir is valid
    pub fn is_workdir_valid(&self) -> bool {
        self.workdir().is_dir()
    }
    /// get the path to the application binary; if unspecified, the current executable is used
    /// otherwise, the path is assumed to be within the workspaces current directory.
    pub fn path_to_application(&self) -> PathBuf {
        let path = if self.application().is_empty() {
            std::env::current_exe().expect("unable to determine the location of the executable")
        } else {
            self.workdir().join(self.application())
        };
        // ensure the path is a file
        debug_assert!(path.is_file());
        // return the path
        path
    }
    /// get the path to the artifacts directory; the artifacts directory assumed to be a
    /// subdirectory of the workspace and is used to store various build artifacts, logs,
    /// temporary files, etc.
    pub fn path_to_artifacts(&self) -> PathBuf {
        self.workdir().join(self.artifacts())
    }
}

impl Default for WorkspaceConfig {
    fn default() -> Self {
        Self::new(_WORKDIR)
    }
}
