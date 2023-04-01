use scsys::prelude::{Id, Message};

fn main() {
    let id = Id::gen_robj();
    println!("{}\n{}", id.clone(), id.id_as_string());
    println!("{:?}", Message::<String>::default());
}
