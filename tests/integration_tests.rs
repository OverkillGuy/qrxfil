use assert_cmd::Command;
use assert_fs::prelude::*;
use rand::Rng;

// Scenario: qrxfil on random file generates PNG files
#[test]
fn file_to_qr_happy() {
    // Given a file with a few KB of random data
    let temp = assert_fs::TempDir::new().unwrap();
    let input_file = temp.child("to_send.bin");

    // Fill input file with random data
    let size_bytes: i32 = 1024; // 1KB right now
    let mut rng = rand::thread_rng();
    let random_chars: Vec<u8> = (0..size_bytes).map(|_| rng.gen_range(0..255)).collect();
    input_file.write_binary(&random_chars).unwrap();

    // When running qrxfil with it
    let mut cmd = Command::cargo_bin("qrxfil").unwrap();
    // Then exit code is zero for success
    cmd.arg(input_file.path()).assert().success();
    // Then a folder is named after the file
    // And folder contains dozens of files
    // And the files are all valid PNG

    temp.close().unwrap(); // clean up the temp folder
}
