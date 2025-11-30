use crate::block::Block;
use crate::hash::Hash;

struct Blockchain {
    blocks: Vec<Block>,
}

impl Blockchain {
    fn new() -> Self {
        Blockchain { blocks: vec![] }
    }

    fn add_block(&mut self, transactions: Vec<String>) {
        let block = match self.blocks.last() {
            None => Block::first(transactions, 0),
            Some(last_block) => Block::new(last_block.hash.clone(), transactions),
        };
        self.blocks.push(block);
    }

    fn replace_genesis(&mut self, transactions: Vec<String>) {
        let block = Block::first(transactions, 0);
        match self.blocks.first() {
            None => self.blocks.push(block),
            Some(_) => self.blocks[0] = block,
        }
    }

    fn hash(&self) -> Option<Hash> {
        self.blocks.last().map(|b| b.hash.clone())
    }

    fn verify(&self) -> bool {
        let mut previous_hash: Option<Hash> = None;
        for b in self.blocks.iter() {
            match previous_hash {
                None => (),
                Some(hash) => {
                    if hash != b.previous_hash.clone().unwrap() {
                        return false;
                    }
                }
            }
            previous_hash = Some(b.hash.clone());
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creates_blockchain() {
        let blockchain = Blockchain::new();

        assert_eq!(blockchain.blocks, vec![]);
    }

    #[test]
    fn test_adds_a_block() {
        let mut blockchain = Blockchain::new();
        let genesis = Block::first(
            vec![
                "Tx1".to_string(),
                "Tx2".to_string(),
                "Tx3".to_string(),
                "Tx4".to_string(),
            ],
            0,
        );

        blockchain.add_block(genesis.transactions);

        assert_eq!(blockchain.hash(), Some(genesis.hash));
    }
    #[test]
    fn test_adds_two_blocks() {
        let mut blockchain = Blockchain::new();

        blockchain.add_block(vec![
            "Tx1".to_string(),
            "Tx2".to_string(),
            "Tx3".to_string(),
            "Tx4".to_string(),
        ]);
        blockchain.add_block(vec![
            "Tx5".to_string(),
            "Tx6".to_string(),
            "Tx7".to_string(),
            "Tx8".to_string(),
        ]);

        assert_eq!(
            blockchain.hash().unwrap().to_hex(),
            "80f2614d770a66d5e8391321482285f2e5167af3d11ff9360d47c5de2ce97421"
        );
    }

    #[test]
    fn test_verifies_chain_validity() {
        let mut blockchain = Blockchain::new();

        blockchain.add_block(vec![
            "Tx1".to_string(),
            "Tx2".to_string(),
            "Tx3".to_string(),
            "Tx4".to_string(),
        ]);
        blockchain.add_block(vec![
            "Tx5".to_string(),
            "Tx6".to_string(),
            "Tx7".to_string(),
            "Tx8".to_string(),
        ]);
        blockchain.add_block(vec![
            "Tx9".to_string(),
            "Tx10".to_string(),
            "Tx11".to_string(),
            "Tx12".to_string(),
        ]);

        assert!(blockchain.verify());
    }

    #[test]
    fn test_does_not_verify_invalid_chain() {
        let mut blockchain = Blockchain::new();

        blockchain.add_block(vec![
            "Tx1".to_string(),
            "Tx2".to_string(),
            "Tx3".to_string(),
            "Tx4".to_string(),
        ]);
        blockchain.add_block(vec![
            "Tx5".to_string(),
            "Tx6".to_string(),
            "Tx7".to_string(),
            "Tx8".to_string(),
        ]);
        blockchain.add_block(vec![
            "Tx9".to_string(),
            "Tx10".to_string(),
            "Tx11".to_string(),
            "Tx12".to_string(),
        ]);

        blockchain.replace_genesis(vec![
            "Tx1".to_string(),
            "Tx2".to_string(),
            "Tx3".to_string(),
            "Tx5".to_string(),
        ]);

        assert!(!blockchain.verify());
    }
}
