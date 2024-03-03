/*
    Appellation: specs <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

pub trait Generate {
    fn generate() -> Self;
}

pub trait GenerateIter: Generate {
    fn generate_iter(n: usize) -> Box<dyn Iterator<Item = Self>>
    where
        Self: Sized,
    {
        let iter = (0..n).map(|_| Self::generate());
        Box::new(iter)
    }
}

impl<T: Generate> GenerateIter for T {}
