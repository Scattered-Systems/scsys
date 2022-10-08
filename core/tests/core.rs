#[cfg(test)]
mod tests {
    use scsys_core::{Error, Event, Message, Timestamp, states::State};

    #[test]
    fn test_error_default() {
        let actual = Error::default();
        let expected = Error::try_from("default").ok().unwrap();
        assert_eq!(actual, expected)
    }

    #[test]
    fn test_event_default() {
        let a = Event::default();
        let b = Event::try_from("event").expect("");
        assert_eq!(a, b)
    }

    #[test]
    fn test_message_default() {
        let a = Message::from("Test message");
        let b = Message::default();
        assert_ne!(a, b.clone());
        assert_eq!(Message::from("").message, b.message)
    }

    #[test]
    fn test_state_default() {
        let actual = State::new("message".to_string(), "test");
        let expected = actual.clone();
        assert_eq!(actual, expected)
    }

    #[test]
    fn test_timestamp() {
        let a = Timestamp::default();
        let b = a.clone();
        b.chrono_to_bson(Timestamp::now());
        assert_eq!(a, b)
    }

    #[test]
    fn test_timestamp_default() {
        let actual = Timestamp::default();
        let expected = actual.clone();
        assert_eq!(actual, expected)
    }
}