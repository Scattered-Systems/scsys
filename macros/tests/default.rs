#[cfg(test)]
use scsys_macros::*;

#[test]
fn compiles() {
    let f = |i: usize| i * i;

    assert_eq!(f(2), 4);
    assert_eq!(f(3), 9);
    assert_ne!(f(2), 3);
}

#[test]
fn test_string() {
    let f = "something";
    assert_eq!(string!(f), string!("something"));
}