/*
    Appellation: specs <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/

/// Interface for identifiable data-structures
pub trait Identifiable<Id> {
    fn id(&self) -> &Id;
}

/// Interface for nameable data-structures
pub trait Nameable {
    fn name(&self) -> String;
    fn slug(&self) -> String {
        self.name().to_lowercase().replace(" ", "-")
    }
}

/// Interface for time-related data-structures
pub trait Temporal {
    fn timestamp(&self) -> i64;
}
