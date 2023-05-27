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

#[test]
fn test_extend_path() {
    let a = extend_path!["/tmp"; ["daemon.out", "daemon.err", "pid.test"]];
    let b = vec!["/tmp/daemon.out", "/tmp/daemon.err", "/tmp/pid.test"];

    assert_eq!(a, b)
}

#[test]
fn test_shared() {
    let a = shared!(String::from("Hello, World!"));
    assert!(a.lock().is_ok());
}