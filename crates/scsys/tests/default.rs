/*
    Appellation: default <test>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

fn addition<A, B, C>(a: A, b: B) -> C
where
    A: core::ops::Add<B, Output = C>,
{
    a + b
}

#[test]
fn compiles() {
    assert_eq!(addition(1, 2), 3);
    assert_ne!(addition(1f64, 0f64), 3f64);
}
