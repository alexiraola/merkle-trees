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
            None => Block::first(transactions),
            Some(last_block) => Block::new(last_block.hash.clone(), transactions),
        };
        self.blocks.push(block);
    }

    fn hash(&self) -> Option<Hash> {
        self.blocks.last().map(|b| b.hash.clone())
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
        let genesis = Block::first(vec![
            "Tx1".to_string(),
            "Tx2".to_string(),
            "Tx3".to_string(),
            "Tx4".to_string(),
        ]);

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
            blockchain.hash().unwrap(),
            "ba607b6f1490f3257354f1831e41a759dafc716d96c230e3858fb6a53393be39"
        );
    }
}
