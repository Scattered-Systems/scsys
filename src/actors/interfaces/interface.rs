/*
    Appellation: interface <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/

pub trait InterfaceSpec<Cnf, Con, Data>
where
    Self:,
{
    fn appellation(&self, name: String, label: Vec<String>) -> (String, Vec<String>)
        where
            Self: Sized,
    {
        (name, label)
    }
    fn client(&self) -> Result<Self, crate::BoxError>
        where
            Self: Sized;
    fn configure(&self, pattern: &str) -> Result<crate::DefaultConfigBuilder, config::ConfigError>
        where
            Self: Sized,
    {
        let mut builder = config::Config::builder();
        builder = builder.add_source(crate::collect_config_files(pattern, false));
        Ok(builder)
    }
}

#[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct InterfaceAppellation {
    pub name: String,
    pub slug: String,
}

impl InterfaceAppellation {
    fn constructor(name: String, slug: String) -> Result<Self, crate::BoxError> {
        Ok(Self { name, slug })
    }
    pub fn init() -> Self {
        Self::from("")
    }
    pub fn from(name: &str) -> Self {
        Self::new(name.to_string())
    }
    pub fn new(name: String) -> Self {
        match Self::constructor(name.clone(), name.to_lowercase().clone()) {
            Ok(v) => v,
            Err(e) => {
                panic!("Interface Error: {}", e)
            }
        }
    }
}

#[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Interface {
    pub id: u64,
    pub name: String,
    pub slug: String,
}

impl Interface {
    pub fn new(id: u64, name: String, slug: String) -> Self {
        Self { id, name, slug }
    }
}