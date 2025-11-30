use crate::hash::Hash;
use crate::merkle::MerkleTree;

#[derive(Debug, Clone, Eq)]
pub struct Block {
    pub hash: Hash,
    pub previous_hash: Hash,
    pub transactions: Vec<String>,
    nonce: u32,
    merkle_tree: MerkleTree,
    timestamp: u32,
}

impl Block {
    pub fn new(previous_hash: Option<Hash>, transactions: Vec<String>, nonce: u32) -> Self {
        let merkle_tree = MerkleTree::new(transactions.clone());
        let timestamp = 0;
        let previous_hash = previous_hash.unwrap_or_default();

        let hash = Hash::from_str(&format!(
            "{}{}{}{}",
            merkle_tree.hash(),
            previous_hash.clone(),
            nonce,
            timestamp
        ));

        Self {
            // hash: merkle_tree.hash(),
            hash,
            previous_hash,
            transactions,
            nonce,
            merkle_tree,
            timestamp,
        }
    }

    pub fn first(transactions: Vec<String>, nonce: u32) -> Self {
        Self::new(None, transactions, nonce)
    }
}

impl PartialEq for Block {
    fn eq(&self, other: &Self) -> bool {
        self.hash == other.hash
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
        let block = Block::first(transactions, 0);

        assert_eq!(block.previous_hash, Hash::default());
        assert_eq!(
            block.hash,
            "13f349812c3129e764236036562c3ade6d5c7f6c5f3f70fa7c94d8cdd6daca5f"
        );
    }

    #[test]
    fn test_creates_block_with_previous() {
        let genesis = Block::first(
            vec![
                "Tx1".to_string(),
                "Tx2".to_string(),
                "Tx3".to_string(),
                "Tx4".to_string(),
            ],
            0,
        );

        let next_block = Block::new(
            Some(genesis.hash.clone()),
            vec![
                "Tx5".to_string(),
                "Tx6".to_string(),
                "Tx7".to_string(),
                "Tx8".to_string(),
            ],
            0,
        );

        assert_eq!(next_block.previous_hash, genesis.hash);
        assert_eq!(next_block.timestamp, 0);
        assert_eq!(
            next_block.hash,
            "65c751e21d19d8c6c1118b3032a669fc9b721b7810d18b26b1b47dbd1f941488"
        );
    }

    #[test]
    fn test_two_blocks_with_the_same_transactions_have_equal_hash() {
        let block = Block::first(
            vec![
                "Tx1".to_string(),
                "Tx2".to_string(),
                "Tx3".to_string(),
                "Tx4".to_string(),
            ],
            0,
        );

        let other_block = Block::first(
            vec![
                "Tx1".to_string(),
                "Tx2".to_string(),
                "Tx3".to_string(),
                "Tx4".to_string(),
            ],
            0,
        );

        assert_eq!(block, other_block);
    }

    #[test]
    fn test_two_blocks_with_the_different_transactions_have_not_equal_hash() {
        let block = Block::first(
            vec![
                "Tx1".to_string(),
                "Tx2".to_string(),
                "Tx3".to_string(),
                "Tx4".to_string(),
            ],
            0,
        );

        let other_block = Block::first(
            vec![
                "Tx1".to_string(),
                "Tx2".to_string(),
                "Tx3".to_string(),
                "Tx5".to_string(),
            ],
            0,
        );

        assert_ne!(block, other_block);
    }
}
