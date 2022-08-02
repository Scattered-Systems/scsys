/*
    Appellation: variants <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
use rand::Rng;

/// A collection of time-related data structures
#[derive(Clone, Debug, PartialEq)]
pub enum Temporal {
    Bson(crate::BsonDateTime),
    Datetime(crate::DateTime<crate::Utc>),
    Timestamp(i64),
    Timezone(crate::Utc),
}

impl Temporal {
    pub fn now() -> crate::DateTime<crate::Utc> {
        crate::Utc::now()
    }
    pub fn bson_datetime() -> Self {
        Self::Bson(Self::now().into())
    }
    pub fn datetime() -> Self {
        Self::Datetime(Self::now())
    }
    pub fn timestamp() -> Self {
        Self::Timestamp(Self::now().timestamp())
    }
}

impl Default for Temporal {
    fn default() -> Self {
        Self::Timestamp(Self::now().timestamp())
    }
}

#[derive(Clone, Debug, Hash, PartialEq, crate::Deserialize, crate::Serialize)]
pub enum Id {
    Obj(crate::ObjectId),
    Std(u64),
    Str(String),
}

impl Id {
    fn random_u64() -> u64 {
        let mut rnd = rand::thread_rng();
        rnd.gen::<u64>()
    }
    pub fn generate_object_id() -> Self {
        Self::Obj(crate::ObjectId::new())
    }
    pub fn random_std() -> Self {
        Self::Std(Self::random_u64())
    }
}

impl Default for Id {
    fn default() -> Self {
        Self::Std(Self::random_u64())
    }
}

#[cfg(test)]
mod tests {
    use super::{Id, Temporal};

    #[test]
    fn test_ids() {
        let actual = Id::generate_object_id();
        let expected = actual.clone();
        assert_eq!(actual, expected)
    }

    #[test]
    fn test_temporal() {
        let actual = Temporal::default();
        let expected = actual.clone();
        assert_eq!(&actual, &expected)
    }
}
