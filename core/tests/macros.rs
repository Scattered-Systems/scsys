use scsys_core::gsw;

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Something<T> {
    a: usize,
    b: u16,
    c: T,
}

impl<T> Something<T> {
    pub fn new(a: usize, b: u16, c: T) -> Self {
        Self { a, b, c }
    }
    gsw! {
        a: usize,
        b: u16,
    }

    gsw! {
        c: &T,
    }
}

#[test]
fn test_gsw() {
    let mut data = Something::new(0, 10, "Hello")
        .with_a(1)
        .with_b(2)
        .with_c("Hi");
    assert_eq!(data.a(), 1);
    assert_eq!(data.b(), 2);
    assert_eq!(*data.c(), "Hi");
    let next = data.set_a(0).set_b(0).set_c("Hello");
    assert_eq!(next.a(), 0);
    assert_eq!(next.b(), 0);
    assert_eq!(*next.c(), "Hello");
}
