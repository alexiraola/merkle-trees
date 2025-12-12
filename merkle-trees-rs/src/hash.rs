use sha2::{Digest, Sha256};
use std::fmt::{Display, Write};

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Hash([u8; 32]);

impl Hash {
    pub fn new(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }

    pub fn from_str(data: &str) -> Self {
        let hash = Sha256::digest(data);
        Self(hash.into())
    }

    pub fn from_bytes(bytes: &[u8]) -> Self {
        let hash = Sha256::digest(bytes);
        Self(hash.into())
    }

    pub fn to_hex(&self) -> String {
        self.0.iter().fold(String::new(), |mut output, b| {
            let _ = write!(output, "{b:02x}");
            output
        })
    }

    pub fn to_bytes(&self) -> [u8; 32] {
        self.0
    }
}

impl From<[u8; 32]> for Hash {
    fn from(bytes: [u8; 32]) -> Self {
        Self(bytes)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hashes_str() {
        let hash = Hash::from_str("Hello, world!");
        assert_eq!(
            hash.to_hex(),
            "315f5bdb76d078c43b8ac0064e4a0164612b1fce77c869345bfc94c75894edd3"
        );
    }

    #[test]
    fn test_equals_string() {
        let hash = Hash::from_str("Hello, world!");
        assert_eq!(
            hash,
            "315f5bdb76d078c43b8ac0064e4a0164612b1fce77c869345bfc94c75894edd3"
        );
    }

    #[test]
    fn test_does_not_equal_wrong_hash() {
        let hash = Hash::from_str("Hello, world!");
        assert_ne!(
            hash,
            "315f5bdb76d078c43b8ac0064e4a0164612b1fce77c869345bfc94c75894edd4"
        );
    }

    #[test]
    fn test_displays_hash() {
        let hash = Hash::from_str("Hello, world!");
        assert_eq!(
            format!("{hash}"),
            "315f5bdb76d078c43b8ac0064e4a0164612b1fce77c869345bfc94c75894edd3"
        );
    }

    #[test]
    fn test_hashes_bytes() {
        const FRAME: [u8; 80] = [
            0x00, 0x00, 0x00, 0x3a, 0x79, 0xf9, 0xb3, 0x11, 0x35, 0x2c, 0x48, 0x4b, 0xb6, 0x17,
            0x20, 0xce, 0x16, 0x4d, 0x6a, 0x5c, 0xa8, 0x8a, 0x0a, 0xf4, 0x26, 0x4e, 0x01, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xdf, 0x2d, 0xdb, 0x62, 0xb3, 0x58,
            0x31, 0x73, 0xce, 0x87, 0x8a, 0x0a, 0x2e, 0x40, 0x77, 0x3d, 0x9f, 0x4e, 0xf4, 0x2d,
            0x12, 0xd7, 0x36, 0x47, 0xa6, 0x20, 0xf3, 0x0e, 0xec, 0xa7, 0x46, 0xe7, 0x09, 0x8a,
            0x80, 0x66, 0x25, 0x5d, 0x03, 0x17, 0x27, 0xf0, 0xc2, 0x09,
        ];
        let hash = Hash::from_bytes(&Hash::from_bytes(&FRAME).to_bytes());

        assert_eq!(
            hash.to_hex(),
            "d2fd965841244f029e5b8ffce0536951a117cbaad65f00000000000000000000"
        );
    }
}
