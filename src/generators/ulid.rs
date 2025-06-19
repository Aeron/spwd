//! ULID (Universally Unique Lexicographically Sortable Identifier) generator.
//!
//! ULIDs are 128-bit identifiers that are:
//! - Lexicographically sortable (by creation time)
//! - Encoded as 26-character Crockford Base32 strings
//! - Compatible with UUID storage (same size)
//! - More efficient for database indexing than random UUIDs
//!
//! # Format
//!
//! A ULID consists of:
//! - 48-bit timestamp (millisecond precision)
//! - 80-bit randomness
//!
//! # Usage
//!
//! The generator can operate in two modes:
//! - **Current time**: Uses the system clock (default)
//! - **Fixed timestamp**: Uses a provided millisecond timestamp for deterministic generation
//!
//! When using a fixed timestamp, the timestamp portion remains constant but the
//! random portion changes with each generation, ensuring uniqueness.

use std::time::{Duration, SystemTime};

use crate::generators::Generate;

/// ULID generator that can use either current time or a fixed timestamp.
///
/// The generator stores an optional timestamp in milliseconds since Unix epoch.
/// If `None`, it generates ULIDs using the current system time.
pub struct UlidGenerator {
    timestamp: Option<u64>,
}

impl UlidGenerator {
    pub fn new(timestamp: Option<u64>) -> Self {
        Self { timestamp }
    }
}

impl Generate for UlidGenerator {
    fn generate(&self) -> String {
        match self.timestamp {
            Some(millis) => {
                ulid::Ulid::from_datetime(SystemTime::UNIX_EPOCH + Duration::from_millis(millis))
                    .to_string()
            }
            None => ulid::Ulid::new().to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Helper function to validate ULID format
    fn assert_ulid_format(ulid_str: &str) {
        assert_eq!(ulid_str.len(), 26, "ULID should be 26 characters long");
        assert!(
            ulid_str
                .chars()
                .all(|c| c.is_ascii_uppercase() || c.is_ascii_digit()),
            "ULID should only contain uppercase letters and digits"
        );
    }

    #[test]
    fn test_new_without_timestamp() {
        let generator = UlidGenerator::new(None);

        assert!(generator.timestamp.is_none());

        let ulid_str = generator.generate();
        assert_ulid_format(&ulid_str);
    }

    #[test]
    fn test_new_with_timestamp() {
        let timestamp = 1234567890123;
        let generator = UlidGenerator::new(Some(timestamp));

        assert_eq!(generator.timestamp, Some(1234567890123));

        let ulid_str = generator.generate();
        assert_ulid_format(&ulid_str);
    }

    #[test]
    fn test_generate_without_timestamp() {
        let generator = UlidGenerator::new(None);

        let ulid = generator.generate();
        assert_ulid_format(&ulid);
    }

    #[test]
    fn test_generate_with_zero_timestamp() {
        let generator = UlidGenerator::new(Some(0));

        let ulid_str = generator.generate();
        assert_ulid_format(&ulid_str);

        // ULID with timestamp 0 should start with all zeros
        assert!(ulid_str.starts_with("00000000"));
    }

    #[test]
    fn test_generate_with_max_timestamp() {
        // Maximum timestamp that won't overflow (281474976710655 ms = about year 10889)
        let generator = UlidGenerator::new(Some(281474976710655));

        let ulid_str = generator.generate();
        assert_ulid_format(&ulid_str);
    }
}
