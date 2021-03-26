#!/usr/bin/env rust-script
//! A sample CSV reader script in rust to parse
//! From https://docs.rs/csv/1.1.6/csv/#example-with-serde
//!
//! ```cargo
//! [dependencies]
//! csv = "1.1"
//! serde = { version = "1", features = ["derive"] }
//! ```
use std::error::Error;
use std::io;
use std::process;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Scan {
    _datetime: String,
    format: String,
    content: String,
    error_correction_level: String,
}

fn example() -> Result<(), Box<dyn Error>> {
    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(b';')
        .has_headers(true)
        .double_quote(true)
        .flexible(true)
        .from_reader(io::stdin());
    for result in rdr.deserialize() {
        // Notice that we need to provide a type hint for automatic
        // deserialization.
        let record: Scan = result?;
        println!("{:?}", record);
    }
    Ok(())
}

fn main() {
    if let Err(err) = example() {
        println!("error running example: {}", err);
        process::exit(1);
    }
}
