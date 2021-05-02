// qrxfil - exfiltrate files with QR codes
// Copyright (C) 2021 Jb Doyon
//
// This program is free software: you can redistribute it and/or
// modify it under the terms of the GNU General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.
//
// This program is distributed in the hope that it will be useful, but
// WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
// General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program. If not, see
// <https://www.gnu.org/licenses/>.

//! # qrxfil - Use QR codes for file exfiltration
//!
//! Use QR codes to "send" files as a sequence of QR codes, to scan on
//! another system, bypassing air-gap systems.
//!
//! Encodes the given file to base64, then generates a sequence of QR
//! code PNG images in target folder containing the split data. QR
//! code "chunks" contain ~1KB of encoded payload, prefixed with a
//! chunk number such as 070OF076, much like page numbers of a book.

#![warn(clippy::pedantic)]
#![deny(missing_debug_implementations, clippy::all)]
#![deny(missing_docs)]

use clap::{crate_version, App, Arg, ArgMatches, SubCommand};
use image::Luma;
use qr_code;
use qrcode::QrCode;
use std::fs;
use std::io::{BufRead, BufReader, Read, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};

mod chunk_iterator;
mod csv_decode;
mod parser;
mod pdf;

/// Size of a header in bytes
/// Three digits twice (id / total) plus delimiter string "OF" e.g.
/// 013OF078
const HEADER_SIZE_BYTES: u64 = 8;

const CHUNK_SIZE: u64 = 1024; // down from 1 KB

fn rewrite_as_base64(input_filename: &Path, output_folder: &Path) -> (PathBuf, u64) {
    let mut input_file = match fs::File::open(input_filename) {
        Ok(f) => f,
        Err(err) => panic!("File error: {}", err),
    };
    let base64_filename = output_folder.join("input_b64.txt");
    // Create a base64 version of our file
    let mut base64_file = match fs::File::create(base64_filename.clone()) {
        Ok(f) => f,
        Err(err) => panic!("File error: {}", err),
    };

    // Read the input file and write it out as base64
    {
        // Separate scope to drop the encoder
        let mut raw_data_buffer = Vec::<u8>::new();
        input_file
            .read_to_end(&mut raw_data_buffer)
            .expect("Error reading input file");

        // Write base64 version of what we read
        let mut base64_encoder =
            base64::write::EncoderWriter::new(&mut base64_file, base64::STANDARD);

        base64_encoder
            .write_all(&raw_data_buffer)
            .expect("Error writing base64 of input file");
    }
    // Measure file length from where we are at the end
    let base64_filesize_bytes = base64_file
        .seek(SeekFrom::End(0))
        .expect("Error checking base64 filesize");

    base64_file
        .sync_data()
        .expect("Error syncing base64 file to disk");

    (base64_filename, base64_filesize_bytes)
}

/// Encodes `input_file` with qrxfil into QR text files inside `output_folder`
///
/// `output_folder` (and parent directories) will be created if
/// doesn't exist
fn encode_txt(input_filename: &Path, output_folder: &Path) {
    // Ensure output folder exists
    fs::create_dir_all(output_folder).expect("Could not create/check output folder");

    let (base64_filename, base64_filesize_bytes) = rewrite_as_base64(input_filename, output_folder);
    let base64_file =
        fs::File::open(base64_filename).expect("Error reopening the base64 file to chunk");
    let chunk_iter = chunk_iterator::ChunkIterator::new(
        BufReader::new(base64_file),
        base64_filesize_bytes,
        CHUNK_SIZE - HEADER_SIZE_BYTES,
    );
    let chunk_total = chunk_iter.chunk_total;
    println!(
        "File {:?}. base64 size: {} bytes = {} chunks of 1KB",
        input_filename.to_str(),
        base64_filesize_bytes,
        chunk_total
    );

    for c in chunk_iter {
        let chunk = c.expect("Problem reading reader");
        // Encode some data into bits.
        let code = qr_code::QrCode::new(format!("{}", chunk).as_bytes())
            .expect("Error encoding chunk into QR code");

        // Save the image.
        fs::write(
            output_folder.join(format!("{:03}.txt", chunk.id)),
            code.to_string(false, 3),
        )
        .unwrap();
        // .expect("Error saving chunk's QR code file");

        println!("Saving QR {:03}/{}", chunk.id, chunk.total);
    }
    println!(
        "Split file in {} QR chunks, in folder {:?}",
        chunk_total, output_folder
    );
}

