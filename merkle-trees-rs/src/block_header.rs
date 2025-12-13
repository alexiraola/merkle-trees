use crate::{bits::DifficultyTarget, hash::Hash, timestamp::Timestamp};
use std::fmt::Write;

#[derive(Debug, Clone, Eq)]
pub struct BlockHeader {
    pub version: i32,
    pub previous_hash: Hash,
    pub merkle_root: Hash,
    pub timestamp: Timestamp,
    pub difficulty_target: DifficultyTarget,
    pub nonce: u32,
}

impl BlockHeader {
    pub fn new(
        version: i32,
        previous_hash: Hash,
        merkle_root: Hash,
        timestamp: Option<Timestamp>,
        difficulty_target: DifficultyTarget,
        nonce: u32,
    ) -> Self {
        Self {
            version,
            previous_hash,
            merkle_root,
            timestamp: timestamp.unwrap_or_default(),
            difficulty_target,
            nonce,
        }
    }

    pub fn to_bytes(&self) -> [u8; 80] {
        let mut bytes = [0u8; 80];
        bytes[0..4].copy_from_slice(&self.version.to_le_bytes());
        bytes[4..36].copy_from_slice(&self.previous_hash.to_bytes());
        bytes[36..68].copy_from_slice(&self.merkle_root.to_bytes());
        bytes[68..72].copy_from_slice(&self.timestamp.to_bytes());
        bytes[72..76].copy_from_slice(&self.difficulty_target.to_bytes());
        bytes[76..80].copy_from_slice(&self.nonce.to_le_bytes());
        bytes
    }

    pub fn to_bytes_hex(&self) -> String {
        self.to_bytes().iter().fold(String::new(), |mut output, b| {
            let _ = write!(output, "{b:02x}");
            output
        })
    }

    pub fn hash(&self) -> Hash {
        Hash::from_bytes(&Hash::from_bytes(&self.to_bytes()).to_bytes())
    }
}

impl PartialEq for BlockHeader {
    fn eq(&self, other: &Self) -> bool {
        self.hash() == other.hash()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn block_header() -> BlockHeader {
        BlockHeader::new(
            0x3a000000,
            Hash::new([
                0x79, 0xf9, 0xb3, 0x11, 0x35, 0x2c, 0x48, 0x4b, 0xb6, 0x17, 0x20, 0xce, 0x16, 0x4d,
                0x6a, 0x5c, 0xa8, 0x8a, 0x0a, 0xf4, 0x26, 0x4e, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00,
            ]),
            Hash::new([
                0xdf, 0x2d, 0xdb, 0x62, 0xb3, 0x58, 0x31, 0x73, 0xce, 0x87, 0x8a, 0x0a, 0x2e, 0x40,
                0x77, 0x3d, 0x9f, 0x4e, 0xf4, 0x2d, 0x12, 0xd7, 0x36, 0x47, 0xa6, 0x20, 0xf3, 0x0e,
                0xec, 0xa7, 0x46, 0xe7,
            ]),
            Some(Timestamp::new(0x66808a09)),
            DifficultyTarget::new(0x17, 0x035d25),
            0x09c2f027,
        )
    }

    #[test]
    fn test_creates_block_header() {
        let block_header = BlockHeader::new(
            0x00000100,
            Hash::default(),
            Hash::default(),
            Some(Timestamp::new(0)),
            DifficultyTarget::new(0x00, 0x00),
            0,
        );

        assert_eq!(
            block_header,
            BlockHeader {
                version: 256,
                previous_hash: Hash::default(),
                merkle_root: Hash::default(),
                timestamp: Timestamp::new(0),
                difficulty_target: DifficultyTarget::new(0x00, 0x000000),
                nonce: 0,
            }
        );
    }

    #[test]
    fn test_serializes_block_header() {
        let block_header = block_header();

        assert_eq!(
            block_header.to_bytes(),
            [
                0x00, 0x00, 0x00, 0x3a, 0x79, 0xf9, 0xb3, 0x11, 0x35, 0x2c, 0x48, 0x4b, 0xb6, 0x17,
                0x20, 0xce, 0x16, 0x4d, 0x6a, 0x5c, 0xa8, 0x8a, 0x0a, 0xf4, 0x26, 0x4e, 0x01, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xdf, 0x2d, 0xdb, 0x62, 0xb3, 0x58,
                0x31, 0x73, 0xce, 0x87, 0x8a, 0x0a, 0x2e, 0x40, 0x77, 0x3d, 0x9f, 0x4e, 0xf4, 0x2d,
                0x12, 0xd7, 0x36, 0x47, 0xa6, 0x20, 0xf3, 0x0e, 0xec, 0xa7, 0x46, 0xe7, 0x09, 0x8a,
                0x80, 0x66, 0x25, 0x5d, 0x03, 0x17, 0x27, 0xf0, 0xc2, 0x09,
            ]
        );
    }

    #[test]
    fn test_serializes_block_header_to_hex() {
        let block_header = block_header();

        assert_eq!(
            block_header.to_bytes_hex(),
            "0000003a79f9b311352c484bb61720ce164d6a5ca88a0af4264e01000000000000000000df2ddb62b3583173ce878a0a2e40773d9f4ef42d12d73647a620f30eeca746e7098a8066255d031727f0c209"
        );
    }

    #[test]
    fn test_hashes_block_header() {
        let block_header = block_header();

        assert_eq!(
            block_header.hash(),
            "d2fd965841244f029e5b8ffce0536951a117cbaad65f00000000000000000000"
        );
    }

    #[test]
    fn test_headers_with_same_properties_are_equal() {
        let block_header1 = block_header();
        let block_header2 = block_header();

        assert_eq!(block_header1, block_header2);
    }

    #[test]
    fn test_headers_with_different_properties_are_not_equal() {
        let block_header1 = block_header();
        let block_header2 = BlockHeader::new(
            0x3b000000,
            Hash::new([
                0x79, 0xf9, 0xb3, 0x11, 0x35, 0x2c, 0x48, 0x4b, 0xb6, 0x17, 0x20, 0xce, 0x16, 0x4d,
                0x6a, 0x5c, 0xa8, 0x8a, 0x0a, 0xf4, 0x26, 0x4e, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00,
            ]),
            Hash::new([
                0xdf, 0x2d, 0xdb, 0x62, 0xb3, 0x58, 0x31, 0x73, 0xce, 0x87, 0x8a, 0x0a, 0x2e, 0x40,
                0x77, 0x3d, 0x9f, 0x4e, 0xf4, 0x2d, 0x12, 0xd7, 0x36, 0x47, 0xa6, 0x20, 0xf3, 0x0e,
                0xec, 0xa7, 0x46, 0xe7,
            ]),
            Some(Timestamp::new(0x66808a09)),
            DifficultyTarget::new(0x17, 0x035d25),
            0x09c2f027,
        );

        assert_ne!(block_header1, block_header2);
    }
}
