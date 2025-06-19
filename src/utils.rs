//! Utility functions for parsing and generating data.
//!
//! This module provides helper functions used across the application:
//!
//! - [`parse_timestamp_ns`]: Parses nanosecond timestamps from CLI strings
//! - [`parse_data`]: Parses hex-encoded data for UUID v8
//! - [`generate_pseudo_mac`]: Generates locally-administered MAC addresses for UUID v1/v6
//!
//! These utilities handle input validation, format conversion, and random data generation
//! needed by the various identifier generators.

use anyhow::anyhow;
use rand::Rng;

const TIMESTAMP_LENGTH_NANOS: usize = 9;
const TIMESTAMP_LENGTH_CHARS: usize = 20 + TIMESTAMP_LENGTH_NANOS;

const DATA_LENGTH_BYTES: usize = 16;
const DATA_LENGTH_CHARS: usize = DATA_LENGTH_BYTES * 2;

const MAX_SECONDS: u64 = u64::MAX;
const MAX_NANOSECONDS: u32 = 999999999;

/// Parses a timestamp string into seconds and nanoseconds.
pub(crate) fn parse_timestamp_ns(value: &str) -> anyhow::Result<(u64, u32)> {
    let length = value.len();
    match length {
        1..=TIMESTAMP_LENGTH_CHARS if value.bytes().all(|c| u8::is_ascii_digit(&c)) => {
            let (sec_result, nano_result) = if value.len() > TIMESTAMP_LENGTH_NANOS {
                let (sec_str, nano_str) = value.split_at(value.len() - 9);
                (sec_str.parse::<u64>(), nano_str.parse::<u32>())
            } else {
                (Ok(0), value.parse::<u32>())
            };

            match (sec_result, nano_result) {
                (Ok(s), Ok(n)) => Ok((s, n)),
                _ => Err(anyhow!(
                    "timestamp must be a valid non-negative integer between 0 and {MAX_SECONDS}{MAX_NANOSECONDS}"
                )),
            }
        }
        1..=TIMESTAMP_LENGTH_CHARS => Err(anyhow!("timestamp must contain only digits")),
        _ => Err(anyhow!(
            "timestamp length must be between 1 and {TIMESTAMP_LENGTH_CHARS} digits, got {length}"
        )),
    }
}

/// Parses user data (hex-encoded) string into bytes.
pub(crate) fn parse_data(value: &str) -> anyhow::Result<[u8; DATA_LENGTH_BYTES]> {
    let length = value.len();
    match length {
        1..=DATA_LENGTH_CHARS if value.bytes().all(|c| u8::is_ascii_hexdigit(&c)) => {
            // Padding short hex strings with trailing zeros
            // NOTE: one string allocation per call, but it is acceptable
            let mut full = String::with_capacity(DATA_LENGTH_CHARS);
            full.push_str(value);
            full.extend(std::iter::repeat_n('0', DATA_LENGTH_CHARS - length));

            // Decoding the 16 bytes of data
            let mut data = [0u8; DATA_LENGTH_BYTES];
            hex::decode_to_slice(full, data.as_mut_slice())
                .map_err(|e| anyhow!("hex decode error: {e}"))?;

            Ok(data)
        }
        1..=DATA_LENGTH_CHARS => Err(anyhow!("data must contain only hex characters")),
        _ => Err(anyhow!(
            "data length must be between 1 and {DATA_LENGTH_CHARS} characters, got {length}"
        )),
    }
}

/// Generates a pseudo-random MAC address.
pub(crate) fn generate_pseudo_mac() -> eui48::MacAddress {
    let mut rng = rand::rng();
    let mut mac = [0u8; eui48::EUI48LEN];

    rng.fill(&mut mac);

    // NOTE: Setting the locally administered bit (bit 1) marks this as a generated
    // MAC address (not from real hardware). Clearing the multicast bit (bit 0)
    // ensures it is treated as a unicast address. This follows IEEE 802 standards
    // and prevents conflicts with real network hardware MAC addresses.
    mac[0] = (mac[0] | 0x2) & 0xFE;

    eui48::MacAddress::new(mac)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_timestamp_min() {
        let result = parse_timestamp_ns("0");

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), (0, 0));
    }

    #[test]
    fn test_parse_timestamp_max() {
        let result = parse_timestamp_ns("18446744073709551615999999999");

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), (u64::MAX, 999999999));
    }

    #[test]
    fn test_parse_timestamp_nanos() {
        let result = parse_timestamp_ns("999");

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), (0, 999));
    }

    #[test]
    fn test_parse_timestamp_negative() {
        let result = parse_timestamp_ns("-1");

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().root_cause().to_string(),
            "timestamp must contain only digits"
        );
    }

    #[test]
    fn test_parse_timestamp_overflow() {
        let result = parse_timestamp_ns("18446744073709551616999999999");

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().root_cause().to_string(),
            "timestamp must be a valid non-negative integer between 0 and 18446744073709551615999999999"
        );
    }

    #[test]
    fn test_parse_timestamp_empty() {
        let result = parse_timestamp_ns("");

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().root_cause().to_string(),
            "timestamp length must be between 1 and 29 digits, got 0"
        );
    }

    #[test]
    fn test_parse_timestamp_invalid() {
        let result = parse_timestamp_ns("abc999");

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().root_cause().to_string(),
            "timestamp must contain only digits"
        );
    }

    #[test]
    fn test_parse_data_short() {
        let result = parse_data("0011223344556677");

        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            [0, 17, 34, 51, 68, 85, 102, 119, 0, 0, 0, 0, 0, 0, 0, 0]
        );
    }

    #[test]
    fn test_parse_data_full() {
        let result = parse_data("00112233445566778899aabbccddeeff");

        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            [
                0, 17, 34, 51, 68, 85, 102, 119, 136, 153, 170, 187, 204, 221, 238, 255
            ]
        );
    }

    #[test]
    fn test_parse_data_invalid() {
        let result = parse_data("gg");

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().root_cause().to_string(),
            "data must contain only hex characters"
        );
    }

    #[test]
    fn test_parse_data_empty() {
        let result = parse_data("");

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().root_cause().to_string(),
            "data length must be between 1 and 32 characters, got 0"
        );
    }

    #[test]
    fn test_parse_data_overflow() {
        let result = parse_data("00112233445566778899aabbccddeefff");

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().root_cause().to_string(),
            "data length must be between 1 and 32 characters, got 33"
        );
    }

    #[test]
    fn test_generate_pseudo_mac() {
        let result = generate_pseudo_mac();

        assert!(result.is_local());
        assert!(!result.is_multicast());
        assert!(!result.is_broadcast());
        assert!(!result.is_nil());
    }
}
