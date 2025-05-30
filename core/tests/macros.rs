use scsys_core::gsw;

#[derive(Clone, Debug, Default)]
pub struct Sample<T = String> {
    pub(crate) apple: usize,
    pub(crate) block: f32,
    pub(crate) store: Vec<u8>,
    pub(crate) cont: T,
}

impl<T> Sample<T> {
    pub fn new(cont: T) -> Self {
        Self {
            apple: 0,
            block: 0.0,
            store: Vec::new(),
            cont,
        }
    }

    gsw! {
        apple: usize,
        block: f32,
    }
    gsw! {
        cont: &T,
        store: &Vec<u8>,
    }
}

#[test]
fn test_sample_gsw_impls() {
    // validate builders
    let mut sample = Sample::new("hello world".to_string())
        .with_apple(10)
        .with_block(3.14);
    // verify setters
    sample.set_store(vec![1, 2, 3]);
    // verify the getters
    assert_eq!(sample.apple(), 10);
    assert_eq!(sample.block(), 3.14);
    assert_eq!(sample.cont(), "hello world");
    assert_eq!(sample.store(), &vec![1, 2, 3]);
    // verify the mutable getters
    sample.store_mut().push(u8::MAX);
    assert!(sample.store().last() == Some(&u8::MAX));
}
