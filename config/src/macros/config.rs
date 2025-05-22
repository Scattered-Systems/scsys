/// create a macro to streamline the process of adding sources
///
/// ```rust
/// use scsys_config::config_sources;
///
/// let mut builder = config::ConfigBuilder::new();
///
/// builder = config_sources! {
///     builder {
///         config::File::with_name("config.toml").required(false),
///     }
/// };
/// ```
#[macro_export]
macro_rules! config_sources {
    ($builder:ident {$($source:expr),* $(,)?}) => {
        $crate::config_sources!(@impl $builder::<[$($source),*]>)
    };
    (@impl $builder:ident::<[$($source:expr),* $(,)?]>) => {
        $builder$(.add_source($source))*
    };
    (@file file($wd:expr).required($req:expr)) => {
        config::File::with_name($wd).required($req);
    };
}
