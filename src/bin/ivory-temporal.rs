#![forbid(unsafe_code)]

use std::error::Error;
use std::fs::{self, OpenOptions};
use std::io::{self, Write};
use std::process::ExitCode;

use ivory_kleio::{QueryResult, Snapshot, could_occur_in_year, validate_year};
use serde::Serialize;

#[derive(Serialize)]
struct Report {
    query_year: i32,
    round_trip_verified: bool,
    results: Vec<QueryResult>,
}

fn evaluate(snapshot: &Snapshot, year: i32) -> Result<Vec<QueryResult>, ivory_kleio::Error> {
    snapshot.validate()?;
    validate_year(year)?;
    snapshot
        .records
        .iter()
        .map(|record| could_occur_in_year(record, year))
        .collect()
}

fn run() -> Result<(), Box<dyn Error>> {
    let args: Vec<_> = std::env::args_os().skip(1).collect();
    if args.len() != 3 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "usage: cargo run --bin ivory-temporal -- INPUT_JSON YEAR NEW_OUTPUT_JSON",
        )
        .into());
    }
    let year: i32 = args[1]
        .to_str()
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "YEAR must be an integer"))?
        .parse()?;
    validate_year(year)?;
    let snapshot = Snapshot::from_json(&fs::read_to_string(&args[0])?)?;
    let results = evaluate(&snapshot, year)?;
    let serialized = snapshot.to_json()?;

    let mut output = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&args[2])?;
    output.write_all(serialized.as_bytes())?;
    output.sync_all()?;
    drop(output);

    let reloaded = Snapshot::from_json(&fs::read_to_string(&args[2])?)?;
    let reloaded_results = evaluate(&reloaded, year)?;
    if reloaded != snapshot || reloaded_results != results {
        return Err(
            io::Error::other("snapshot or query result changed after save and reload").into(),
        );
    }
    let report = Report {
        query_year: year,
        round_trip_verified: true,
        results,
    };
    let stdout = io::stdout();
    let mut stdout = stdout.lock();
    serde_json::to_writer_pretty(&mut stdout, &report)?;
    writeln!(stdout)?;
    Ok(())
}

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("{error}");
            ExitCode::FAILURE
        }
    }
}
