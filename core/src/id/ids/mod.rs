/*
   Appellation: ids <mod>
   Contrib: FL03 <jo3mccain@icloud.com>
*/
//! # ids
pub use self::atomic::*;

pub(crate) mod atomic;

pub use bson::oid::ObjectId;

impl super::Identifier for ObjectId {}
