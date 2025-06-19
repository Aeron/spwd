//! UUID-specific CLI types and enumerations.
//!
//! This module defines types used exclusively by the UUID subcommand:
//!
//! - [`SupportedUUIDVersion`]: The UUID versions supported by this tool (v1, v3-v8)
//! - [`SupportedUUIDNamespace`]: Standard UUID namespaces for v3 and v5 (DNS, OID, URL, X500)
//!
//! These types integrate with `clap` through `ValueEnum` to provide CLI argument parsing
//! and validation. They also implement conversions to the underlying `uuid` crate types.

use std::fmt;

#[allow(clippy::upper_case_acronyms)]
#[derive(clap::ValueEnum, Clone, Copy, Debug)]
pub(crate) enum SupportedUUIDVersion {
    #[value(name = "1")]
    V1 = 1,
    #[value(name = "3")]
    V3 = 3,
    #[value(name = "4")]
    V4 = 4,
    #[value(name = "5")]
    V5 = 5,
    #[value(name = "6")]
    V6 = 6,
    #[value(name = "7")]
    V7 = 7,
    #[value(name = "8")]
    V8 = 8,
}

impl fmt::Display for SupportedUUIDVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", *self as u8)
    }
}

#[allow(clippy::upper_case_acronyms)]
#[derive(clap::ValueEnum, Clone)]
pub(crate) enum SupportedUUIDNamespace {
    DNS,
    OID,
    URL,
    X500,
}

impl From<&SupportedUUIDNamespace> for uuid::Uuid {
    fn from(namespace: &SupportedUUIDNamespace) -> Self {
        match namespace {
            SupportedUUIDNamespace::DNS => uuid::Uuid::NAMESPACE_DNS,
            SupportedUUIDNamespace::OID => uuid::Uuid::NAMESPACE_OID,
            SupportedUUIDNamespace::URL => uuid::Uuid::NAMESPACE_URL,
            SupportedUUIDNamespace::X500 => uuid::Uuid::NAMESPACE_X500,
        }
    }
}
