/*
    Appellation: database <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#[doc(inline)]
pub use self::db_uri::*;

pub(crate) mod db_uri;

///
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(default, rename_all = "snake_case")
)]
pub struct StandardDatabaseConfig {
    /// The database connection URI.
    pub connection: DatabaseUriSchema,
    pub max_connections: u32,
    pub pool_size: u32,
}

impl StandardDatabaseConfig {
    const DEFAULT_MAX_CONNECTIONS: u32 = 10;
    const DEFAULT_DB_POOL_SIZE: u32 = 10;

    /// returns a new instance with defaults
    pub fn new() -> Self {
        Self::from_schema(Default::default())
    }
    /// returns a new instance from the given connection schema
    pub fn from_schema(connection: DatabaseUriSchema) -> Self {
        Self {
            connection,
            max_connections: StandardDatabaseConfig::DEFAULT_MAX_CONNECTIONS,
            pool_size: StandardDatabaseConfig::DEFAULT_DB_POOL_SIZE,
        }
    }
    /// attempts to create a new instance of the configuration using the provide uri
    pub fn from_uri(uri: &str) -> crate::Result<Self> {
        use core::str::FromStr;
        let schema = DatabaseUriSchema::from_str(uri)?;
        // create a new instance from the default schema
        let inst = Self::from_schema(schema);
        Ok(inst)
    }
    /// Returns the database connection URI.
    pub const fn connection(&self) -> &DatabaseUriSchema {
        &self.connection
    }

    /// Returns the database connection URL.
    pub const fn pool_size(&self) -> u32 {
        self.pool_size
    }
}

impl Default for StandardDatabaseConfig {
    fn default() -> Self {
        Self::new()
    }
}
