/*
    Appellation: catalysts <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
pub use self::{catalyst::*, specs::*};

pub(crate) mod catalyst;

pub(crate) mod specs {
    use std::convert::Into;

    pub trait Transformable<T>: Into<T> {}

    pub trait Converter<S, T>
    where
        S: Clone + Transformable<T>,
    {
        fn data(&self) -> &[S];
        fn catalyst(&mut self) -> Vec<T> where S: Iterator{
            self.data().to_owned().iter().cloned().map(|i| i.into()).collect::<Vec<_>>()
        }
    }

    pub trait Transformation<S> {
        fn data(&self) -> Vec<S>;
        fn transform<T>(
            &self,
            catalyst: fn(&S) -> T,
        ) -> Result<Vec<T>, Box<dyn std::error::Error>> {
            let res = self.data().iter().map(catalyst).collect::<Vec<_>>();
            Ok(res)
        }
    }
}
