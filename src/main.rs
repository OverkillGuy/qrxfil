//! These are module-level docs

#![warn(clippy::pedantic)]
#![deny(missing_debug_implementations, clippy::all)]
#![deny(missing_docs)]

use image::Luma;
use qrcode::QrCode;
use std::{fs, io::BufReader, path::PathBuf};
use std::io::Read;
use std::io::Write;
use std::io::{Seek, SeekFrom};
use std::path::Path;
extern crate base64;
extern crate clap;
extern crate image;
extern crate qrcode;
use std::io;
use std::fs::File;

use clap::Clap;

/// Transfer/backup files as a sequence of QR codes
#[derive(Debug, Clap)]
pub(crate) enum Command {
    Exfil(Exfil)
}

impl Command {
    pub(crate) fn run(&self) -> io::Result<()> {
        match self {
            Self::Exfil(exfil) => exfil.run()
        }
    }
}

/// Generates QR code sequence from file
#[derive(Debug, Clap)]
pub(crate) struct Exfil {
    /// The input file to split into QR codes
    input: PathBuf,

    /// The output folder to generate codes into
    output_folder: PathBuf,
}

impl Exfil {
    pub(crate) fn run(&self) -> io::Result<()> {
        // ensure the output folder exists
        fs::create_dir_all(&self.output_folder)?;

        // read the input file
        let mut input = BufReader::new(File::open(&self.input)?);

        // create a base64 encoded output file
        let base64_file = File::create(self.output_folder.join("input_b64.txt"))?;
        let mut base64_encoder = base64::write::EncoderWriter::new(base64_file, base64::STANDARD);

        // copy everything from the input file to the base64 file
        std::io::copy(&mut input, &mut base64_encoder)?;

        // .... and so on

        Ok(())
    }
}

fn main() -> io::Result<()>{
    let command = Command::parse();
    command.run()
}

/*     // Measure file length from where we are at the end
    let base64_filesize_bytes = base64_file
        .seek(SeekFrom::End(0))
        .expect("Error checking base64 filesize");

    base64_file
        .sync_data()
        .expect("Error syncing base64 file to disk");

    let mut base64_file = fs::File::open(output_path.join("input_b64.txt"))
        .expect("Error reopening the base64 file to chunk");

    let chunk_size: usize = 1024; // 1 KB

    #[allow(clippy::cast_precision_loss)]
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
            .save(output_path.join(format!("{:02}.png", chunk_count)))
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
        output_path
    );
}
 */
