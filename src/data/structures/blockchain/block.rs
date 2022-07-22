/*
    Appellation: block <module>
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        ... Summary ...
*/
use crate::{BlockHs, BlockId, BlockNc, BlockTr, BlockTs};

#[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Block<Dt> {
    pub hash: BlockHs,
    pub id: BlockId,
    pub nonce: BlockNc,
    pub previous: BlockHs,
    pub timestamp: BlockTs,
    pub transactions: BlockTr<Dt>,
}

impl<Dt> Block<Dt> {
    fn constructor(
        hash: BlockHs,
        id: BlockId,
        nonce: BlockNc,
        previous: BlockHs,
        timestamp: BlockTs,
        transactions: BlockTr<Dt>,
    ) -> Result<Self, crate::BoxError> {
        Ok(Self {
            hash,
            id,
            nonce,
            previous,
            timestamp,
            transactions,
        })
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
