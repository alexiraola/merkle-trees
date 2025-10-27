use crate::hash::Hash;
use crate::merkle::MerkleTree;

#[derive(Debug, Clone, Eq)]
pub struct Block {
    pub hash: Hash,
    pub previous_hash: Option<Hash>,
    pub transactions: Vec<String>,
    merkle_tree: MerkleTree,
    timestamp: u32,
}

impl Block {
    pub fn new(previous_hash: Hash, transactions: Vec<String>) -> Self {
        let merkle_tree = MerkleTree::new(transactions.clone());
        let timestamp = 0;

        let hash = Hash::from_str(&format!(
            "{}{}{}",
            merkle_tree.hash(),
            previous_hash,
            timestamp
        ));

        Self {
            // hash: merkle_tree.hash(),
            hash,
            previous_hash: Some(previous_hash),
            transactions,
            merkle_tree,
            timestamp,
        }
    }

    pub fn first(transactions: Vec<String>) -> Self {
        let merkle_tree = MerkleTree::new(transactions.clone());
        let timestamp = 0;

        let hash = Hash::from_str(&format!("{}{}", merkle_tree.hash(), timestamp));

        Self {
            hash,
            previous_hash: None,
            transactions,
            merkle_tree,
            timestamp,
        }
    }
}

impl PartialEq for Block {
    fn eq(&self, other: &Self) -> bool {
        return self.hash == other.hash;
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
        let block = Block::first(transactions);

        assert_eq!(block.previous_hash, None);
        assert_eq!(
            block.hash,
            "0ee2232ffc71c2c2c9bf07d1d09a0877b287694a99a85aba50e6202e1d131250"
        );
    }

    #[test]
    fn test_creates_block_with_previous() {
        let genesis = Block::first(vec![
            "Tx1".to_string(),
            "Tx2".to_string(),
            "Tx3".to_string(),
            "Tx4".to_string(),
        ]);

        let next_block = Block::new(
            genesis.hash.clone(),
            vec![
                "Tx5".to_string(),
                "Tx6".to_string(),
                "Tx7".to_string(),
                "Tx8".to_string(),
            ],
        );

        assert_eq!(next_block.previous_hash, Some(genesis.hash));
        assert_eq!(next_block.timestamp, 0);
        assert_eq!(
            next_block.hash,
            "ba607b6f1490f3257354f1831e41a759dafc716d96c230e3858fb6a53393be39"
        );
    }

    #[test]
    fn test_two_blocks_with_the_same_transactions_have_equal_hash() {
        let block = Block::first(vec![
            "Tx1".to_string(),
            "Tx2".to_string(),
            "Tx3".to_string(),
            "Tx4".to_string(),
        ]);

        let other_block = Block::first(vec![
            "Tx1".to_string(),
            "Tx2".to_string(),
            "Tx3".to_string(),
            "Tx4".to_string(),
        ]);

        assert_eq!(block, other_block);
    }

    #[test]
    fn test_two_blocks_with_the_different_transactions_have_not_equal_hash() {
        let block = Block::first(vec![
            "Tx1".to_string(),
            "Tx2".to_string(),
            "Tx3".to_string(),
            "Tx4".to_string(),
        ]);

        let other_block = Block::first(vec![
            "Tx1".to_string(),
            "Tx2".to_string(),
            "Tx3".to_string(),
            "Tx5".to_string(),
        ]);

        assert_ne!(block, other_block);
    }
}
