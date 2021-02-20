use std::env;
use std::fs::File;
use std::io::Read;
use std::process;

fn main() {
    // Check arguments for file to open
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} FILE_TO_SEND", args[0]);
        process::exit(1);
    }
    let mut file = match File::open(&args[1]) {
        Ok(f) => f,
        Err(err) => panic!("File error: {}", err),
    };

    // Read the file and print contents (ASCII numbers)
    let mut buffer = Vec::<u8>::new();
    file.read_to_end(&mut buffer).unwrap();
    println!("{:?}", buffer);
}
