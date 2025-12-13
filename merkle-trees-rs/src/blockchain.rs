use std::time::{SystemTime, UNIX_EPOCH};

use crate::block::Block;
use crate::hash::Hash;
use crate::pow::build_block;
use crate::timestamp::Timestamp;

pub struct Blockchain {
    blocks: Vec<Block>,
}

impl Blockchain {
    pub fn new() -> Self {
        Blockchain { blocks: vec![] }
    }

    fn add_block(&mut self, transactions: Vec<String>, timestamp: Option<Timestamp>) {
        let block = match self.blocks.last() {
            None => Block::genesis(transactions, Some(Timestamp::new(0)), 0),
            Some(last_block) => Block::new(Some(last_block.hash()), transactions, timestamp, 0),
        };
        self.blocks.push(block);
    }

    pub fn build_with_hash_rate(&mut self, hash_rate: f64) {
        let mut difficulty = 4;
        let start_time = SystemTime::now();

        println!(
            "Starting block generation: {}",
            start_time.duration_since(UNIX_EPOCH).unwrap().as_secs()
        );

        while self.blocks.len() < 2016 {
            let transactions = (self.blocks.len()..self.blocks.len() + 4)
                .map(|i| format!("Tx{}", i).to_string())
                .collect();

            println!(
                "Block num {}, difficulty {}",
                self.blocks.len() + 1,
                difficulty
            );

            let next_block = match self.blocks.last() {
                None => build_block(None, transactions, difficulty),
                Some(last_block) => build_block(Some(last_block.hash()), transactions, difficulty),
            };

            println!(
                "Built block with hash {}, nonce {}",
                next_block.hash().to_hex(),
                next_block.header.nonce
            );

            self.blocks.push(next_block);

            let total_time = SystemTime::now()
                .duration_since(start_time)
                .map(|duration| duration.as_secs_f64() / self.blocks.len() as f64);

            match total_time {
                Ok(rate) => {
                    if rate > hash_rate {
                        difficulty -= 1;
                        println!("Hash rate is {}, decreasing difficulty", rate);
                    } else {
                        difficulty += 1;
                        println!("Hash rate is {}, increasing difficulty", rate);
                    }
                }
                Err(e) => print!("{}", e),
            };
        }
    }

    fn replace_genesis(&mut self, transactions: Vec<String>) {
        let block = Block::genesis(transactions, Some(Timestamp::new(0)), 0);
        match self.blocks.first() {
            None => self.blocks.push(block),
            Some(_) => self.blocks[0] = block,
        }
    }

    fn hash(&self) -> Option<Hash> {
        self.blocks.last().map(|b| b.hash())
    }

    fn verify(&self) -> bool {
        let mut previous_hash: Option<Hash> = None;
        for b in self.blocks.iter() {
            match previous_hash {
                None => (),
                Some(hash) => {
                    if hash != b.header.previous_hash.clone() {
                        return false;
                    }
                }
            }
            previous_hash = Some(b.hash());
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use rand::rand_core::block;

    use super::*;

    #[test]
    fn test_creates_blockchain() {
        let blockchain = Blockchain::new();

        assert_eq!(blockchain.blocks, vec![]);
    }

    #[test]
    fn test_adds_a_block() {
        let mut blockchain = Blockchain::new();
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

        blockchain.add_block(genesis.transactions.clone(), Some(Timestamp::new(0)));

        assert_eq!(blockchain.hash(), Some(genesis.hash()));
    }
    #[test]
    fn test_adds_two_blocks() {
        let mut blockchain = Blockchain::new();

        blockchain.add_block(
            vec![
                "Tx1".to_string(),
                "Tx2".to_string(),
                "Tx3".to_string(),
                "Tx4".to_string(),
            ],
            Some(Timestamp::new(0)),
        );
        blockchain.add_block(
            vec![
                "Tx5".to_string(),
                "Tx6".to_string(),
                "Tx7".to_string(),
                "Tx8".to_string(),
            ],
            Some(Timestamp::new(0)),
        );

        assert_eq!(
            blockchain.hash().unwrap().to_hex(),
            "0c9713b3c13b1301c5f108c27926aaa85fa4b2ddefca76e206916384de9c2811"
        );
    }

    #[test]
    fn test_verifies_chain_validity() {
        let mut blockchain = Blockchain::new();

        blockchain.add_block(
            vec![
                "Tx1".to_string(),
                "Tx2".to_string(),
                "Tx3".to_string(),
                "Tx4".to_string(),
            ],
            Some(Timestamp::new(0)),
        );
        blockchain.add_block(
            vec![
                "Tx5".to_string(),
                "Tx6".to_string(),
                "Tx7".to_string(),
                "Tx8".to_string(),
            ],
            Some(Timestamp::new(0)),
        );
        blockchain.add_block(
            vec![
                "Tx9".to_string(),
                "Tx10".to_string(),
                "Tx11".to_string(),
                "Tx12".to_string(),
            ],
            Some(Timestamp::new(0)),
        );

        assert!(blockchain.verify());
    }

    #[test]
    fn test_does_not_verify_invalid_chain() {
        let mut blockchain = Blockchain::new();

        blockchain.add_block(
            vec![
                "Tx1".to_string(),
                "Tx2".to_string(),
                "Tx3".to_string(),
                "Tx4".to_string(),
            ],
            Some(Timestamp::new(0)),
        );
        blockchain.add_block(
            vec![
                "Tx5".to_string(),
                "Tx6".to_string(),
                "Tx7".to_string(),
                "Tx8".to_string(),
            ],
            Some(Timestamp::new(0)),
        );
        blockchain.add_block(
            vec![
                "Tx9".to_string(),
                "Tx10".to_string(),
                "Tx11".to_string(),
                "Tx12".to_string(),
            ],
            Some(Timestamp::new(0)),
        );

        blockchain.replace_genesis(vec![
            "Tx1".to_string(),
            "Tx2".to_string(),
            "Tx3".to_string(),
            "Tx5".to_string(),
        ]);

        assert!(!blockchain.verify());
    }
}
