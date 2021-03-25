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

use clap::{App, Arg, ArgMatches, SubCommand};
use image::Luma;
use qrcode::QrCode;
use std::{
    fs,
    io::{BufRead, BufReader, Read, Seek, SeekFrom, Write},
    path::Path,
};
extern crate base64;
extern crate clap;
extern crate image;
extern crate qrcode;

mod parser;
mod payload_size;

/// Encodes `input_file` with qrxfil into QR files inside `output_folder`
///
/// `output_folder` (and parent directories) will be created if
/// doesn't exist
fn encode(input_file: &Path, output_folder: &Path) {
    let mut input_file = match fs::File::open(input_file) {
        Ok(f) => f,
        Err(err) => panic!("File error: {}", err),
    };

    // Ensure output folder exists
    fs::create_dir_all(output_folder).expect("Could not create/check output folder");
    // Create a base64 version of our file
    let mut base64_file = match fs::File::create(output_folder.join("input_b64.txt")) {
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

    let mut base64_file = fs::File::open(output_folder.join("input_b64.txt"))
        .expect("Error reopening the base64 file to chunk");

    let chunk_size: usize = 1024; // 1 KB

    #[allow(clippy::cast_precision_loss)]
    let chunk_totals = (base64_filesize_bytes as f64 / (chunk_size as f64)).ceil(); // round UP on f64 division
    println!(
        "File {:?}. base64 size: {} bytes = {} chunks of 1KB",
        input_file, base64_filesize_bytes, chunk_totals
    );
    let mut chunk_count = 1;
    let header_size = format!("{:03}OF{:03}", 1, 10).len();
    let expected_chunk_bytes_read = chunk_size - header_size;
    loop {
        let mut chunk_header: Vec<u8> =
            format!("{:03}OF{:03}", chunk_count, chunk_totals).into_bytes();

        let mut chunk = Vec::<u8>::with_capacity(chunk_size);
        chunk.append(&mut chunk_header); // FIXME write prefix to buffer

        let bytes_read_chunk = std::io::Read::by_ref(&mut base64_file)
            .take(expected_chunk_bytes_read as u64)
            .read_to_end(&mut chunk)
            .expect("Error reading chunk off file");
        if bytes_read_chunk == 0 {
            break;
        }

        // Encode some data into bits.
        let code = QrCode::new(&chunk).expect("Error encoding chunk into QR code");

        // Render the bits into an image.
        let image = code.render::<Luma<u8>>().build();

        // Save the image.
        image
            .save(output_folder.join(format!("{:03}.png", chunk_count)))
            .expect("Error saving chunk's QR code file");

        println!("Saved QR {:03}/{}", chunk_count, chunk_totals);
        // let mut out_file = fs::File::create())

        // out_file
        //     .write(chunk_header.as_bytes())
        //     .expect("Error writing out file chunk header");
        // out_file
        //     .write_all(&chunk)
        //     .expect("Error writing out file chunk");
        chunk_count += 1;
    }
    println!(
        "Split file in {} QR chunks, in folder {:?}",
        chunk_count - 1,
        output_folder
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
    .version("0.1")
    .about("Transfer/backup files as a sequence of QR codes")
    .author("Jb DOYON") // And authors
    .subcommand(
        SubCommand::with_name("exfil")
            .about("Generates QR code sequence from file")
            .arg(
                Arg::with_name("input") // And their own arguments
                    .help("The input file to split into QR codes")
                    .index(1)
                    .required(true),
            )
            .arg(
                Arg::with_name("output_folder")
                    .help("The output folder to generate codes into")
                    .index(2)
                    .required(true),
            ),
    )
    .subcommand(
        SubCommand::with_name("restore")
            .about("Decodes encoded strings back into file"
        )
            .arg(
                Arg::with_name("encoded_input") // And their own arguments
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

).get_matches()
}

fn run(matches: &ArgMatches<'static>) -> Result<(), parser::RestoreError> {
    if let Some(matches_exfil) = matches.subcommand_matches("exfil") {
        let input_filename = matches_exfil.value_of("input").unwrap();
        let output_folder = matches_exfil.value_of("output_folder").unwrap();

        encode(Path::new(input_filename), Path::new(output_folder));
        return Ok(());
    }
    if let Some(matches_restore) = matches.subcommand_matches("restore") {
        let encoded_input_filename = matches_restore.value_of("encoded_input").unwrap();
        let output_file = matches_restore.value_of("output_file").unwrap();

        return decode(Path::new(encoded_input_filename), Path::new(output_file));
    }
    Ok(())
}
