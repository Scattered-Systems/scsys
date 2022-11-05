/*
    Appellation: messages <module>
    Contributors: FL03 <jo3mccain@icloud.com> (https://gitlab.com/FL03)
    Description:
        ... Summary ...
*/
pub use self::message::*;

mod message;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_message_default() {
        let a = Message::from("Test message");
        let b = Message::default();
        assert_ne!(a, b.clone());
        assert_eq!(Message::from("").message, b.message)
    }
}
