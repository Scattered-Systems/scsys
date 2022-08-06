/*
    Appellation: utils <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/

// Gather configuration files following the specified pattern and collect them into a vector
pub fn collect_config_files(pattern: &str, required: bool) -> crate::ConfigFileVec {
    glob::glob(pattern)
        .unwrap()
        .map(|path| crate::ConfigFile::from(path.unwrap()).required(required))
        .collect::<Vec<_>>()
}

/// Consolidate resources required for generating random numbers
pub fn random_number<T>() -> T
where
    rand::distributions::Standard: rand::prelude::Distribution<T>,
{
    use rand::Rng;
    let mut rnd = rand::thread_rng();
    rnd.gen::<T>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_collect_config_files() {
        let actual = collect_config_files("**/*.config.*", false);
        let expected = actual.clone().len();
        assert_eq!(actual.len(), expected)
    }

    #[test]
    fn test_random_number() {
        assert_ne!(random_number::<f64>(), random_number::<f64>())
    }
}
