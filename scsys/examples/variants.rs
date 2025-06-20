/*
    Appellation: derive <example>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

fn main() -> scsys::Result<()> {
    let variant = Something::a();
    println!("Variant: {:?}", variant);
    let variant = Something::b(42);
    println!("Variant: {:?}", variant);
    let variant = Something::c(1, 2);
    println!("Variant: {:?}", variant);
    Ok(())
}

#[derive(Clone, Copy, Debug, scsys::VariantConstructors)]
pub enum Something {
    A,
    B(usize),
    C { x: usize, y: usize },
}