/// Encodes `input_file` with qrxfil into QR image files inside `output_folder`
///
/// `output_folder` (and parent directories) will be created if
/// doesn't exist
fn encode_png(input_filename: &Path, output_folder: &Path) {
    // Ensure output folder exists
    fs::create_dir_all(output_folder).expect("Could not create/check output folder");

    let (base64_filename, base64_filesize_bytes) = rewrite_as_base64(input_filename, output_folder);
    let base64_file =
        fs::File::open(base64_filename).expect("Error reopening the base64 file to chunk");
    let chunk_iter = chunk_iterator::ChunkIterator::new(
        BufReader::new(base64_file),
        base64_filesize_bytes,
        CHUNK_SIZE - HEADER_SIZE_BYTES,
    );
    let chunk_total = chunk_iter.chunk_total;
    println!(
        "File {:?}. base64 size: {} bytes = {} chunks of 1KB",
        input_filename.to_str(),
        base64_filesize_bytes,
        chunk_total
    );

    for c in chunk_iter {
        let chunk = c.expect("Problem reading reader");
        // Encode some data into bits.
        let code = QrCode::new(format!("{}", chunk).as_bytes())
            .expect("Error encoding chunk into QR code");

        // Render the bits into an image.
        let image = code.render::<Luma<u8>>().build();

        // Save the image
        image
            .save(output_folder.join(format!("{:03}.png", chunk.id)))
            .expect("Error saving chunk's QR code file");
        println!("Saving QR {:03}/{}", chunk.id, chunk.total);
    }
    println!(
        "Split file in {} QR chunks, in folder {:?}",
        chunk_total, output_folder
    );
}

/// Decodes QR strings found in `input_path` (newline-separated) with
/// qrxfil to restore file to `restored_file`
fn decode(input_path: &Path, restored_path: &Path) -> Result<(), parser::RestoreError> {
    let input_file = match fs::File::open(input_path) {
        Ok(f) => f,
        Err(err) => panic!("File error on opening decode input: {}", err),
    };

    let reader = BufReader::new(input_file);

    let mut chunks = Vec::<parser::EncodedChunk>::new();
    for line in reader.lines() {
        let l = line.expect("Error reading a line off input file");
        let chunk = parser::parse(&l);
        match chunk {
            Ok(c) => chunks.push(c),
            Err(err) => {
                println!("Erroring {:?} ", err);
                return Err(parser::RestoreError::ChunkDecodeError {
                    error: err,
                    raw_chunk: l,
                });
            }
        }
    }
    reassemble(&mut chunks, restored_path)
}

fn reassemble(
    chunks: &mut Vec<parser::EncodedChunk>,
    restored_path: &Path,
) -> Result<(), parser::RestoreError> {
    // re-sort the chunks for out-of-order scanning
    chunks.sort_by_key(|chunk| chunk.id);

    let chunks = parser::check_chunk_range(&chunks)?;

    let concatenated_chunk_payloads = chunks
        .iter()
        .map(|c| c.payload.clone())
        .collect::<Vec<String>>()
        .concat();

    let mut restored_file = match fs::File::create(restored_path) {
        Ok(f) => f,
        Err(err) => panic!("File error creating restored file: {}", err),
    };

    let decoded_contents =
        base64::decode(concatenated_chunk_payloads).expect("Error base64 decoding file");
    restored_file
        .write_all(&decoded_contents)
        .expect("Error writing out restored file chunk");
    Ok(())
}

