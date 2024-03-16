#[cfg(test)]

pub fn multiply<A, B, C>(a: A, b: B) -> C
where
    A: std::ops::Mul<B, Output = C>,
{
    a * b
}

pub fn square<A>(a: A) -> A
where
    A: std::ops::Mul<Output = A> + Copy,
{
    a * a
}

#[test]
fn compiles() {
    assert_eq!(multiply(2, 2), 4);
    assert_eq!(multiply(3_f64, 3_f64), square(3_f64));
    assert_ne!(multiply(2.0, 3.0), -6.0);
}
