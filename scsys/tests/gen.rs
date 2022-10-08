#[cfg(test)]
mod tests {
    use scsys::prelude::{DigitGenerator, StringGenerator};

    #[test]
    fn test_generate_number_default() {
        assert_ne!(
            DigitGenerator::<f64>::default(),
            DigitGenerator::<f64>::default()
        )
    }

    #[test]
    fn test_generate_alpha_default() {
        assert_ne!(StringGenerator::default(), StringGenerator::default())
    }
}