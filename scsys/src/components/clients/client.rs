/*
    Appellation: client <module>
    Contributors: FL03 <jo3mccain@icloud.com> (https://github.com)
    Description:
        ... Summary ...
*/



#[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct OAuthClient {
    pub endpoint: Option<String>,
    pub data: Option<Vec<String>>
}