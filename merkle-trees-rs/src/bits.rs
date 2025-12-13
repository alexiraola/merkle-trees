use crate::hash::Hash;

#[derive(Debug, Clone, Eq)]
pub struct Bits {
    pub exponent: u8,
    pub coefficient: u32,
}

impl Bits {
    pub fn new(exponent: u8, coefficient: u32) -> Self {
        Self {
            exponent,
            coefficient: coefficient & 0x00ffffff,
        }
    }

    pub fn to_bytes(&self) -> [u8; 4] {
        let mut bytes = [0u8; 4];
        bytes[0..3].copy_from_slice(&self.coefficient.to_le_bytes()[0..3]);
        bytes[3] = self.exponent;
        bytes
    }

    pub fn target(&self) -> [u8; 32] {
        let mut target = [0u8; 32];
        let start = (32 - self.exponent) as usize;
        target[start..start + 3].copy_from_slice(&self.coefficient.to_le_bytes()[0..3]);
        target
    }

    pub fn meets_target(&self, hash: &Hash) -> bool {
        let mut hash_be = hash.to_bytes();
        hash_be.reverse();

        hash_be < self.target()
    }
}

impl PartialEq for Bits {
    fn eq(&self, other: &Self) -> bool {
        self.exponent == other.exponent && self.coefficient == other.coefficient
    }
}

#[cfg(test)]
mod tests {
    use crate::hash::Hash;

    use super::*;

    #[test]
    fn test_creates_bits() {
        let bits = Bits::new(0x17, 0x255d03);

        assert_eq!(bits.exponent, 23);
        assert_eq!(bits.coefficient, 0x255d03);
    }

    #[test]
    fn test_clips_coefficient() {
        let bits = Bits::new(0x17, 0x25255d03);

        assert_eq!(bits.exponent, 23);
        assert_eq!(bits.coefficient, 0x255d03);
    }

    #[test]
    fn test_serializes_to_bytes() {
        let bits = Bits::new(0x17, 0x255d03);

        assert_eq!(bits.to_bytes(), [0x03, 0x5d, 0x25, 0x17]);
    }

    #[test]
    fn test_builds_target() {
        let bits = Bits::new(0x1d, 0xffff00);
        let expected_target = [
            0x00, 0x00, 0x00, 0x00, 0xff, 0xff, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00,
        ];

        assert_eq!(bits.target(), expected_target);
    }

    #[test]
    fn test_hash_meets_target() {
        let bits = Bits::new(0x1d, 0xffff00);
        let hash = Hash::new([
            0x6f, 0xe2, 0x8c, 0x0a, 0xb6, 0xf1, 0xb3, 0x72, 0xc1, 0xa6, 0xa2, 0x46, 0xae, 0x63,
            0xf7, 0x4f, 0x93, 0x1e, 0x83, 0x65, 0xe1, 0x5a, 0x08, 0x9c, 0x68, 0xd6, 0x19, 0x00,
            0x00, 0x00, 0x00, 0x00,
        ]);

        assert!(bits.meets_target(&hash));
    }

    #[test]
    fn test_hash_does_not_meet_target() {
        let bits = Bits::new(0x1d, 0xffff00);
        let hash = Hash::new([
            0x6f, 0xe2, 0x8c, 0x0a, 0xb6, 0xf1, 0xb3, 0x72, 0xc1, 0xa6, 0xa2, 0x46, 0xae, 0x63,
            0xf7, 0x4f, 0x93, 0x1e, 0x83, 0x65, 0xe1, 0x5a, 0x08, 0x9c, 0x68, 0xd6, 0x19, 0x00,
            0x00, 0x00, 0x00, 0x01,
        ]);

        assert!(!bits.meets_target(&hash));
    }
}
