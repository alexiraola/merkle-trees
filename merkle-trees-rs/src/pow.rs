use crate::block::Block;
use crate::hash;

pub fn build_block(
    previous_hash: Option<hash::Hash>,
    transactions: Vec<String>,
    difficulty: usize,
) -> Block {
    let prefix = build_prefix(difficulty);
    let mut nonce = 0;
    let mut block = Block::new(previous_hash.clone(), transactions.clone(), nonce);

    while !block.hash.to_hex().starts_with(&prefix) {
        nonce = rand::random_range(0..u32::MAX);
        block = Block::new(previous_hash.clone(), transactions.clone(), nonce);
    }
    block.clone()
}

fn proof_of_work(block: String, difficulty: usize) -> u32 {
    let prefix = build_prefix(difficulty);
    let mut nonce = 0;

    let mut block_hash = hash::Hash::from_str(&format!("{}{}", block, nonce));

    while !block_hash.to_hex().starts_with(&prefix) {
        nonce += 1;
        block_hash = hash::Hash::from_str(&format!("{}{}", block, nonce));
    }
    nonce
}

fn build_prefix(difficulty: usize) -> String {
    "0".repeat(difficulty)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builds_prefix() {
        let prefix = build_prefix(10);
        assert_eq!(prefix, "0000000000");
    }

    #[test]
    fn test_proof_of_work() {
        let block = "Tx1".to_string();
        let difficulty = 4;
        let nonce = proof_of_work(block.clone(), difficulty);
        let hash = hash::Hash::from_str(&format!("{}{}", block, nonce));
        assert!(hash.to_hex().starts_with("0000"));
    }

    #[test]
    fn test_build_block_with_difficulty() {
        let transactions = vec!["Tx1".to_string(), "Tx2".to_string(), "Tx3".to_string()];
        let difficulty = 2;
        let block = build_block(None, transactions.clone(), difficulty);
        assert!(block.hash.to_hex().starts_with("00"));
    }
}
