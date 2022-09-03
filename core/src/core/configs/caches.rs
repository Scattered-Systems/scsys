/*
    Appellation: caches <module>
    Contributors: FL03 <jo3mccain@icloud.com> (https://github.com)
    Description:
        ... Summary ...
*/


#[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Cache {
    pub name: String,
    pub uri: String
}