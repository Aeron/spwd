//! UUID generator implementation supporting versions 1, 3-8.
//!
//! This module implements UUID generation for all major UUID versions defined in
//! RFC 4122 and the draft RFC for v6-v8. Each version serves different use cases:
//!
//! - **v1**: Time-based with MAC address (legacy, has privacy concerns)
//! - **v3**: Name-based using MD5 hashing (deterministic)
//! - **v4**: Random (most common for general use)
//! - **v5**: Name-based using SHA-1 hashing (deterministic, preferred over v3)
//! - **v6**: Time-ordered, improved over v1 (better database indexing)
//! - **v7**: Time-ordered with Unix timestamp (recommended for new systems)
//! - **v8**: Custom/experimental format
//!
//! # Design
//!
//! [`UuidGenerator`] is an enum with variants for each UUID version, storing the
//! configuration needed for that version. This allows each variant to be created
//! with type-safe constructors ([`new_v1`], [`new_v3`], etc.) while sharing a
//! common [`Generate`] implementation.
//!
//! The [`from_params`] method provides a unified interface for CLI argument conversion,
//! delegating to the appropriate version-specific constructor.
//!
//! [`new_v1`]: UuidGenerator::new_v1
//! [`new_v3`]: UuidGenerator::new_v3
//! [`from_params`]: UuidGenerator::from_params

use crate::cli::uuid::{SupportedUUIDNamespace, SupportedUUIDVersion};
use crate::generators::Generate;
use crate::utils;

/// UUID generator with variants for each supported version.
///
/// Each variant stores the configuration specific to that UUID version.
/// Use the version-specific constructors ([`UuidGenerator::new_v1`], etc.)
/// or [`UuidGenerator::from_params`] for CLI integration.
pub enum UuidGenerator {
    V1 {
        node_id: [u8; 6],
        timestamp: Option<(u64, u32)>,
    },
    V3 {
        namespace: uuid::Uuid,
        name: String,
    },
    V4,
    V5 {
        namespace: uuid::Uuid,
        name: String,
    },
    V6 {
        node_id: [u8; 6],
        timestamp: Option<(u64, u32)>,
    },
    V7 {
        timestamp: Option<(u64, u32)>,
    },
    V8 {
        data: [u8; 16],
    },
}

impl UuidGenerator {
    fn resolve_node_id(node_id: Option<&eui48::MacAddress>) -> [u8; 6] {
        match node_id {
            Some(mac) => mac.to_array(),
            None => utils::generate_pseudo_mac().to_array(),
        }
    }

    pub fn new_v1(node_id: Option<&eui48::MacAddress>, timestamp: Option<(u64, u32)>) -> Self {
        Self::V1 {
            node_id: Self::resolve_node_id(node_id),
            timestamp,
        }
    }

    pub fn new_v3(namespace: &SupportedUUIDNamespace, name: &str) -> Self {
        Self::V3 {
            namespace: namespace.into(),
            name: name.to_string(),
        }
    }

    pub fn new_v4() -> Self {
        Self::V4
    }

    pub fn new_v5(namespace: &SupportedUUIDNamespace, name: &str) -> Self {
        Self::V5 {
            namespace: namespace.into(),
            name: name.to_string(),
        }
    }

    pub fn new_v6(node_id: Option<&eui48::MacAddress>, timestamp: Option<(u64, u32)>) -> Self {
        Self::V6 {
            node_id: Self::resolve_node_id(node_id),
            timestamp,
        }
    }

    pub fn new_v7(timestamp: Option<(u64, u32)>) -> Self {
        Self::V7 { timestamp }
    }

    pub fn new_v8(data: [u8; 16]) -> Self {
        Self::V8 { data }
    }

    pub fn from_params(
        version: SupportedUUIDVersion,
        timestamp: Option<(u64, u32)>,
        namespace: Option<&SupportedUUIDNamespace>,
        name: Option<&String>,
        node_id: Option<&eui48::MacAddress>,
        data: Option<&[u8; 16]>,
    ) -> Self {
        match version {
            SupportedUUIDVersion::V1 => Self::new_v1(node_id, timestamp),
            SupportedUUIDVersion::V3 => Self::new_v3(
                namespace.expect("namespace is required for UUID v3 by clap validation"),
                name.expect("name is required for UUID v3 by clap validation"),
            ),
            SupportedUUIDVersion::V4 => Self::new_v4(),
            SupportedUUIDVersion::V5 => Self::new_v5(
                namespace.expect("namespace is required for UUID v5 by clap validation"),
                name.expect("name is required for UUID v5 by clap validation"),
            ),
            SupportedUUIDVersion::V6 => Self::new_v6(node_id, timestamp),
            SupportedUUIDVersion::V7 => Self::new_v7(timestamp),
            SupportedUUIDVersion::V8 => {
                Self::new_v8(*data.expect("data is required for UUID v8 by clap validation"))
            }
        }
    }
}

