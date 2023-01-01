/*
    Appellation: s3 <storage>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct S3Credential {
    pub access_key: String,
    pub secret_key: String,
}

impl S3Credential {
    pub fn new(access_key: String, secret_key: String) -> Self {
        Self {
            access_key,
            secret_key,
        }
    }
    pub fn from_env(
        access: Option<&str>,
        secret: Option<&str>,
    ) -> Result<Self, std::env::VarError> {
        let access = match access {
            Some(v) => std::env::var(v),
            None => std::env::var("S3_ACCESS_KEY"),
        };
        let secret = match secret {
            Some(v) => std::env::var(v),
            None => std::env::var("S3_SECRET_KEY"),
        };
        Ok(Self::new(access?, secret?))
    }
}

impl std::convert::From<(&str, &str)> for S3Credential {
    fn from(data: (&str, &str)) -> Self {
        Self::new(data.0.to_string(), data.1.to_string())
    }
}

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct S3Region {
    pub endpoint: String,
    pub region: String,
}

impl S3Region {
    pub fn new(endpoint: String, region: String) -> Self {
        Self { endpoint, region }
    }
    pub fn from_env(
        endpoint: Option<&str>,
        region: Option<&str>,
    ) -> Result<Self, std::env::VarError> {
        let endpoint = match endpoint {
            Some(v) => std::env::var(v),
            None => std::env::var("S3_ENDPOINT"),
        };
        let region = match region {
            Some(v) => std::env::var(v),
            None => std::env::var("S3_REGION"),
        };
        Ok(Self::new(endpoint?, region?))
    }
}

impl std::convert::From<(&str, &str)> for S3Region {
    fn from(data: (&str, &str)) -> Self {
        Self::new(data.0.to_string(), data.1.to_string())
    }
}

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct S3Configuration {
    pub credential: Option<S3Credential>,
    pub region: Option<S3Region>,
}

impl S3Configuration {
    pub fn new(credential: Option<S3Credential>, region: Option<S3Region>) -> Self {
        Self { credential, region }
    }
    pub fn from_env(
        access: Option<&str>,
        secret: Option<&str>,
        endpoint: Option<&str>,
        region: Option<&str>,
    ) -> Self {
        let cred = S3Credential::from_env(access, secret).unwrap_or_default();
        let region = S3Region::from_env(endpoint, region).unwrap_or_default();
        Self::new(Some(cred), Some(region))
    }
}
