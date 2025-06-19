//! Identifier generator implementations and abstraction layer.
//!
//! This module provides a unified interface for generating different types of unique identifiers.
//! Each identifier type (UUID, ULID, ObjectId) has its own submodule with a generator struct that
//! implements the [`Generate`] trait.
//!
//! # Architecture
//!
//! The design uses a trait-based approach to allow polymorphic generation:
//!
//! 1. [`Generate`] trait: Common interface for all generators
//! 2. [`Generator`] enum: Top-level wrapper that dispatches to specific generators
//! 3. Type-specific generators: [`uuid::UuidGenerator`], [`ulid::UlidGenerator`], [`objectid::ObjectIdGenerator`]
//!
//! This pattern allows the main application logic to work with any generator type
//! without knowing the specifics of each identifier format.
//!
//! # Usage Flow
//!
//! ```text
//! CLI Args → Commands → Generator enum → Specific Generator → String output
//! ```
//!
//! The [`Generator::from`] implementation handles the conversion from CLI commands
//! to the appropriate generator instance.

pub mod objectid;
pub mod ulid;
pub mod uuid;

use crate::cli::Commands;

/// Common interface for identifier generators.
///
/// All generator types implement this trait to provide a uniform way
/// to generate identifiers as strings. This allows the main application to
/// remain agnostic to the specific identifier type being generated.
pub trait Generate {
    /// Generates a new identifier and returns it as a string.
    fn generate(&self) -> String;
}

/// Top-level generator wrapper that dispatches to specific identifier generators.
///
/// This enum allows the application to work with different generator types
/// polymorphically. It's constructed from CLI [`Commands`] and delegates
/// generation to the appropriate underlying generator.
pub enum Generator {
    Uuid(uuid::UuidGenerator),
    Ulid(ulid::UlidGenerator),
    ObjectId(objectid::ObjectIdGenerator),
}

impl Generate for Generator {
    fn generate(&self) -> String {
        match self {
            Generator::Uuid(g) => g.generate(),
            Generator::Ulid(g) => g.generate(),
            Generator::ObjectId(g) => g.generate(),
        }
    }
}

impl From<&Commands> for Generator {
    fn from(command: &Commands) -> Self {
        match command {
            Commands::Uuid {
                version,
                timestamp,
                namespace,
                name,
                node_id,
                data,
            } => Generator::Uuid(uuid::UuidGenerator::from_params(
                *version,
                *timestamp,
                namespace.as_ref(),
                name.as_ref(),
                node_id.as_ref(),
                data.as_ref(),
            )),
            Commands::Ulid { timestamp } => Generator::Ulid(ulid::UlidGenerator::new(*timestamp)),
            Commands::ObjectId { timestamp } => {
                Generator::ObjectId(objectid::ObjectIdGenerator::new(*timestamp))
            }
        }
    }
}
