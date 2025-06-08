/*
    Appellation: connection <module>
    Contrib: @FL03
*/

/// A standard database connection url schema
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize, serde::Serialize),
    serde(default, rename_all = "snake_case")
)]
pub struct DatabaseUriSchema {
    pub(crate) prefix: String,
    pub(crate) host: String,
    pub(crate) port: u16,
    pub(crate) user: String,
    pub(crate) password: String,
    pub(crate) database: String,
}

impl DatabaseUriSchema {
    pub fn from_parts(
        prefix: impl ToString,
        host: impl ToString,
        port: u16,
        user: impl ToString,
        password: impl ToString,
        database: impl ToString,
    ) -> Self {
        Self {
            prefix: prefix.to_string(),
            host: host.to_string(),
            port,
            user: user.to_string(),
            password: password.to_string(),
            database: database.to_string(),
        }
    }
    /// returns a new instance pre-configured with the postgresql prefix.
    pub fn postgresql(
        host: impl ToString,
        port: u16,
        user: impl ToString,
        password: impl ToString,
        database: impl ToString,
    ) -> Self {
        Self::from_parts("postgresql", host, port, user, password, database)
    }
    /// returns a string representation of the database URL.
    pub fn as_uri_string(&self) -> String {
        format!(
            "{prefix}://{user}:{password}@{host}:{port}/{database}",
            prefix = self.prefix,
            user = self.user,
            password = self.password,
            host = self.host,
            port = self.port,
            database = self.database
        )
    }
    /// returns a reference to the driver
    pub const fn prefix(&self) -> &String {
        &self.prefix
    }
    /// returns a mutable reference to the driver
    pub const fn prefix_mut(&mut self) -> &mut String {
        &mut self.prefix
    }
    /// returns a reference to the host
    #[inline]
    pub fn host(&self) -> &String {
        &self.host
    }
    /// returns a mutable reference to the host
    pub const fn host_mut(&mut self) -> &mut String {
        &mut self.host
    }
    /// returns a copy of the configured port
    pub const fn port(&self) -> u16 {
        self.port
    }
    /// returns a mutable reference to the port
    pub const fn port_mut(&mut self) -> &mut u16 {
        &mut self.port
    }
    /// returns a reference to the username
    pub const fn user(&self) -> &String {
        &self.user
    }
    /// returns a mutable reference to the username
    pub const fn user_mut(&mut self) -> &mut String {
        &mut self.user
    }
    /// returns a reference to the password
    pub const fn password(&self) -> &String {
        &self.password
    }
    /// returns a mutable reference to the password
    pub const fn password_mut(&mut self) -> &mut String {
        &mut self.password
    }
    /// returns a reference to the database name
    pub const fn database(&self) -> &String {
        &self.database
    }
    /// returns a mutable reference to the database name
    pub const fn database_mut(&mut self) -> &mut String {
        &mut self.database
    }
    /// update the configured database and return a mutable reference to the current instance
    #[inline]
    pub fn set_database<U: ToString>(&mut self, value: U) -> &mut Self {
        *self.database_mut() = value.to_string();
        self
    }
    /// update the configured hostname and return a mutable reference to the current instance
    #[inline]
    pub fn set_host(&mut self, host: impl ToString) -> &mut Self {
        self.host = host.to_string();
        self
    }
    /// update the configured password and return a mutable reference to the current instance
    #[inline]
    pub fn set_password<U: ToString>(&mut self, value: U) -> &mut Self {
        *self.password_mut() = value.to_string();
        self
    }
    /// update the configured port and return a mutable reference to the current instance
    #[inline]
    pub fn set_port(&mut self, port: u16) -> &mut Self {
        self.port = port;
        self
    }
    /// update the configured prefix and return a mutable reference to the current instance
    #[inline]
    pub fn set_prefix<U: ToString>(&mut self, value: U) -> &mut Self {
        *self.prefix_mut() = value.to_string();
        self
    }
    /// update the configured username and return a mutable reference to the current instance
    #[inline]
    pub fn set_user<U: ToString>(&mut self, value: U) -> &mut Self {
        *self.user_mut() = value.to_string();
        self
    }
    /// consumes the current instance to create another with the given database name
    pub fn with_database<U: ToString>(self, value: U) -> Self {
        Self {
            database: value.to_string(),
            ..self
        }
    }
    /// consumes the current instance to create another with the given host
    pub fn with_host<U: ToString>(self, value: U) -> Self {
        Self {
            host: value.to_string(),
            ..self
        }
    }
    /// consumes the current instance to create another with the given password
    pub fn with_password<U: ToString>(self, value: U) -> Self {
        Self {
            password: value.to_string(),
            ..self
        }
    }
    /// consumes the current instance to create another with the given port
    pub fn with_port(self, value: u16) -> Self {
        Self {
            port: value,
            ..self
        }
    }
    /// consumes the current instance to create another with the given prefix
    pub fn with_prefix<U: ToString>(self, value: U) -> Self {
        Self {
            prefix: value.to_string(),
            ..self
        }
    }
    /// consumes the current instance to create another with the given username
    pub fn with_user<U: ToString>(self, value: U) -> Self {
        Self {
            user: value.to_string(),
            ..self
        }
    }
}

impl Default for DatabaseUriSchema {
    fn default() -> Self {
        Self::postgresql("localhost", 5432, "postgres", "postgres", "postgres")
    }
}

impl core::fmt::Display for DatabaseUriSchema {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.write_str(&self.as_uri_string())
    }
}

#[cfg(feature = "url")]
impl core::str::FromStr for DatabaseUriSchema {
    type Err = crate::ConfigError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut schema: DatabaseUriSchema = Self::default();
        // parse the URL using the `url` crate
        let url = url::Url::parse(s)?;
        // try extracting the components from the URL
        schema.host = url
            .host_str()
            .ok_or(url::ParseError::EmptyHost)?
            .to_string();
        schema.port = url.port().unwrap_or(5432);
        schema.user = url.username().to_string();
        schema.password = url.password().unwrap_or("").to_string();
        schema.database = url.path().trim_start_matches('/').to_string();
        schema.prefix = url.scheme().to_string();
        // return the parsed object
        Ok(schema)
    }
}
