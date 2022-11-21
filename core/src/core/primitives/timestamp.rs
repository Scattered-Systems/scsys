/*
    Appellation: timestamp <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use crate::{timestamp, Temporal};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Timestamp(pub i64);

impl Timestamp {
    pub fn new() -> Self {
        Self(timestamp())
    }
    pub fn ts() -> i64 {
        chrono::Utc::now().timestamp()
    }
}

impl Temporal for Timestamp {
    fn timestamp(&self) -> i64 {
        self.0
    }
}

impl std::convert::From<&Timestamp> for Timestamp {
    fn from(ts: &Timestamp) -> Self {
        Self(ts.0)
    }
}

impl std::convert::From<i64> for Timestamp {
    fn from(ts: i64) -> Self {
        Self(ts)
    }
}

impl std::convert::From<Timestamp> for i64 {
    fn from(ts: Timestamp) -> i64 {
        ts.0
    }
}

impl Default for Timestamp {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for Timestamp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap())
    }
}

#[cfg(test)]
mod tests {
    use crate::{Temporal, Timestamp};

    #[test]
    fn test_timestamp() {
        let a = Timestamp::default();
        let b = a.clone();
        b.chrono_to_bson(Timestamp::datetime());
        assert_eq!(a, b)
    }

    #[test]
    fn test_timestamp_default() {
        let m = Timestamp::default();
        let a = Timestamp::from(&m);
        let b: i64 = Timestamp::from(&a).into();
        assert_eq!(&m, &a);
        assert_eq!(&a.timestamp(), &b);
    }
}
