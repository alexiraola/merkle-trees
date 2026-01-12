use crate::hash::Hash;
use crate::transaction::Transaction;

#[derive(Debug, Clone, Eq)]
struct Node {
    hash: Hash,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
    size: usize,
}

impl Node {
    fn leaf(tx: &Transaction) -> Self {
        Self {
            hash: tx.tx_id(),
            left: None,
            right: None,
            size: 1,
        }
    }

    fn new(left: Node, right: Node) -> Self {
        Self {
            hash: Hash::from_str(&format!("{}{}", left.hash, right.hash)),
            left: Some(Box::new(left.clone())),
            right: Some(Box::new(right.clone())),
            size: left.size + right.size,
        }
    }

    fn merkle_path(&self, index: usize) -> Option<Vec<ProofStep>> {
        if self.left.is_none() && self.right.is_none() {
            return Some(Vec::new());
        }

        if let (Some(left), Some(right)) = (&self.left, &self.right) {
            if index < left.size {
                let mut path = left.merkle_path(index)?;
                path.push(ProofStep {
                    hash: right.hash.clone(),
                    position: Position::Right,
                });
                Some(path)
            } else {
                let mut path = right.merkle_path(index - left.size)?;
                path.push(ProofStep {
                    hash: left.hash.clone(),
                    position: Position::Left,
                });
                Some(path)
            }
        } else {
            None
        }
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.hash == other.hash
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Position {
    Left,
    Right,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ProofStep {
    hash: Hash,
    position: Position,
}

#[derive(Debug, Clone, Eq)]
pub struct MerkleTree {
    root: Node,
}

impl MerkleTree {
    pub fn new(leaves: Vec<Transaction>) -> Self {
        let root = Self::build_tree(leaves);
        Self { root }
    }

    pub fn hash(&self) -> Hash {
        self.root.hash.clone()
    }

    fn build_tree(leaves: Vec<Transaction>) -> Node {
        let mut level: Vec<Node> = leaves.iter().map(|leaf| Node::leaf(leaf)).collect();

        while level.len() > 1 {
            let mut next_level: Vec<Node> = Vec::new();
            for i in (0..level.len()).step_by(2) {
                let left = level[i].clone();
                let right = if i + 1 < level.len() {
                    level[i + 1].clone()
                } else {
                    level[i].clone()
                };

                next_level.push(Node::new(left, right));
            }
            level = next_level;
        }
        level.remove(0)
    }

    fn verify_proof(&self, proof: Vec<ProofStep>, hash: Hash) -> bool {
        let root = proof.iter().fold(hash, |hash, step| match step.position {
            Position::Left => Hash::from_str(&format!("{}{}", step.hash, hash)),
            Position::Right => Hash::from_str(&format!("{}{}", hash, step.hash)),
        });
        root == self.root.hash
    }
}

impl PartialEq for MerkleTree {
    fn eq(&self, other: &Self) -> bool {
        self.hash() == other.hash()
    }
}

// Test module
#[cfg(test)]
mod tests {
    use super::*;
    use crate::timestamp::Timestamp;

    fn create_test_transaction(id: &str) -> Transaction {
        Transaction::new(
            1,
            "alice".to_string(),
            format!("recipient_{}", id),
            1000,
            Some(Timestamp::new(0)),
        )
    }

    #[test]
    fn test_creates_leaf_with_data() {
        let tx = create_test_transaction("Tx1");
        let leaf = Node::leaf(&tx);
        assert_eq!(leaf.hash, tx.tx_id());
    }

    #[test]
    fn test_creates_node_with_left_and_right() {
        let tx1 = create_test_transaction("Tx1");
        let tx2 = create_test_transaction("Tx2");
        let left = Node::leaf(&tx1);
        let right = Node::leaf(&tx2);
        let node = Node::new(left, right);

        let combined_hash = Hash::from_str(&format!("{}{}", tx1.tx_id(), tx2.tx_id()));
        assert_eq!(node.hash, combined_hash);
    }

    #[test]
    fn test_creates_merkle_tree() {
        let leaves = vec![
            create_test_transaction("Tx1"),
            create_test_transaction("Tx2"),
            create_test_transaction("Tx3"),
            create_test_transaction("Tx4"),
        ];
        let tree = MerkleTree::new(leaves);
        assert_eq!(
            tree.root.hash,
            "39af34a258981e8ce8fc8ae00e672204318b024f1d8f53c955ec4537082a6873".to_string()
        );
        assert_eq!(tree.root.size, 4);
    }

    #[test]
    fn test_creates_merkle_tree_with_odd_number_of_leaves() {
        let leaves = vec![
            create_test_transaction("Tx1"),
            create_test_transaction("Tx2"),
            create_test_transaction("Tx3"),
        ];
        let tree = MerkleTree::new(leaves);
        assert_eq!(
            tree.root.hash,
            "4af4e4be22e129326516eb57290f3063721e3c88b35493206dd04dd4847bb30a".to_string()
        );
        assert_eq!(tree.root.size, 4);
    }

    #[test]
    fn test_creates_proof_for_first_index() {
        let tx1 = create_test_transaction("Tx1");
        let tx2 = create_test_transaction("Tx2");
        let tx3 = create_test_transaction("Tx3");
        let tx4 = create_test_transaction("Tx4");

        let leaves = vec![tx1.clone(), tx2.clone(), tx3.clone(), tx4.clone()];
        let tree = MerkleTree::new(leaves);
        let proof = tree.root.merkle_path(0);

        let expected = vec![
            ProofStep {
                hash: tx2.tx_id(),
                position: Position::Right,
            },
            ProofStep {
                hash: Node::new(Node::leaf(&tx3), Node::leaf(&tx4)).hash,
                position: Position::Right,
            },
        ];

        let proof_unwrapped = proof.unwrap();
        assert!(tree.verify_proof(proof_unwrapped.clone(), tx1.tx_id()));
        assert!(!tree.verify_proof(proof_unwrapped, create_test_transaction("Tx5").tx_id()));
    }

    #[test]
    fn test_creates_proof_for_second_index() {
        let tx1 = create_test_transaction("Tx1");
        let tx2 = create_test_transaction("Tx2");
        let tx3 = create_test_transaction("Tx3");
        let tx4 = create_test_transaction("Tx4");

        let leaves = vec![tx1, tx2.clone(), tx3, tx4.clone()];
        let tree = MerkleTree::new(leaves);
        let proof = tree.root.merkle_path(1);

        let tx1_ref = create_test_transaction("Tx1");
        let expected = vec![
            ProofStep {
                hash: tx1_ref.tx_id(),
                position: Position::Left,
            },
            ProofStep {
                hash: Node::new(
                    Node::leaf(&create_test_transaction("Tx3")),
                    Node::leaf(&tx4),
                )
                .hash,
                position: Position::Right,
            },
        ];

        assert!(tree.verify_proof(proof.unwrap(), tx2.tx_id()));
    }

    #[test]
    fn test_verifies_leaf_with_valid_proof() {
        let tx1 = create_test_transaction("Tx1");
        let tx2 = create_test_transaction("Tx2");
        let tx3 = create_test_transaction("Tx3");
        let tx4 = create_test_transaction("Tx4");

        let leaves = vec![tx1.clone(), tx2.clone(), tx3, tx4.clone()];
        // let leaves = vec![
        //     "Tx1".to_string(),
        //     "Tx2".to_string(),
        //     "Tx3".to_string(),
        //     "Tx4".to_string(),
        // ];
        let tree = MerkleTree::new(leaves);
        let proof0 = tree.root.merkle_path(0);
        let proof1 = tree.root.merkle_path(1);

        // assert!(tree.verify_proof(proof0.unwrap(), Hash::from_str("Tx1")));
        assert!(tree.verify_proof(proof0.unwrap(), tx1.tx_id()));
        assert!(!tree.verify_proof(proof1.unwrap(), tx1.tx_id()));
    }
}
