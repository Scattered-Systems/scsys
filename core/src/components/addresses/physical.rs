/*
    Appellation: locational <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/

#[derive(Clone, Debug, PartialEq, crate::Deserialize, crate::Serialize)]
pub enum PhysicalAddresses {
    LatLong(LatLongPair),
    Mailing(MailingAddress),
}

#[derive(Clone, Debug, PartialEq, crate::Deserialize, crate::Serialize)]
pub struct LatLongPair {
    pub latitude: Latitude,
    pub longitude: Longitude,
}

impl LatLongPair {
    fn constructor(latitude: Latitude, longitude: Longitude) -> Self {
        Self {
            latitude,
            longitude,
        }
    }
    pub fn new(latitude: Latitude, longitude: Longitude) -> Self {
        Self::constructor(latitude, longitude)
    }
}

impl Default for LatLongPair {
    fn default() -> Self {
        Self::new(Latitude::default(), Longitude::default())
    }
}

#[derive(Clone, Debug, PartialEq, crate::Deserialize, crate::Serialize)]
pub struct Latitude(f64);

impl Default for Latitude {
    fn default() -> Self {
        Self(0f64)
    }
}

#[derive(Clone, Debug, PartialEq, crate::Deserialize, crate::Serialize)]
pub struct Longitude(f64);

impl Default for Longitude {
    fn default() -> Self {
        Self(0f64)
    }
}

#[derive(Clone, Debug, Hash, PartialEq, crate::Deserialize, crate::Serialize)]
pub struct MailingAddress {
    pub street: String,
    pub city: String,
    pub state: String,
    pub zip_code: usize,
}

impl MailingAddress {
    fn constructor(street: String, city: String, state: String, zip_code: usize) -> Self {
        Self {
            street,
            city,
            state,
            zip_code,
        }
    }
    pub fn new(street: String, city: String, state: String, zip_code: usize) -> Self {
        Self::constructor(street, city, state, zip_code)
    }
}

impl Default for MailingAddress {
    fn default() -> Self {
        Self::new("".to_string(), "".to_string(), "".to_string(), 000000)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let f = |x: usize, y: usize| x + y;
        assert_eq!(f(4, 2), 6)
    }
}
