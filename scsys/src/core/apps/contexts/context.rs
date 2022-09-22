/*
    Appellation: context <module>
    Contributors: FL03 <jo3mccain@icloud.com> (https://gitlab.com/FL03)
*/


#[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Context<Cnf: Clone + PartialEq + serde::Deserialize<'a>> {
    pub settings: Cnf
}

impl<Cnf> Context<Cnf> {
    pub fn new(settings: Cnf) -> Self {
        Self { settings }
    }
}

#[cfg(test)]

mod tests {
    use super::Context;

    pub fn test_conetxt()
}