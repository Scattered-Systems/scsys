/*
    appellation: drivers <module>
    authors: @FL03
*/
/// The [`DatabaseDrivers`] implementation is an enumeration of supported databases and their 
/// connection formats.
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
    serde::Deserialize,
    serde::Serialize,
    strum::AsRefStr,
    strum::Display,
    strum::EnumString,
    strum::EnumIs,
    strum::EnumIter,
    strum::VariantArray,
    strum::VariantNames,
)]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
pub enum DatabaseDrivers {
    #[serde(alias = "postgresql", alias = "postgres")]
    #[strum(to_string = "postgresql")]
    Postgresql,
    #[serde(alias = "mysql")]
    Mysql,
    #[default]
    #[serde(alias = "sqlite", alias = "sqlite3")]
    #[strum(to_string = "sqlite")]
    Sqlite,
}

/// Extends the standard [`DatabaseUriSchema`] connection schema with additional options for
/// alternative resource types such as redis, object-storage, etc.
#[derive(
    Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, serde::Deserialize, serde::Serialize,
)]
#[serde(untagged, rename_all = "snake_case")]
pub enum StorageProvider {
    Sql(DatabaseUriSchema),
}
