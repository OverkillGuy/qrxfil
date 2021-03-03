use clap::Clap;
use std::{fs, fs::File, io, io::BufReader, path::PathBuf};

/// Generates QR code sequence from file
#[derive(Debug, Clap)]
pub struct Exfil {
    /// The input file to split into QR codes
    input: PathBuf,

    /// The output folder to generate codes into
    output_folder: PathBuf,
}

impl Exfil {
    pub fn run(&self) -> io::Result<()> {
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
