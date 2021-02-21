use std::env;
use std::fs;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use std::process;

fn main() {
    // Check arguments for file to open
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("Usage: {} FILE_TO_SEND OUTPUT_FOLDER", args[0]);
        process::exit(1);
    }
    let input_filename = &args[1];
    let output_folder = &args[2];
    let mut file = match fs::File::open(input_filename) {
        Ok(f) => f,
        Err(err) => panic!("File error: {}", err),
    };

    // Read the file and print contents (ASCII numbers)
    let mut buffer = Vec::<u8>::new();
    file.read_to_end(&mut buffer)
        .expect("Error reading input file");
    println!("{:?}", buffer);

    // Ensure output folder exists
    fs::create_dir_all(output_folder).expect("Could not create/check output folder");
    let output_path = Path::new(output_folder);
    let mut output_file = match fs::File::create(output_path.join("1.png")) {
        Ok(f) => f,
        Err(err) => panic!("File error: {}", err),
    };
    output_file
        .write_all(&buffer)
        .expect("Error populating the folder with binary content");
}
