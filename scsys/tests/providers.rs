#[cfg(test)]
mod tests {
    use scsys::components::providers::S3Configuration;

    #[test]
    fn test_s3_setup() {
        let a = S3Configuration::from_env(None, None, None, None);
        let b = a.clone();

        assert_eq!(a, b)
    }
}
