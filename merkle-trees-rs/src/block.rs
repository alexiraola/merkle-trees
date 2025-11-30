use crate::hash::Hash;
use crate::merkle::MerkleTree;

#[derive(Debug, Clone, Eq)]
pub struct Block {
    pub hash: Hash,
    pub previous_hash: Option<Hash>,
    pub transactions: Vec<String>,
    nonce: u32,
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
            nonce: 0,
            merkle_tree,
            timestamp,
        }
    }

    pub fn first(transactions: Vec<String>, nonce: u32) -> Self {
        let merkle_tree = MerkleTree::new(transactions.clone());
        let timestamp = 0;

        let hash = Hash::from_str(&format!("{}{}{}", merkle_tree.hash(), nonce, timestamp));

        Self {
            hash,
            previous_hash: None,
            transactions,
            nonce,
            merkle_tree,
            timestamp,
        }
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

        assert_eq!(block.previous_hash, None);
        assert_eq!(
            block.hash,
            "114114b048f64608bc91d8f0c55da372716cef4d1e0e89544decf9f74dfa8659"
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
            "80f2614d770a66d5e8391321482285f2e5167af3d11ff9360d47c5de2ce97421"
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