fn decode_csv(input_path: &Path, restored_path: &Path) -> Result<(), parser::RestoreError> {
    let input_file = match fs::File::open(input_path) {
        Ok(f) => f,
        Err(err) => panic!("Error opening file for decoding: {}", err),
    };

    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(b';')
        .has_headers(true)
        .double_quote(true)
        .flexible(true)
        .from_reader(input_file);
    let mut scans: Vec<parser::EncodedChunk> = rdr
        .deserialize()
        .map(|line| {
            let record: csv_decode::Scan =
                line.expect("Error deserializing CSV line into Scan record");
            record
        })
        .filter(|s| s.format == "QR_CODE")
        .map(|s| parser::parse(&s.content).expect("Bad chunk"))
        .collect();
    reassemble(&mut scans, restored_path)
}

fn main() {
    let matches = get_args();
    std::process::exit(match run(&matches) {
        Ok(_) => 0,
        Err(err) => {
            eprintln!("{}", err);
            1
        }
    });
}

fn get_args() -> ArgMatches<'static> {
    App::new("qrxfil")
        .version(crate_version!())
        .about("Transfer/backup files as a sequence of QR codes (requires pandoc)")
        .author("Jb DOYON")
        .subcommand(
            SubCommand::with_name("exfil")
                .about("Generates QR code image sequence from file")
                .arg(
                    Arg::with_name("input")
                        .help("The input file to split into QR codes")
                        .index(1)
                        .required(true),
                )
                .arg(
                    Arg::with_name("output_folder")
                        .help("The output folder to generate codes into")
                        .index(2)
                        .required(true),
                )
                .arg(
                    Arg::with_name("txt")
                        .short("t")
                        .long("txt")
                        .help("Export QR codes as UTF8 text files not PNG"),
                ),
        )
        .subcommand(
            SubCommand::with_name("restore")
                .about("Decodes encoded strings back into file")
                .arg(
                    Arg::with_name("csv")
                        .short("c")
                        .long("csv")
                        .help("Parse the given file as CSV"),
                )
                .arg(
                    Arg::with_name("encoded_input")
                        .help("The input file with newline-delimited QR strings")
                        .index(1)
                        .required(true),
                )
                .arg(
                    Arg::with_name("output_file")
                        .help("The output file to restore into")
                        .index(2)
                        .required(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("pdfprint")
                .about("Generates a PDF of QR codes from file")
                .arg(
                    Arg::with_name("input")
                        .help("The input file to split into QR codes")
                        .index(1)
                        .required(true),
                )
                .arg(
                    Arg::with_name("output_folder")
                        .help("The output folder to generate PDF into")
                        .index(2)
                        .required(true),
                ),
        )
        .get_matches()
}

fn run(matches: &ArgMatches<'static>) -> Result<(), parser::RestoreError> {
    if let Some(matches_exfil) = matches.subcommand_matches("exfil") {
        let input_filename = matches_exfil.value_of("input").unwrap();
        let output_folder = matches_exfil.value_of("output_folder").unwrap();

        if matches_exfil.is_present("txt") {
            encode_txt(Path::new(input_filename), Path::new(output_folder));
        } else {
            encode_png(Path::new(input_filename), Path::new(output_folder));
        }
        return Ok(());
    }
    if let Some(matches_printpdf) = matches.subcommand_matches("pdfprint") {
        let input_filename = matches_printpdf.value_of("input").unwrap();
        let output_folder = matches_printpdf.value_of("output_folder").unwrap();

        encode_png(Path::new(input_filename), Path::new(output_folder));
        pdf::genpandoc(Path::new(output_folder));
    }
    if let Some(matches_restore) = matches.subcommand_matches("restore") {
        let encoded_input_filename = matches_restore.value_of("encoded_input").unwrap();
        let output_file = matches_restore.value_of("output_file").unwrap();

        if matches_restore.is_present("csv") {
            return decode_csv(Path::new(encoded_input_filename), Path::new(output_file));
        }
        return decode(Path::new(encoded_input_filename), Path::new(output_file));
    }
    Ok(())
}
