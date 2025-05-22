/*
    Appellation: vars <module>
    Contrib: @FL03
*/

/// a utility function to get an environment variable or return a default value
// #[cfg(feature = "std")]
pub fn env_or_default<T: ToString>(var: &str, default: T) -> String {
    std::env::var(var).unwrap_or_else(|_| default.to_string())
}
