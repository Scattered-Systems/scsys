#[cfg(test)]
#[test]
fn lib_compiles() {
    let f = |i: usize| i * i;

    assert_eq!(f(2), 4);
    assert_eq!(f(3), 9);
    assert_ne!(f(2), 3);
}
