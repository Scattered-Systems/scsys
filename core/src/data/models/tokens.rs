/*
    Appellation: tokens <module>
    Contributors: FL03 <jo3mccain@icloud.com> (https://github.com)
    Description:
        ... Summary ...
*/

#[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Tokens {
    pub access_token: String,
    pub token_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>
}

impl Tokens {
    pub fn new(access_token: String, token_type: String, username: Option<String>) -> Self {
        Self { access_token, token_type, username }
    }
}