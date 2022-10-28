/*
    Appellation: actor <module>
    Contributors: FL03 <jo3mccain@icloud.com> (https://github.com)
    Description:
        ... Summary ...
*/
use crate::components::identities::Appellation;

pub trait Transformation<S> {
    fn data(&self) -> Vec<S>;
    fn transform<T>(&self, catalyst: fn(&S) -> T) -> crate::BoxResult<Vec<T>> {
        let res = self.data().iter().map(|i| catalyst(i)).collect::<Vec<_>>();
        Ok(res)
    }
}
pub trait ActorSpec {
    fn appellation(&self) -> Appellation;
    fn justification(&self) -> serde_json::Value;
}
