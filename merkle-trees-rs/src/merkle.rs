use sha2::{Digest, Sha256};

fn hash_hex(data: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data.as_bytes());
    let hash = hasher.finalize();
    format!("{:x}", hash)
}

#[derive(Debug, Clone)]
struct Node {
    hash: String,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    fn leaf(data: &str) -> Self {
        Self {
            hash: hash_hex(data),
            left: None,
            right: None,
        }
    }

    fn new(left: Node, right: Node) -> Self {
        Self {
            hash: hash_hex(&format!("{}{}", left.hash, right.hash)),
            left: Some(Box::new(left.clone())),
            right: Some(Box::new(right.clone())),
        }
    }
}

struct MerkleTree {
    root: Node,
}

impl MerkleTree {
    fn new(leaves: Vec<String>) -> Self {
        let root = Self::build_tree(leaves);
        Self { root }
    }

    fn build_tree(leaves: Vec<String>) -> Node {
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
}

// Test module
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creates_leaf_with_data() {
        let leaf = Node::leaf("Tx1");
        assert_eq!(
            leaf.hash,
            "55f743d0d1b9bd86bbd96a46ba4272ddde19f09e3f6e47832e34bb2779a120b5".to_string()
        );
    }

    #[test]
    fn test_creates_node_with_left_and_right() {
        let left = Node::leaf("Tx1");
        let right = Node::leaf("Tx2");
        let node = Node::new(left, right);
        assert_eq!(
            node.hash,
            "0971909734e9c49e0f45caeb15a450d717de387a0a27df245e7e924bb7e62b0e".to_string()
        );
    }

    #[test]
    fn test_creates_merkle_tree() {
        let leaves = vec![
            "Tx1".to_string(),
            "Tx2".to_string(),
            "Tx3".to_string(),
            "Tx4".to_string(),
        ];
        let tree = MerkleTree::new(leaves);
        assert_eq!(
            tree.root.hash,
            "5b260dbcbff182d10cdbd21d8cb9e4446fe71820bb91c8dced8dcfd0e8a9c8ac".to_string()
        );
    }

    #[test]
    fn test_creates_merkle_tree_with_odd_number_of_leaves() {
        let leaves = vec!["Tx1".to_string(), "Tx2".to_string(), "Tx3".to_string()];
        let tree = MerkleTree::new(leaves);
        assert_eq!(
            tree.root.hash,
            "d450c7864e6af68eab970295be53ea3d4e550b775079c366de34d21e15610add".to_string()
        );
    }
}
