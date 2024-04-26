/*
    Appellation: string <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/

pub trait StringExt {
    /// Remove the first and last charecters of a string
    fn remove_fnl(&self) -> &str;

    
}

impl StringExt for str {
    fn remove_fnl(&self) -> &str {
        &self[1..self.len() - 1]
    }
}
