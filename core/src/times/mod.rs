/*
    Appellation: times <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
pub use self::{specs::*, timestamp::*, utils::*};

pub(crate) mod timestamp;

pub(crate) mod specs {
    use crate::{chrono_into_bson, ChronoDateTime};
    use chrono::Utc;

    pub trait TemporalExt {}

    /// Interface for time-related data-structures
    pub trait Temporal {
        fn chrono_to_bson(&self, data: ChronoDateTime) -> bson::DateTime {
            chrono_into_bson::<Utc>(data)
        }
        fn bson_datetime() -> bson::DateTime {
            chrono_into_bson::<chrono::Utc>(chrono::Utc::now())
        }
        fn datetime() -> ChronoDateTime {
            chrono::Utc::now()
        }
        fn gen_rfc3339() -> String {
            chrono::Utc::now().to_rfc3339()
        }
        fn ts() -> i64 {
            chrono::Utc::now().timestamp()
        }
        fn timestamp(&self) -> i64; // Recall an objects time of creation
    }
}

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
