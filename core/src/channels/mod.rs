/*
    Appellation: channels <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

pub trait Reciever<T> {
    fn recv(&self) -> Option<T>;
}
