/*
    Appellation: times <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
pub use self::{timestamp::*, utils::*};

pub(crate) mod timestamp;

pub(crate) mod utils {
    use chrono::{DateTime, TimeZone, Utc};

    pub fn chrono_datetime_now() -> DateTime<Utc> {
        Utc::now()
    }

    pub fn ts_rfc3339() -> String {
        Utc::now().to_rfc3339()
    }

    pub fn chrono_into_bson<T: TimeZone>(data: DateTime<T>) -> bson::DateTime {
        bson::DateTime::from_chrono(data)
    }

    pub fn timestamp() -> i64 {
        Utc::now().timestamp()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_timestamp() {
        let boundary = Timestamp::now();
        let a = Timestamp::from(&boundary);
        let b = Timestamp::try_from(boundary.to_rfc3339()).ok().unwrap();
        assert_eq!(&a, &b);
    }
}
