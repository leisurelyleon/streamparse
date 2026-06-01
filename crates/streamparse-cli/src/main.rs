//! `streamparse` command-line entry point.

use std::fs::File;
use std::io::{self, BufReader, Read, Write};

use clap::Parser;

use streamparse_cli::cli::{Cli, Command, FormatKind};
use streamparse_core::{CoreError, Format, ParseEvent, RecordPayload, StreamParser};
use streamparse_formats::{Delimited, Ndjson};

fn main() -> io::Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Command::Parse { format, input } => run(format, &input, false),
        Command::Stats { format, input } => run(format, &input, true),
    }
}

fn run(format: FormatKind, input: &str, stats_only: bool) -> io::Result<()> {
    let reader: Box<dyn Read> = if input == "-" {
        Box::new(BufReader::new(io::stdin()))
    } else {
        Box::new(BufReader::new(File::open(input)?))
    };

    match format {
        FormatKind::Ndjson => stream(Ndjson, reader, stats_only),
        FormatKind::Delimited => stream(Delimited::comma(), reader, stats_only),
    }
}

/// Drives the parser over a reader in bounded-memory chunks.
fn stream<F: Format>(format: F, mut reader: Box<dyn Read>, stats_only: bool) -> io::Result<()> {
    let mut parser = StreamParser::new(format);
    let mut chunk = [0u8; 8192];
    let stdout = io::stdout();
    let mut out = stdout.lock();

    let mut count: u64 = 0;
    let mut bytes: u64 = 0;

    loop {
        let read = reader.read(&mut chunk)?;
        if read == 0 {
            break;
        }
        let events = parser.feed(&chunk[..read]).map_err(to_io_error)?;
        emit(&mut out, &events, stats_only, &mut count, &mut bytes)?;
    }

    let events = parser.finish().map_err(to_io_error)?;
    emit(&mut out, &events, stats_only, &mut count, &mut bytes)?;

    if stats_only {
        writeln!(out, "records: {count}")?;
        writeln!(out, "record bytes: {bytes}")?;
    }
    Ok(())
}

fn emit(
    out: &mut impl Write,
    events: &[ParseEvent],
    stats_only: bool,
    count: &mut u64,
    bytes: &mut u64,
) -> io::Result<()> {
    for event in events {
        *count += 1;
        *bytes += event.raw_len as u64;
        if !stats_only {
            print_event(out, event)?;
        }
    }
    Ok(())
}

fn print_event(out: &mut impl Write, event: &ParseEvent) -> io::Result<()> {
    let index = event.index;
    match &event.payload {
        RecordPayload::Json(text) => writeln!(out, "[{index}] json {text}"),
        RecordPayload::Fields(fields) => {
            let joined = fields.join(" | ");
            writeln!(out, "[{index}] fields {joined}")
        }
    }
}

fn to_io_error(err: CoreError) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, err)
}
