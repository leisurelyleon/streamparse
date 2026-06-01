//! Command-line argument definitions.

use clap::{Parser, Subcommand, ValueEnum};

/// A streaming, incremental data parser.
#[derive(Debug, Parser)]
#[command(name = "streamparse", version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Stream-parse input and print one line per record.
    Parse {
        #[arg(long, value_enum, default_value = "ndjson")]
        format: FormatKind,
        /// Input path, or "-" for stdin.
        input: String,
    },
    /// Stream-parse input and print aggregate statistics only.
    Stats {
        #[arg(long, value_enum, default_value = "ndjson")]
        format: FormatKind,
        /// Input path, or "-" for stdin.
        input: String,
    },
}

/// The supported input formats.
#[derive(Clone, Debug, ValueEnum)]
pub enum FormatKind {
    Ndjson,
    Delimited,
}
