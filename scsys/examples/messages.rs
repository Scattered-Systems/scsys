/*
    Appellation: messages <example>
    Creator: FL03 <jo3mccain@icloud.com>
*/
use scsys::core::id::AtomicId;
use scsys::prelude::Message;

fn main() {
    let id = AtomicId::new();
    println!("{}\n{}", id.clone(), id.to_string());
    println!("{:?}", Message::<String>::default());
}
