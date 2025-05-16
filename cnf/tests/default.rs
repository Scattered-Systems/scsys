/*
    Appellation: default <test>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

#[test]
fn compiles() {
    fn adder<A, B, C>(a: A, b: B) -> C
    where
        A: core::ops::Add<B, Output = C>,
    {
        a + b
    }

    assert_eq!(adder(1, 2), 3);
    assert_ne!(adder(1f64, 0f64), 3f64);
}
