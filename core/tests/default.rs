#[cfg(test)]

fn addition<A, B, C>(a: A, b: B) -> C
where
    A: std::ops::Add<B, Output = C>,
{
    a + b
}

#[test]
fn compiles() {
    assert_eq!(addition(1, 2), 3);
    assert_ne!(addition(1_f64, 0_f64), 3_f64);
}
