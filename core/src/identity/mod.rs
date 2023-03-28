/*
    Appellation: identity <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
pub use self::{builder::*, ids::*};

mod builder;
mod ids;

use bson::oid::ObjectId;

pub type BoxedId = Box<dyn Identifier>;

/// Interface for identifiable data-structures
pub trait Identifiable<Id: Identifier> {
    fn id(&self) -> &Id;
}

/// Interface for identifier data-structures
pub trait Identifier {}

impl Identifier for i8 {}

impl Identifier for i32 {}

impl Identifier for i64 {}

impl Identifier for i128 {}

impl Identifier for isize {}

impl Identifier for u8 {}

impl Identifier for u32 {}

impl Identifier for u64 {}

impl Identifier for u128 {}

impl Identifier for usize {}

impl Identifier for f32 {}

impl Identifier for f64 {}

impl Identifier for String {}

impl Identifier for &str {}

impl Identifier for char {}

impl Identifier for ObjectId {}
