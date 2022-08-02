/*
    Appellation: parser <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
#[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub enum ParserState {
    Activating,
    Parsing,
    Skipping,
    Start,
    Stop,
}

impl Default for ParserState {
    fn default() -> Self {
        Self::Stop
    }
}

#[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Parser {
    pub state: ParserState,
    pub data: Vec<String>,
}

impl Parser {
    fn constructor(state: ParserState, data: Vec<String>) -> Self {
        Self { state, data }
    }
    pub fn new(data: Vec<String>) -> Self {
        Self::constructor(ParserState::Start, data)
    }
}

impl Default for Parser {
    fn default() -> Self {
        todo!()
    }
}
