#[cfg(test)]
mod tests {
    use scsys::actors::generate::{generate_random_number, DigitGenerator, StringGenerator};

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

    #[test]
    fn test_random_number() {
        assert_ne!(
            generate_random_number::<f64>(),
            generate_random_number::<f64>()
        )
    }
}
