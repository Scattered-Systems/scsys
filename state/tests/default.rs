/*
    Appellation: default <test>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

#[test]
fn lib_compiles() {
    fn adder<A, B, C>(a: A, b: B) -> C
    where
        A: core::ops::Add<B, Output = C>,
    {
        a + b
    }
    // ensure that 1 + 2 == 3
    assert_eq!(adder(1, 2), 3);
    // ensure that 1 + 0 != 4
    assert_ne!(adder(1.0, 0.0), 0.0);
}