impl Generate for UuidGenerator {
    fn generate(&self) -> String {
        match self {
            UuidGenerator::V1 { node_id, timestamp } => match timestamp {
                Some((seconds, subsec_nanos)) => uuid::Uuid::new_v1(
                    uuid::Timestamp::from_unix(uuid::Context::new(0), *seconds, *subsec_nanos),
                    node_id,
                )
                .to_string(),
                None => uuid::Uuid::now_v1(node_id).to_string(),
            },
            UuidGenerator::V3 { namespace, name } => {
                uuid::Uuid::new_v3(namespace, name.as_bytes()).to_string()
            }
            UuidGenerator::V4 => uuid::Uuid::new_v4().to_string(),
            UuidGenerator::V5 { namespace, name } => {
                uuid::Uuid::new_v5(namespace, name.as_bytes()).to_string()
            }
            UuidGenerator::V6 { node_id, timestamp } => match timestamp {
                Some((seconds, subsec_nanos)) => uuid::Uuid::new_v6(
                    uuid::Timestamp::from_unix(
                        uuid::Context::new_random(),
                        *seconds,
                        *subsec_nanos,
                    ),
                    node_id,
                )
                .to_string(),
                None => uuid::Uuid::now_v6(node_id).to_string(),
            },
            UuidGenerator::V7 { timestamp } => {
                match timestamp {
                    Some((seconds, subsec_nanos)) => uuid::Uuid::new_v7(
                        uuid::Timestamp::from_unix(uuid::ContextV7::new(), *seconds, *subsec_nanos),
                    )
                    .to_string(),
                    None => uuid::Uuid::now_v7().to_string(),
                }
            }
            UuidGenerator::V8 { data } => uuid::Uuid::new_v8(*data).to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Helper function to validate UUID format and version
    fn assert_uuid_format(uuid_str: &str, expected_version: u8) {
        assert_eq!(uuid_str.len(), 36, "UUID should be 36 characters long");
        assert_eq!(
            &uuid_str[8..9],
            "-",
            "UUID should have hyphen at position 8"
        );
        assert_eq!(
            &uuid_str[13..14],
            "-",
            "UUID should have hyphen at position 13"
        );
        assert_eq!(
            &uuid_str[18..19],
            "-",
            "UUID should have hyphen at position 18"
        );
        assert_eq!(
            &uuid_str[23..24],
            "-",
            "UUID should have hyphen at position 23"
        );

        // Checking version nibble
        let version_char = uuid_str.chars().nth(14).unwrap();
        assert_eq!(
            version_char.to_digit(16).unwrap() as u8,
            expected_version,
            "UUID version should be {}",
            expected_version
        );

        // Checking variant bits (should be 8, 9, a, or b)
        let variant_char = uuid_str.chars().nth(19).unwrap();
        assert!(
            matches!(variant_char, '8' | '9' | 'a' | 'b'),
            "UUID variant should be RFC4122"
        );
    }

    #[test]
    fn test_new_v1_without_node_id() {
        let generator = UuidGenerator::new_v1(None, None);

        match generator {
            UuidGenerator::V1 { node_id, timestamp } => {
                assert_eq!(node_id.len(), 6);
                assert!(timestamp.is_none());
            }
            _ => panic!("Expected V1 variant"),
        }

        let uuid_str = generator.generate();
        assert_uuid_format(&uuid_str, 1);
    }

    #[test]
    fn test_new_v1_with_node_id() {
        let mac = eui48::MacAddress::new([0x01, 0x23, 0x45, 0x67, 0x89, 0xab]);
        let generator = UuidGenerator::new_v1(Some(&mac), None);

        match generator {
            UuidGenerator::V1 { node_id, timestamp } => {
                assert_eq!(node_id, [0x01, 0x23, 0x45, 0x67, 0x89, 0xab]);
                assert!(timestamp.is_none());
            }
            _ => panic!("Expected V1 variant"),
        }

        let uuid_str = generator.generate();
        assert_uuid_format(&uuid_str, 1);
    }

    #[test]
    fn test_new_v1_with_timestamp() {
        let timestamp = (1234567890, 123456789);
        let generator = UuidGenerator::new_v1(None, Some(timestamp));

        match generator {
            UuidGenerator::V1 { timestamp: ts, .. } => {
                assert_eq!(ts, Some((1234567890, 123456789)));
            }
            _ => panic!("Expected V1 variant"),
        }

        let uuid_str = generator.generate();
        assert_uuid_format(&uuid_str, 1);
    }

    #[test]
    fn test_new_v3() {
        let namespace = SupportedUUIDNamespace::DNS;
        let name = "example.com";
        let generator = UuidGenerator::new_v3(&namespace, name);

        match &generator {
            UuidGenerator::V3 {
                namespace: ns,
                name: n,
            } => {
                assert_eq!(ns, &uuid::Uuid::NAMESPACE_DNS);
                assert_eq!(n, "example.com");
            }
            _ => panic!("Expected V3 variant"),
        }

        let uuid_str = generator.generate();
        assert_uuid_format(&uuid_str, 3);
    }

    #[test]
    fn test_new_v4() {
        let generator = UuidGenerator::new_v4();

        match generator {
            UuidGenerator::V4 => {}
            _ => panic!("Expected V4 variant"),
        }

        let uuid_str = generator.generate();
        assert_uuid_format(&uuid_str, 4);
    }

    #[test]
    fn test_new_v5() {
        let namespace = SupportedUUIDNamespace::URL;
        let name = "https://example.com";
        let generator = UuidGenerator::new_v5(&namespace, name);

        match &generator {
            UuidGenerator::V5 {
                namespace: ns,
                name: n,
            } => {
                assert_eq!(ns, &uuid::Uuid::NAMESPACE_URL);
                assert_eq!(n, "https://example.com");
            }
            _ => panic!("Expected V5 variant"),
        }

        let uuid_str = generator.generate();
        assert_uuid_format(&uuid_str, 5);
    }

    #[test]
    fn test_new_v6_without_node_id() {
        let generator = UuidGenerator::new_v6(None, None);

        match generator {
            UuidGenerator::V6 { node_id, timestamp } => {
                assert_eq!(node_id.len(), 6);
                assert!(timestamp.is_none());
            }
            _ => panic!("Expected V6 variant"),
        }

        let uuid_str = generator.generate();
        assert_uuid_format(&uuid_str, 6);
    }

    #[test]
    fn test_new_v6_with_node_id() {
        let mac = eui48::MacAddress::new([0xfe, 0xdc, 0xba, 0x98, 0x76, 0x54]);
        let generator = UuidGenerator::new_v6(Some(&mac), None);

        match generator {
            UuidGenerator::V6 { node_id, timestamp } => {
                assert_eq!(node_id, [0xfe, 0xdc, 0xba, 0x98, 0x76, 0x54]);
                assert!(timestamp.is_none());
            }
            _ => panic!("Expected V6 variant"),
        }

        let uuid_str = generator.generate();
        assert_uuid_format(&uuid_str, 6);
    }

    #[test]
    fn test_new_v6_with_timestamp() {
        let timestamp = (9876543210, 987654321);
        let generator = UuidGenerator::new_v6(None, Some(timestamp));

        match generator {
            UuidGenerator::V6 { timestamp: ts, .. } => {
                assert_eq!(ts, Some((9876543210, 987654321)));
            }
            _ => panic!("Expected V6 variant"),
        }

        let uuid_str = generator.generate();
        assert_uuid_format(&uuid_str, 6);
    }

    #[test]
    fn test_new_v7_without_timestamp() {
        let generator = UuidGenerator::new_v7(None);

        match generator {
            UuidGenerator::V7 { timestamp } => {
                assert!(timestamp.is_none());
            }
            _ => panic!("Expected V7 variant"),
        }

        let uuid_str = generator.generate();
        assert_uuid_format(&uuid_str, 7);
    }

    #[test]
    fn test_new_v7_with_timestamp() {
        let timestamp = (1700000000, 500000000);
        let generator = UuidGenerator::new_v7(Some(timestamp));

        match generator {
            UuidGenerator::V7 { timestamp: ts } => {
                assert_eq!(ts, Some((1700000000, 500000000)));
            }
            _ => panic!("Expected V7 variant"),
        }

        let uuid_str = generator.generate();
        assert_uuid_format(&uuid_str, 7);
    }

    #[test]
    fn test_new_v8() {
        let data = [
            0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef, 0xfe, 0xdc, 0xba, 0x98, 0x76, 0x54,
            0x32, 0x10,
        ];
        let generator = UuidGenerator::new_v8(data);

        match generator {
            UuidGenerator::V8 { data: d } => {
                assert_eq!(d, data);
            }
            _ => panic!("Expected V8 variant"),
        }

        let uuid_str = generator.generate();
        assert_uuid_format(&uuid_str, 8);
    }

    #[test]
    fn test_from_params_v1() {
        let mac = eui48::MacAddress::new([0x11, 0x22, 0x33, 0x44, 0x55, 0x66]);
        let timestamp = Some((1234567890, 123456789));

        let generator = UuidGenerator::from_params(
            SupportedUUIDVersion::V1,
            timestamp,
            None,
            None,
            Some(&mac),
            None,
        );

        match generator {
            UuidGenerator::V1 {
                node_id,
                timestamp: ts,
            } => {
                assert_eq!(node_id, [0x11, 0x22, 0x33, 0x44, 0x55, 0x66]);
                assert_eq!(ts, timestamp);
            }
            _ => panic!("Expected V1 variant"),
        }
    }

    #[test]
    fn test_from_params_v3() {
        let namespace = SupportedUUIDNamespace::DNS;
        let name = String::from("test.example.com");

        let generator = UuidGenerator::from_params(
            SupportedUUIDVersion::V3,
            None,
            Some(&namespace),
            Some(&name),
            None,
            None,
        );

        match generator {
            UuidGenerator::V3 {
                namespace: ns,
                name: n,
            } => {
                assert_eq!(ns, uuid::Uuid::NAMESPACE_DNS);
                assert_eq!(n, "test.example.com");
            }
            _ => panic!("Expected V3 variant"),
        }
    }

    #[test]
    fn test_from_params_v4() {
        let generator =
            UuidGenerator::from_params(SupportedUUIDVersion::V4, None, None, None, None, None);

        match generator {
            UuidGenerator::V4 => {}
            _ => panic!("Expected V4 variant"),
        }
    }

    #[test]
    fn test_from_params_v5() {
        let namespace = SupportedUUIDNamespace::URL;
        let name = String::from("https://example.org");

        let generator = UuidGenerator::from_params(
            SupportedUUIDVersion::V5,
            None,
            Some(&namespace),
            Some(&name),
            None,
            None,
        );

        match generator {
            UuidGenerator::V5 {
                namespace: ns,
                name: n,
            } => {
                assert_eq!(ns, uuid::Uuid::NAMESPACE_URL);
                assert_eq!(n, "https://example.org");
            }
            _ => panic!("Expected V5 variant"),
        }
    }

    #[test]
    fn test_from_params_v6() {
        let mac = eui48::MacAddress::new([0xaa, 0xbb, 0xcc, 0xdd, 0xee, 0xff]);
        let timestamp = Some((9999999999, 999999999));

        let generator = UuidGenerator::from_params(
            SupportedUUIDVersion::V6,
            timestamp,
            None,
            None,
            Some(&mac),
            None,
        );

        match generator {
            UuidGenerator::V6 {
                node_id,
                timestamp: ts,
            } => {
                assert_eq!(node_id, [0xaa, 0xbb, 0xcc, 0xdd, 0xee, 0xff]);
                assert_eq!(ts, timestamp);
            }
            _ => panic!("Expected V6 variant"),
        }
    }

    #[test]
    fn test_from_params_v7() {
        let timestamp = Some((1234567890, 0));

        let generator =
            UuidGenerator::from_params(SupportedUUIDVersion::V7, timestamp, None, None, None, None);

        match generator {
            UuidGenerator::V7 { timestamp: ts } => {
                assert_eq!(ts, timestamp);
            }
            _ => panic!("Expected V7 variant"),
        }
    }

    #[test]
    fn test_from_params_v8() {
        let data = [
            0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc, 0xde, 0xf0, 0x0f, 0xed, 0xcb, 0xa9, 0x87, 0x65,
            0x43, 0x21,
        ];

        let generator = UuidGenerator::from_params(
            SupportedUUIDVersion::V8,
            None,
            None,
            None,
            None,
            Some(&data),
        );

        match generator {
            UuidGenerator::V8 { data: d } => {
                assert_eq!(d, data);
            }
            _ => panic!("Expected V8 variant"),
        }
    }

    #[test]
    fn test_resolve_node_id_with_mac() {
        let mac = eui48::MacAddress::new([0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc]);
        let node_id = UuidGenerator::resolve_node_id(Some(&mac));

        assert_eq!(node_id, [0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc]);
    }

    #[test]
    fn test_resolve_node_id_without_mac() {
        let node_id = UuidGenerator::resolve_node_id(None);

        // Should generate a pseudo-MAC address (locally administered)
        assert_eq!(node_id.len(), 6);
        assert_eq!(
            node_id[0] & 0x02,
            0x02,
            "Should have locally administered bit set"
        );
        assert_eq!(node_id[0] & 0x01, 0x00, "Should not have multicast bit set");
    }
}
