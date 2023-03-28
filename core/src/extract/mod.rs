/*
    Appellation: extract <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
pub use self::extractor::*;

mod extractor;

pub trait Extract<T> {
    fn extract(&self) -> Vec<T>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extractor_comma() {
        let extractor = Extractor::new("[0, 0, 0, 0]".to_string()).break_at(',');
        assert_eq!(extractor.extract::<u8>(), vec![0, 0, 0, 0])
    }

    #[test]
    fn test_extractor_period() {
        let extractor = Extractor::new("0.0.0.0".to_string()).break_at('.');
        assert_eq!(extractor.extract::<u8>(), vec![0, 0, 0, 0])
    }
}
