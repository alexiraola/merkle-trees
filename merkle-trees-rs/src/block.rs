use crate::bits::DifficultyTarget;
use crate::block_header::BlockHeader;
use crate::hash::Hash;
use crate::merkle::MerkleTree;
use crate::timestamp::Timestamp;

#[derive(Debug, Clone, Eq)]
pub struct Block {
    pub header: BlockHeader,
    pub transactions: Vec<String>,
}

impl Block {
    pub fn new(
        previous_hash: Option<Hash>,
        transactions: Vec<String>,
        timestamp: Option<Timestamp>,
        nonce: u32,
    ) -> Self {
        let merkle_tree = MerkleTree::new(transactions.clone());

        let header = BlockHeader::new(
            256,
            previous_hash.unwrap_or_default(),
            merkle_tree.hash(),
            timestamp,
            DifficultyTarget::new(0x00, 0x00),
            nonce,
        );

        Self {
            header,
            transactions,
        }
    }

    pub fn hash(&self) -> Hash {
        self.header.hash()
    }

    pub fn genesis(transactions: Vec<String>, timestamp: Option<Timestamp>, nonce: u32) -> Self {
        Self::new(None, transactions, timestamp, nonce)
    }
}

impl PartialEq for Block {
    fn eq(&self, other: &Self) -> bool {
        self.hash() == other.hash()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creates_genesis_block() {
        let transactions = vec![
            "Tx1".to_string(),
            "Tx2".to_string(),
            "Tx3".to_string(),
            "Tx4".to_string(),
        ];
        let block = Block::genesis(transactions, Some(Timestamp::new(0)), 0);

        assert_eq!(block.header.previous_hash, Hash::default());
        assert_eq!(
            block.hash(),
            "84c32ec45ffb02449c58ddc80c8b58e51da1d5b630f0e18dfc63ac5983e16139"
        );
    }

    #[test]
    fn test_creates_block_with_previous() {
        let genesis = Block::genesis(
            vec![
                "Tx1".to_string(),
                "Tx2".to_string(),
                "Tx3".to_string(),
                "Tx4".to_string(),
            ],
            Some(Timestamp::new(0)),
            0,
        );

        let next_block = Block::new(
            Some(genesis.hash().clone()),
            vec![
                "Tx5".to_string(),
                "Tx6".to_string(),
                "Tx7".to_string(),
                "Tx8".to_string(),
            ],
            Some(Timestamp::new(0)),
            0,
        );

        assert_eq!(next_block.header.previous_hash, genesis.hash());
        assert_eq!(next_block.header.timestamp, Timestamp::new(0));
        assert_eq!(
            next_block.hash(),
            "0c9713b3c13b1301c5f108c27926aaa85fa4b2ddefca76e206916384de9c2811"
        );
    }

    #[test]
    fn test_two_blocks_with_the_same_transactions_have_equal_hash() {
        let block = Block::genesis(
            vec![
                "Tx1".to_string(),
                "Tx2".to_string(),
                "Tx3".to_string(),
                "Tx4".to_string(),
            ],
            Some(Timestamp::new(0)),
            0,
        );

        let other_block = Block::genesis(
            vec![
                "Tx1".to_string(),
                "Tx2".to_string(),
                "Tx3".to_string(),
                "Tx4".to_string(),
            ],
            Some(Timestamp::new(0)),
            0,
        );

        assert_eq!(block, other_block);
    }

    #[test]
    fn test_two_blocks_with_the_different_transactions_have_not_equal_hash() {
        let block = Block::genesis(
            vec![
                "Tx1".to_string(),
                "Tx2".to_string(),
                "Tx3".to_string(),
                "Tx4".to_string(),
            ],
            Some(Timestamp::new(0)),
            0,
        );

        let other_block = Block::genesis(
            vec![
                "Tx1".to_string(),
                "Tx2".to_string(),
                "Tx3".to_string(),
                "Tx5".to_string(),
            ],
            Some(Timestamp::new(0)),
            0,
        );

        assert_ne!(block, other_block);
    }
}
