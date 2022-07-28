/*
    Appellation: files <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
#[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct FileHandler {
    pub path: String,
}

impl FileHandler {
    fn constructor(path: String) -> Result<Self, crate::BoxError> {
        Ok(Self { path })
    }
    pub fn new(path: String) -> Self {
        match Self::constructor(path) {
            Ok(v) => v,
            Err(e) => panic!("FileHandler Error: {}", e),
        }
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
