/*
Appellation: generate <module>
Creator: FL03 <jo3mccain@icloud.com>
Description:
... Summary ...
*/
pub use self::{alphabet::*, utils::*};

pub(crate) mod alphabet;

pub(crate) mod utils {
    use rand::{distributions::Alphanumeric, Rng};

    pub fn generate_random_string(len: usize) -> String {
        rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(len)
            .map(char::from)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_alpha_default() {
        assert_ne!(StringGenerator::default(), StringGenerator::default())
    }
}
