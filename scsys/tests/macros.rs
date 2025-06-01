/*
    appellation: macros <test>
    authors: @FL03
*/

#[test]
fn test_get_field_macro() {
    const fn sample() -> usize {
        42
    }
    assert_eq!(42, sample());
}
