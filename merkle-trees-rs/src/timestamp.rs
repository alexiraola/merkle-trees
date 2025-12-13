use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Eq)]
pub struct Timestamp(u32);

impl Timestamp {
    pub fn new(timestamp: u32) -> Self {
        Self(timestamp)
    }

    pub fn now() -> Self {
        let start_time = SystemTime::now();
        let timestamp = start_time
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs() as u32;

        Self(timestamp)
    }

    pub fn to_bytes(&self) -> [u8; 4] {
        self.0.to_le_bytes()
    }
}

impl PartialEq for Timestamp {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Default for Timestamp {
    fn default() -> Self {
        Self::now()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creates_timestamp() {
        let timestamp = Timestamp::new(0);
        assert_eq!(timestamp.0, 0);
    }

    #[test]
    fn test_returns_bytes() {
        let timestamp = Timestamp::new(0);
        assert_eq!(timestamp.to_bytes(), [0, 0, 0, 0]);
    }
}
