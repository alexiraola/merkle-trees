use crate::hash::Hash;
use crate::timestamp::Timestamp;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Transaction {
    pub version: u32,
    pub from: String,
    pub to: String,
    pub amount: u64,
    pub timestamp: Option<Timestamp>,
}

impl Transaction {
    pub fn new(
        version: u32,
        from: String,
        to: String,
        amount: u64,
        timestamp: Option<Timestamp>,
    ) -> Self {
        Self {
            version,
            from,
            to,
            amount,
            timestamp,
        }
    }

    pub fn coinbase(to: String, amount: u64, timestamp: Option<Timestamp>) -> Self {
        Self {
            version: 1,
            from: String::new(), // Coinbase transactions have no sender
            to,
            amount,
            timestamp,
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        // Version (4 bytes, little endian)
        bytes.extend_from_slice(&self.version.to_le_bytes());

        // From address (length + bytes)
        let from_bytes = self.from.as_bytes();
        bytes.extend_from_slice(&(from_bytes.len() as u32).to_le_bytes());
        bytes.extend_from_slice(from_bytes);

        // To address (length + bytes)
        let to_bytes = self.to.as_bytes();
        bytes.extend_from_slice(&(to_bytes.len() as u32).to_le_bytes());
        bytes.extend_from_slice(to_bytes);

        // Amount (8 bytes, little endian)
        bytes.extend_from_slice(&self.amount.to_le_bytes());

        // Timestamp (4 bytes, little endian, or zeros if None)
        let timestamp_bytes = self
            .timestamp
            .as_ref()
            .map(|ts| ts.to_bytes())
            .unwrap_or([0u8; 4]);
        bytes.extend_from_slice(&timestamp_bytes);

        bytes
    }

    pub fn tx_id(&self) -> Hash {
        Hash::from_bytes(&self.to_bytes())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creates_transaction() {
        let tx = Transaction::new(
            1,
            "alice".to_string(),
            "bob".to_string(),
            1000000,
            Some(Timestamp::new(1234567890)),
        );

        assert_eq!(tx.version, 1);
        assert_eq!(tx.from, "alice");
        assert_eq!(tx.to, "bob");
        assert_eq!(tx.amount, 1000000);
        assert_eq!(tx.timestamp, Some(Timestamp::new(1234567890)));
    }

    #[test]
    fn test_creates_genesis_transaction() {
        let tx = Transaction::coinbase("miner".to_string(), 5000000000, Some(Timestamp::new(0)));

        assert_eq!(tx.version, 1);
        assert_eq!(tx.from, "");
        assert_eq!(tx.to, "miner");
        assert_eq!(tx.amount, 5000000000);
        assert_eq!(tx.timestamp, Some(Timestamp::new(0)));
    }

    #[test]
    fn test_serializes_to_bytes() {
        let tx = Transaction::new(
            1,
            "alice".to_string(),
            "bob".to_string(),
            1000000,
            Some(Timestamp::new(1234567890)),
        );

        let bytes = tx.to_bytes();

        // Verify structure: version(4) + from_len(4) + from(5) + to_len(4) + to(3) + amount(8) + timestamp(4) = 32 bytes
        assert_eq!(bytes.len(), 32);

        // Check version
        assert_eq!(&bytes[0..4], &1u32.to_le_bytes());

        // Check from length and content
        assert_eq!(&bytes[4..8], &5u32.to_le_bytes()); // "alice" length
        assert_eq!(&bytes[8..13], b"alice");

        // Check to length and content
        assert_eq!(&bytes[13..17], &3u32.to_le_bytes()); // "bob" length
        assert_eq!(&bytes[17..20], b"bob");

        // Check amount
        assert_eq!(&bytes[20..28], &1000000u64.to_le_bytes());

        // Check timestamp
        assert_eq!(&bytes[28..32], &1234567890u32.to_le_bytes());
    }

    #[test]
    fn test_serializes_without_timestamp() {
        let tx = Transaction::new(1, "alice".to_string(), "bob".to_string(), 1000000, None);

        let bytes = tx.to_bytes();

        // Check timestamp is zeros
        assert_eq!(&bytes[28..32], &[0u8, 0u8, 0u8, 0u8]);
    }

    #[test]
    fn test_calculates_tx_id() {
        let tx1 = Transaction::new(
            1,
            "alice".to_string(),
            "bob".to_string(),
            1000000,
            Some(Timestamp::new(1234567890)),
        );

        let tx2 = Transaction::new(
            1,
            "alice".to_string(),
            "bob".to_string(),
            1000000,
            Some(Timestamp::new(1234567890)),
        );

        let tx3 = Transaction::new(
            1,
            "alice".to_string(),
            "charlie".to_string(),
            1000000,
            Some(Timestamp::new(1234567890)),
        );

        // Same transactions should have same tx_id
        assert_eq!(tx1.tx_id(), tx2.tx_id());

        // Different transactions should have different tx_id
        assert_ne!(tx1.tx_id(), tx3.tx_id());
    }

    #[test]
    fn test_genesis_transaction_different() {
        let genesis_tx =
            Transaction::coinbase("miner".to_string(), 5000000000, Some(Timestamp::new(0)));

        let regular_tx = Transaction::new(
            2, // Different version to make it different
            "".to_string(),
            "miner".to_string(),
            5000000000,
            Some(Timestamp::new(0)),
        );

        // Genesis transaction should be different from regular transaction
        assert_ne!(genesis_tx.tx_id(), regular_tx.tx_id());
    }
}
