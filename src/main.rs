#![warn(clippy::pedantic)]
#![deny(missing_debug_implementations, clippy::all)]

// Disabled due to spurius E0753 when adding //!
// #![deny(missing_docs)]

use image::Luma;
use qrcode::QrCode;
use std::env;
use std::fs;
use std::io::Read;
use std::io::Write;
use std::io::{Seek, SeekFrom};
use std::path::Path;
use std::process;

extern crate base64;
extern crate image;
extern crate qrcode;

fn main() {
    // Check arguments for file to open
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        let binary_name = &args[0];
        println!("Usage: {} FILE_TO_SEND OUTPUT_FOLDER", binary_name);
        process::exit(1);
    }
    let input_filename = &args[1];
    let output_folder = &args[2];
    let mut input_file = match fs::File::open(input_filename) {
        Ok(f) => f,
        Err(err) => panic!("File error: {}", err),
    };

    // Ensure output folder exists
    fs::create_dir_all(output_folder).expect("Could not create/check output folder");
    let output_path = Path::new(output_folder);
    // Create a base64 version of our file
    let mut base64_file = match fs::File::create(output_path.join("input_b64.txt")) {
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

    let mut base64_file = fs::File::open(output_path.join("input_b64.txt"))
        .expect("Error reopening the base64 file to chunk");

    let chunk_size: usize = 1024; // 1 KB

    let chunk_totals = (base64_filesize_bytes as f64 / (chunk_size as f64)).ceil(); // round UP on f64 division
    println!(
        "File {}. base64 size: {} bytes = {} chunks of 1KB",
        input_filename, base64_filesize_bytes, chunk_totals
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
            .save(output_path.join(format!("{}.png", chunk_count)))
            .expect("Error saving chunk's QR code file");

        println!("Saved QR {}/{}", chunk_count, chunk_totals);
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
        output_path
    );
}
