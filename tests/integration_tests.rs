use rand::Rng;
use std::io::Read;
use std::io::Seek;
use std::io::SeekFrom;
use std::io::Write;
use test_temp_file::TestTempFile;

// Scenario: qrxfil on random file generates PNG files
#[test]
fn file_to_qr_happy() {
    // Given a file with a few KB of random data
    let original_filename = "Test_file.bin";
    let mut tmp_file = TestTempFile::new(original_filename.to_string());

    let size_bytes: i32 = 1024; // 1KB right now
    let mut rng = rand::thread_rng();
    let random_chars: Vec<u8> = (0..size_bytes).map(|_| rng.gen_range(0..255)).collect();
    tmp_file.write_all(&random_chars).unwrap();

    // Rewind to read back the numbers
    let _s = tmp_file.seek(SeekFrom::Start(0));
    // read the whole file
    let mut buffer = Vec::new();
    tmp_file.read_to_end(&mut buffer).unwrap();
    println!("{:?}", buffer);

    // When running qrxfil with it
    // Then a folder is named after the file
    // And folder contains dozens of files
    // And the files are all valid PNG
    assert_eq!(1, 1);
}
