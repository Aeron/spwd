//! MongoDB/BSON ObjectId generator implementation.
//!
//! ObjectIds are 12-byte identifiers commonly used in MongoDB as primary keys.
//! They are designed to be:
//! - Globally unique across distributed systems
//! - Roughly sortable by creation time
//! - Compact (96 bits)
//! - Fast to generate
//!
//! # Format
//!
//! An ObjectId consists of:
//! - 4 bytes: Unix timestamp (seconds since epoch)
//! - 5 bytes: Random value (process identifier + machine identifier)
//! - 3 bytes: Incrementing counter
//!
//! # Usage
//!
//! The generator can operate in two modes:
//! - **Current time**: Uses the system clock (default)
//! - **Fixed timestamp**: Uses a provided seconds-since-epoch timestamp
//!
//! When using a fixed timestamp, the timestamp portion is deterministic but
//! the random and counter portions still change, ensuring uniqueness.

use crate::generators::Generate;

/// ObjectId generator that can use either current time or a fixed timestamp.
///
/// The generator stores an optional timestamp in seconds since Unix epoch.
/// If `None`, it generates ObjectIds using the current system time.
pub struct ObjectIdGenerator {
    timestamp: Option<u32>,
}

impl ObjectIdGenerator {
    pub fn new(timestamp: Option<u32>) -> Self {
        Self { timestamp }
    }
}

impl Generate for ObjectIdGenerator {
    fn generate(&self) -> String {
        match self.timestamp {
            Some(seconds) => {
                // HACK: The BSON crate does not provide a constructor for ObjectId with a custom
                // timestamp. So, the workaround is to use original process identifier and counter
                // bytes, then rebuild it with our timestamp using from_parts(). This maintains
                // the original ObjectId generation behavior for everything except the timestamp
                // portion.
                let oid = bson::oid::ObjectId::new().bytes();
                bson::oid::ObjectId::from_parts(
                    seconds,
                    [oid[4], oid[5], oid[6], oid[7], oid[8]],
                    [oid[9], oid[10], oid[11]],
                )
                .to_hex()
            }
            None => bson::oid::ObjectId::new().to_hex(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Helper function to validate ObjectId format
    fn assert_objectid_format(oid_str: &str) {
        assert_eq!(oid_str.len(), 24, "ObjectId should be 24 characters long");
        assert!(
            oid_str.chars().all(|c| c.is_ascii_hexdigit()),
            "ObjectId should only contain hex characters"
        );
    }

    #[test]
    fn test_new_without_timestamp() {
        let generator = ObjectIdGenerator::new(None);

        assert!(generator.timestamp.is_none());

        let oid_str = generator.generate();
        assert_objectid_format(&oid_str);
    }

    #[test]
    fn test_new_with_timestamp() {
        let timestamp = 1234567890;
        let generator = ObjectIdGenerator::new(Some(timestamp));

        assert_eq!(generator.timestamp, Some(1234567890));

        let oid_str = generator.generate();
        assert_objectid_format(&oid_str);
    }

    #[test]
    fn test_generate_without_timestamp() {
        let generator = ObjectIdGenerator::new(None);

        let oid = generator.generate();
        assert_objectid_format(&oid);
    }

    #[test]
    fn test_generate_with_zero_timestamp() {
        let generator = ObjectIdGenerator::new(Some(0));

        let oid_str = generator.generate();
        assert_objectid_format(&oid_str);

        // ObjectId with timestamp 0 should start with 8 zeros
        assert!(oid_str.starts_with("00000000"));
    }

    #[test]
    fn test_generate_with_max_u32_timestamp() {
        // Maximum u32 timestamp (year 2106)
        let generator = ObjectIdGenerator::new(Some(u32::MAX));

        let oid_str = generator.generate();
        assert_objectid_format(&oid_str);

        // Maximum u32 as hex should be "ffffffff"
        assert!(oid_str.starts_with("ffffffff"));
    }
}
