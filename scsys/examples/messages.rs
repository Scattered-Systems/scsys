/*
    Appellation: messages <example>
    Creator: FL03 <jo3mccain@icloud.com>
*/
use scsys::prelude::{AtomicId, Message};

fn main() {
    let id = AtomicId::new();
    println!("{}\n{}", id.clone(), id.to_string());
    println!("{:?}", Message::<String>::default());
}
