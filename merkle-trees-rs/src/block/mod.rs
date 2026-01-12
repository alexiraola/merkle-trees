mod bits;
mod header;

pub use bits::DifficultyTarget;

use crate::hash::Hash;
use crate::merkle::MerkleTree;
use crate::timestamp::Timestamp;
use crate::transaction::Transaction;
use header::BlockHeader;

#[derive(Debug, Clone, Eq)]
pub struct Block {
    pub header: BlockHeader,
    pub transactions: Vec<Transaction>,
}

impl Block {
    pub fn new(
        previous_hash: Option<Hash>,
        transactions: Vec<Transaction>,
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

    pub fn genesis(
        transactions: Vec<Transaction>,
        timestamp: Option<Timestamp>,
        nonce: u32,
    ) -> Self {
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

    fn create_test_transactions() -> Vec<Transaction> {
        vec![
            Transaction::new(
                1,
                "alice".to_string(),
                "bob".to_string(),
                1000,
                Some(Timestamp::new(0)),
            ),
            Transaction::new(
                1,
                "bob".to_string(),
                "charlie".to_string(),
                500,
                Some(Timestamp::new(0)),
            ),
            Transaction::new(
                1,
                "charlie".to_string(),
                "dave".to_string(),
                750,
                Some(Timestamp::new(0)),
            ),
            Transaction::coinbase("miner".to_string(), 5000000, Some(Timestamp::new(0))),
        ]
    }

    #[test]
    fn test_creates_genesis_block() {
        let transactions = create_test_transactions();
        let block = Block::genesis(transactions, Some(Timestamp::new(0)), 0);

        assert_eq!(block.header.previous_hash, Hash::default());
        assert_eq!(
            block.hash(),
            "2ec7676f56d345c71f9bcf8b0f44de42a81a7edce4ab8f96a4c3da6533a902ab"
        );
    }

    #[test]
    fn test_creates_block_with_previous() {
        let genesis = Block::genesis(create_test_transactions(), Some(Timestamp::new(0)), 0);

        let next_block = Block::new(
            Some(genesis.hash().clone()),
            vec![
                Transaction::new(
                    1,
                    "eve".to_string(),
                    "frank".to_string(),
                    2000,
                    Some(Timestamp::new(0)),
                ),
                Transaction::new(
                    1,
                    "frank".to_string(),
                    "grace".to_string(),
                    1500,
                    Some(Timestamp::new(0)),
                ),
                Transaction::new(
                    1,
                    "grace".to_string(),
                    "henry".to_string(),
                    800,
                    Some(Timestamp::new(0)),
                ),
                Transaction::coinbase("miner2".to_string(), 5000000, Some(Timestamp::new(0))),
            ],
            Some(Timestamp::new(0)),
            0,
        );

        assert_eq!(next_block.header.previous_hash, genesis.hash());
        assert_eq!(next_block.header.timestamp, Timestamp::new(0));
        assert_eq!(
            next_block.hash(),
            "22365fbfe1880bef49e3261ec9e7ae3bf411abbd836685990a6b97bb96e7a68d"
        );
    }

    #[test]
    fn test_two_blocks_with_the_same_transactions_have_equal_hash() {
        let transactions = create_test_transactions();
        let block = Block::genesis(transactions.clone(), Some(Timestamp::new(0)), 0);
        let other_block = Block::genesis(transactions, Some(Timestamp::new(0)), 0);

        assert_eq!(block, other_block);
    }

    #[test]
    fn test_two_blocks_with_the_different_transactions_have_not_equal_hash() {
        let transactions1 = create_test_transactions();
        let mut transactions2 = create_test_transactions();
        transactions2[3] = Transaction::coinbase(
            "different_miner".to_string(),
            5000000,
            Some(Timestamp::new(0)),
        );

        let block = Block::genesis(transactions1, Some(Timestamp::new(0)), 0);
        let other_block = Block::genesis(transactions2, Some(Timestamp::new(0)), 0);

        assert_ne!(block, other_block);
    }
}
