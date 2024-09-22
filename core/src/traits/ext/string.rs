/*
    Appellation: string <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
#![cfg(feature = "alloc")]

use alloc::string::String;

pub trait StringExt {
    /// Remove the first and last charecters of a string
    fn remove_fnl(&self) -> &str;
}

impl StringExt for str {
    fn remove_fnl(&self) -> &str {
        &self[1..self.len() - 1]
    }
}

pub trait StringFmt {
    fn snake_case(&self) -> String;

    fn title_case(&self) -> String;
}
