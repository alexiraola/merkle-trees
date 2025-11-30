use sha2::{Digest, Sha256};
use std::fmt::{Display, Write};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Hash([u8; 32]);

impl Hash {
    pub fn from_str(data: &str) -> Self {
        let hash = Sha256::digest(data);
        let mut bytes = [0u8; 32];
        bytes.copy_from_slice(&hash);
        Self(bytes)
    }

    pub fn to_hex(&self) -> String {
        self.0.iter().fold(String::new(), |mut output, b| {
            let _ = write!(output, "{b:02x}");
            output
        })
    }
}

impl Display for Hash {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_hex())
    }
}

impl PartialEq<&str> for Hash {
    fn eq(&self, other: &&str) -> bool {
        self.to_hex() == *other
    }
}

impl PartialEq<String> for Hash {
    fn eq(&self, other: &String) -> bool {
        self.to_hex() == *other
    }
}

impl Default for Hash {
    fn default() -> Self {
        Hash([0u8; 32])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hashes_bytes() {
        let hash = Hash::from_str("Hello, world!");
        assert_eq!(
            hash.to_hex(),
            "315f5bdb76d078c43b8ac0064e4a0164612b1fce77c869345bfc94c75894edd3"
        );
    }
}
