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

use clap::{App, Arg, SubCommand};
use image::Luma;
use qrcode::QrCode;
use std::fs;
use std::io::Write;
use std::io::{BufRead, BufReader, Read};
use std::io::{Seek, SeekFrom};
use std::path::Path;
extern crate base64;
extern crate clap;
extern crate image;
extern crate qrcode;

mod parser;

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
    let header_size = format!("{:02}OF{:02}", 1, 10).len();
    let expected_chunk_bytes_read = chunk_size - header_size;
    loop {
        let mut chunk_header: Vec<u8> =
            format!("{:02}OF{:02}", chunk_count, chunk_totals).into_bytes();

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
            .save(output_folder.join(format!("{:02}.png", chunk_count)))
            .expect("Error saving chunk's QR code file");

        println!("Saved QR {:02}/{}", chunk_count, chunk_totals);
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
///
fn decode(input_path: &Path, restored_path: &Path) {
    let input_file = match fs::File::open(input_path) {
        Ok(f) => f,
        Err(err) => panic!("File error on opening decode input: {}", err),
    };

    let reader = BufReader::new(input_file);

    let chunks: Vec<parser::EncodedChunk> = reader
        .lines()
        .map(|l| parser::parse(&l.unwrap()).expect("Invalid chunk read"))
        .collect();

    // TODO re-sort the chunks if needed

    let mut restored_file = match fs::File::create(restored_path) {
        Ok(f) => f,
        Err(err) => panic!("File error creating restored file: {}", err),
    };
    for chunk in chunks {
        println!("{}/{}", chunk.id, chunk.total);
        let decoded_chunk_bytes =
            base64::decode(chunk.payload).expect("Error base64 decoding chunk");
        restored_file
            .write_all(&decoded_chunk_bytes)
            .expect("Error writing out restored file chunk");
    }
    // panic!("Noooo");
}

fn main() {
    let matches = App::new("qrxfil")
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
                .about("Decodes encoded strings back into file")
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

	)
        .get_matches();

    // // You can check if a subcommand was used like normal
    // let matches_exfil = match matches.subcommand_matches("exfil") {
    //     Some(i) => i,
    //     None => panic!("Exfil alone is implemented"),
    // };

    if let Some(matches_exfil) = matches.subcommand_matches("exfil") {
        let input_filename = matches_exfil.value_of("input").unwrap();
        let output_folder = matches_exfil.value_of("output_folder").unwrap();

        encode(Path::new(input_filename), Path::new(output_folder));
    }
    if let Some(matches_restore) = matches.subcommand_matches("restore") {
        let encoded_input_filename = matches_restore.value_of("encoded_input").unwrap();
        let output_file = matches_restore.value_of("output_file").unwrap();

        decode(Path::new(encoded_input_filename), Path::new(output_file));
    }
}
