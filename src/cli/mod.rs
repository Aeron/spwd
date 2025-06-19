//! Command-line interface definitions and parsing.
//!
//! This module defines the CLI structure using `clap`, including all commands,
//! arguments, and custom validation logic. The main entry point is [`Args::parse()`],
//! which handles argument parsing and performs additional validation that cannot
//! be expressed through `clap`'s declarative API.
//!
//! # Structure
//!
//! - [`Args`]: Top-level argument structure with global options (like `--num`)
//! - [`Commands`]: Subcommands for each identifier type (UUID, ULID, ObjectId)
//! - `uuid` submodule: UUID-specific types (versions, namespaces)
//!
//! # Custom Validation
//!
//! Some validation rules are too complex for `clap`'s built-in validators:
//! - Timestamp argument compatibility with UUID versions (only v1, v6, v7 support it)
//!
//! These are checked in [`Args::parse()`] after `clap` performs basic validation.

pub mod uuid;
mod validation;

use clap::error::{ContextKind, ContextValue, ErrorKind};
use clap::{
    CommandFactory, Parser, Subcommand, crate_description, crate_name, crate_version, value_parser,
};

use crate::utils;

#[derive(Parser)]
#[command(
    name = crate_name!(),
    version = crate_version!(),
    about = crate_description!(),
    disable_help_subcommand=true,
)]
pub(crate) struct Args {
    #[command(subcommand)]
    pub(crate) command: Commands,

    /// Number of results
    #[arg(short = 'n', long = "num", default_value = "1")]
    pub(crate) number: usize,
}

#[derive(Subcommand)]
pub(crate) enum Commands {
    /// Generate a new UUID
    #[command(long_about = "Generates a new Universally Unique Identifier.")]
    Uuid {
        /// UUID version
        #[arg(short, long, value_enum, default_value = "4")]
        version: uuid::SupportedUUIDVersion,

        /// UUID timestamp (in nanoseconds; versions 1, 6, and 7 only)
        #[arg(long, value_parser = utils::parse_timestamp_ns)]
        timestamp: Option<(u64, u32)>,

        /// UUID namespace (versions 3 and 5 only)
        #[arg(long, required_if_eq_any = [("version", "3"), ("version", "5")])]
        namespace: Option<uuid::SupportedUUIDNamespace>,

        /// UUID name (versions 3 and 5 only)
        #[arg(long, required_if_eq_any = [("version", "3"), ("version", "5")])]
        name: Option<String>,

        /// UUID node identifier (a MAC address; versions 1 and 6 only)
        #[arg(long)]
        node_id: Option<eui48::MacAddress>,

        /// UUID user data (hex-encoded; version 8 only)
        #[arg(long, value_parser = utils::parse_data, required_if_eq("version", "8"))]
        data: Option<[u8; 16]>,
    },

    /// Generate a new ULID
    #[command(
        long_about = "Generates a new Universally Unique Lexicographically Sortable Identifier."
    )]
    Ulid {
        /// ULID timestamp (in milliseconds)
        #[arg(long, value_parser = value_parser!(u64))]
        timestamp: Option<u64>,
    },

    /// Generate a new ObjectId
    #[command(
        name = "oid",
        alias = "objectid",
        long_about = "Generates a new MongoDB/BSON ObjectId."
    )]
    ObjectId {
        /// ObjectId timestamp (in seconds)
        #[arg(long, value_parser = value_parser!(u32))]
        timestamp: Option<u32>,
    },
}

impl Args {
    /// Parses command-line arguments with additional custom validation.
    ///
    /// This extends `clap`'s built-in validation with application-specific rules
    /// that are too complex to express declaratively. Currently validates:
    ///
    /// - UUID timestamps are only used with compatible versions (v1, v6, v7)
    ///
    /// # Panics
    ///
    /// Calls `std::process::exit` if validation fails, printing an error message
    /// to stderr in the same style as `clap` errors.
    pub(crate) fn parse() -> Self {
        let args = <Self as Parser>::parse();

        if let Err(err) = validation::validate_args(&args.command) {
            let cmd = <Self as CommandFactory>::command();

            match err {
                validation::ValidationError::UuidTimestampVersionMismatch { version } => {
                    let mut clap_err = clap::Error::new(ErrorKind::ArgumentConflict).with_cmd(&cmd);
                    clap_err.insert(
                        ContextKind::InvalidArg,
                        ContextValue::String("--timestamp".to_owned()),
                    );
                    clap_err.insert(
                        ContextKind::PriorArg,
                        ContextValue::String("--version ".to_owned() + &version.to_string()),
                    );
                    clap_err.exit();
                }
            }
        }

        args
    }
}
