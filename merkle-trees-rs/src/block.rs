use crate::hash::Hash;
use crate::merkle::MerkleTree;

#[derive(Debug, Clone)]
pub struct Block {
    pub hash: Hash,
    previous_hash: Option<Hash>,
    transactions: Vec<String>,
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
}
