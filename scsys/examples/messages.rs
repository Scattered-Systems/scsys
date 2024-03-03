use scsys::core::id::ids::AtomicId;
use scsys::prelude::Message;

fn main() {
    let id = AtomicId::new();
    println!("{}\n{}", id.clone(), id.to_string());
    println!("{:?}", Message::<String>::default());
}
