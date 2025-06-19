//! Custom validation rules for CLI arguments.
//!
//! This module contains validation logic that cannot be expressed through `clap`'s
//! declarative API. Each validation rule checks argument combinations and returns
//! a [`ValidationError`] if the combination is invalid.

use super::Commands;
use super::uuid::SupportedUUIDVersion;

/// Validation errors for argument combinations that are invalid.
///
/// These errors are converted to `clap` errors in the CLI parsing flow,
/// ensuring users see error messages consistent with `clap`'s style.
#[derive(Debug)]
pub(super) enum ValidationError {
    /// Timestamp argument used with incompatible UUID version.
    ///
    /// Only UUID versions 1, 6, and 7 support custom timestamps.
    UuidTimestampVersionMismatch { version: SupportedUUIDVersion },
}

/// Validates parsed CLI arguments for complex rules.
///
/// This function orchestrates all validation rules and returns the first
/// error encountered, or `Ok(())` if all validations pass.
pub(super) fn validate_args(commands: &Commands) -> Result<(), ValidationError> {
    validate_uuid_timestamp_compatibility(commands)?;
    // TODO: future validation rules go here
    Ok(())
}

/// Validates that UUID timestamps are only used with compatible versions.
///
/// Only UUID versions 1, 6, and 7 support custom timestamps. Other versions
/// (v3, v4, v5, v8) do not use timestamps in their generation algorithm.
fn validate_uuid_timestamp_compatibility(commands: &Commands) -> Result<(), ValidationError> {
    if let Commands::Uuid {
        version, timestamp, ..
    } = commands
        && timestamp.is_some()
        && !matches!(
            version,
            SupportedUUIDVersion::V1 | SupportedUUIDVersion::V6 | SupportedUUIDVersion::V7
        )
    {
        return Err(ValidationError::UuidTimestampVersionMismatch { version: *version });
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cli::uuid::SupportedUUIDNamespace;

    #[test]
    fn test_uuid_v1_with_timestamp_valid() {
        let cmd = Commands::Uuid {
            version: SupportedUUIDVersion::V1,
            timestamp: Some((1234567890, 0)),
            namespace: None,
            name: None,
            node_id: None,
            data: None,
        };

        assert!(validate_args(&cmd).is_ok());
    }

    #[test]
    fn test_uuid_v6_with_timestamp_valid() {
        let cmd = Commands::Uuid {
            version: SupportedUUIDVersion::V6,
            timestamp: Some((1234567890, 0)),
            namespace: None,
            name: None,
            node_id: None,
            data: None,
        };

        assert!(validate_args(&cmd).is_ok());
    }

    #[test]
    fn test_uuid_v7_with_timestamp_valid() {
        let cmd = Commands::Uuid {
            version: SupportedUUIDVersion::V7,
            timestamp: Some((1234567890, 0)),
            namespace: None,
            name: None,
            node_id: None,
            data: None,
        };

        assert!(validate_args(&cmd).is_ok());
    }

    #[test]
    fn test_uuid_v3_with_timestamp_invalid() {
        let cmd = Commands::Uuid {
            version: SupportedUUIDVersion::V3,
            timestamp: Some((1234567890, 0)),
            namespace: Some(SupportedUUIDNamespace::DNS),
            name: Some(String::from("test")),
            node_id: None,
            data: None,
        };

        assert!(matches!(
            validate_args(&cmd),
            Err(ValidationError::UuidTimestampVersionMismatch { .. })
        ));
    }

    #[test]
    fn test_uuid_v4_with_timestamp_invalid() {
        let cmd = Commands::Uuid {
            version: SupportedUUIDVersion::V4,
            timestamp: Some((1234567890, 0)),
            namespace: None,
            name: None,
            node_id: None,
            data: None,
        };

        assert!(matches!(
            validate_args(&cmd),
            Err(ValidationError::UuidTimestampVersionMismatch { .. })
        ));
    }

    #[test]
    fn test_uuid_v5_with_timestamp_invalid() {
        let cmd = Commands::Uuid {
            version: SupportedUUIDVersion::V5,
            timestamp: Some((1234567890, 0)),
            namespace: Some(SupportedUUIDNamespace::URL),
            name: Some(String::from("test")),
            node_id: None,
            data: None,
        };

        assert!(matches!(
            validate_args(&cmd),
            Err(ValidationError::UuidTimestampVersionMismatch { .. })
        ));
    }

    #[test]
    fn test_uuid_v8_with_timestamp_invalid() {
        let cmd = Commands::Uuid {
            version: SupportedUUIDVersion::V8,
            timestamp: Some((1234567890, 0)),
            namespace: None,
            name: None,
            node_id: None,
            data: Some([0u8; 16]),
        };

        assert!(matches!(
            validate_args(&cmd),
            Err(ValidationError::UuidTimestampVersionMismatch { .. })
        ));
    }

    #[test]
    fn test_uuid_without_timestamp_valid() {
        let cmd = Commands::Uuid {
            version: SupportedUUIDVersion::V4,
            timestamp: None,
            namespace: None,
            name: None,
            node_id: None,
            data: None,
        };

        assert!(validate_args(&cmd).is_ok());
    }

    #[test]
    fn test_ulid_no_validation_needed() {
        let cmd = Commands::Ulid {
            timestamp: Some(1234567890),
        };

        assert!(validate_args(&cmd).is_ok());
    }

    #[test]
    fn test_objectid_no_validation_needed() {
        let cmd = Commands::ObjectId {
            timestamp: Some(1234567890),
        };

        assert!(validate_args(&cmd).is_ok());
    }
}
