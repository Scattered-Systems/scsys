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
        .map(|path| config::File::from(path.unwrap()).required(required))
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let f = |x: usize, y: usize| x + y;
        assert_eq!(f(4, 2), 6)
    }
}
