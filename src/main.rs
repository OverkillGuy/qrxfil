#![feature(seek_stream_len)]
#![warn(clippy::pedantic)]

use std::io::Read;
use std::io::Write;
use std::io::{BufRead, BufReader};
use std::{fs, path::PathBuf};
// use std::io::{BufRead, BufReader};
use std::io::{Seek, SeekFrom};
use std::process;

extern crate base64;

mod chunk_iterator;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Args {
    #[structopt(short, long = "input")]
    pub input_file: PathBuf,
    #[structopt(short, long = "output")]
    pub output_directory: PathBuf,
}

fn main() {
    let args = Args::from_args();
    // Check arguments for file to open
    let input_filename = args.input_file;
    let mut file = match fs::File::open(input_filename) {
        Ok(f) => f,
        Err(err) => panic!("File error: {}", err),
    };

    // Ensure output folder exists
    fs::create_dir_all(&args.output_directory).expect("Could not create/check output folder");
    let output_path = args.output_directory.join("input_b64.txt");
    // Create a base64 version of our file
    let mut base64_file = match fs::File::create(output_path.join("input_b64.txt")) {
        Ok(f) => f,
        Err(err) => panic!("File error: {}", err),
    };

    // Read the input file and write it out as base64
    {
        // Separate scope to drop the encoder
        let mut raw_data_buffer = Vec::<u8>::new();
        file.read_to_end(&mut raw_data_buffer)
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
    println!("File size: {}", base64_filesize_bytes);

    // rewind to the start of base64 encoded file to fan-out into chunks
    base64_file
        .seek(SeekFrom::Start(0))
        .expect("Error rewinding base64 file before chunking");

    base64_file
        .sync_all()
        .expect("Error syncing base64 file to disks");

    let base64_filesize_bytes = base64_file
        .stream_len()
        .expect("Error checking base64 filesize");

    let mut chunk_reader;
    let chunk_read;
    {
        chunk_reader = BufReader::with_capacity(1024, base64_file);

        chunk_read = chunk_reader
            .fill_buf()
            .expect("Error reading chunk off base64 file");
    }
}
