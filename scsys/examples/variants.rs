/*
    Appellation: variants <example>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

fn main() -> scsys::Result<()> {
    let a = Something::a();
    println!("Variant: {a:?}");
    let b = Something::b(42);
    println!("Variant: {b:?}");
    let c = Something::c(1, 2);
    println!("Variant: {c:?}");
    Ok(())
}

#[derive(Clone, Copy, Debug, scsys::VariantConstructors)]
pub enum Something {
    A,
    B(usize),
    C { x: usize, y: usize },
}
