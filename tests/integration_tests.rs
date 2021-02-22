use assert_cmd::Command;
use assert_fs::prelude::*;
use predicates::prelude::*;
use rand::Rng;

// Scenario: qrxfil on random file generates PNG files
#[test]
fn file_to_qr_happy() {
    // Given a file with a few KB of random data
    let temp = assert_fs::TempDir::new().unwrap();
    let input_file = temp.child("to_send.bin");
    let output_folder = temp.child("output_qrs");

    // Fill input file with random data
    let size_bytes: i32 = 1024; // 1KB right now
    let mut rng = rand::thread_rng();
    let random_chars: Vec<u8> = (0..size_bytes).map(|_| rng.gen_range(0..255)).collect();
    input_file
        .write_binary(&random_chars)
        .expect("Could not write random to file for test seeding");

    // When running qrxfil with input filename + output folder
    let mut cmd = Command::cargo_bin("qrxfil").expect("Error find qrxfil command");
    // Then exit code is zero for success
    cmd.args(&[input_file.path(), output_folder.path()])
        .assert()
        .success();
    // Then a folder is created
    output_folder.assert(predicate::path::is_dir());
    // And folder contains files
    let output_files =
        std::fs::read_dir(output_folder.path()).expect("Could not list output directory");
    assert_ne!(
        output_files.count(),
        0,
        "Should have found files inside output directory"
    );
    // And a base64 version of input file is present
    output_folder
        .child("input_b64.txt")
        .assert(predicate::path::is_file());

    // clean up the temp folder
    temp.close().expect("Error deleting temporary folder");
}
