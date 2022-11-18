/*
    Appellation: catalyst <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/

pub(crate) mod specs {

    pub trait Catalyst<S: std::convert::Into<T>, T> {
        fn catalyst(&self, data: &S) -> T;
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
