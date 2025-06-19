//! IdGen - A command-line utility for generating unique identifiers.
//!
//! This application generates various types of unique identifiers (UUIDs, ULIDs, ObjectIds)
//! with configurable parameters. It's designed as a standalone CLI tool for use in shell
//! scripts, development workflows, and anywhere unique identifiers are needed.
//!
//! # Architecture
//!
//! The application follows a modular design:
//!
//! - [`cli`]: Command-line interface definitions and argument parsing
//! - [`generators`]: Identifier generator implementations (UUID, ULID, ObjectId)
//! - [`utils`]: Shared utility functions for parsing and data generation
//!
//! # Flow
//!
//! ```text
//! CLI Args (clap) → Generator (enum) → Specific Generator → String Output
//! ```
//!
//! 1. Arguments are parsed using `clap` with custom validation
//! 2. A `Generator` enum is created based on the subcommand
//! 3. The generator produces the requested number of identifiers
//! 4. Identifiers are written to stdout, one per line

mod cli;
mod generators;
mod utils;

use std::io::{self, Write};

use crate::cli::Args;
use crate::generators::{Generate, Generator};

fn main() -> anyhow::Result<()> {
    // Parsing the CLI arguments
    let args = Args::parse();

    // Creating an appropriate generator from the command
    let generator = Generator::from(&args.command);

    // Locking stdout for efficient buffered writing
    let mut stdout = io::stdout().lock();

    // Running it as many times as specified
    for _ in 0..args.number {
        writeln!(stdout, "{}", generator.generate())?;
    }

    Ok(())
}
