/*
    Appellation: tokens <module>
    Contributors: FL03 <jo3mccain@icloud.com> (https://github.com)
    Description:
        ... Summary ...
*/
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Tokens {
    pub access_token: String,
    pub token_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
}

impl Tokens {
    pub fn new(access_token: String, token_type: String, username: Option<String>) -> Self {
        Self {
            access_token,
            token_type,
            username,
        }
    }
}
