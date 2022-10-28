#[cfg(test)]
mod tests {
    use scsys_core::Timestamp;

    #[test]
    fn test_timestamp() {
        let a = Timestamp::default();
        let b = a.clone();
        b.chrono_to_bson(Timestamp::now());
        assert_eq!(a, b)
    }

    #[test]
    fn test_timestamp_default() {
        let actual = Timestamp::default();
        let expected = actual.clone();
        assert_eq!(actual, expected)
    }
}
