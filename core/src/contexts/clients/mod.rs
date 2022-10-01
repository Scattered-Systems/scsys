/*
    Appellation: clients <module>
    Contributors: FL03 <jo3mccain@icloud.com> (https://github.com)
    Description:
        ... Summary ...
*/
pub use self::client::*;

mod client;

pub trait ClientSpec {
    fn architecture(&self) -> String;
}
