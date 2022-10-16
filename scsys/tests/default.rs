#[cfg(test)]

mod tests {
    use scsys::components::messages::Message;

    #[test]
    fn test_message_default() {
        let a = Message::from("Test message");
        let b = Message::default();
        assert_ne!(a, b.clone());
        assert_eq!(Message::from("").message, b.message)
    }
}
