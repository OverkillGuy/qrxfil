// qrxfil - exfiltrate files with QR codes
// Copyright (C) 2021 Jb Doyon
//
// This program is free software: you can redistribute it and/or
// modify it under the terms of the GNU General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.
//
// This program is distributed in the hope that it will be useful, but
// WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
// General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program. If not, see
// <https://www.gnu.org/licenses/>.

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

    // When running qrxfil in exfil-mode with input filename + output folder
    let mut cmd = Command::cargo_bin("qrxfil").expect("Error find qrxfil command");
    // Then exit code is zero for success
    let args = [
        "exfil",
        input_file.path().to_str().unwrap(),
        output_folder.path().to_str().unwrap(),
    ];
    println!("{} {} {}", &args[0], &args[1], &args[2]);
    cmd.args(&args).assert().success();
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
    // And the first chunk is created as qr code
    output_folder
        .child("001.png")
        .assert(predicate::path::is_file());

    // clean up the temp folder
    temp.close().expect("Error deleting temporary folder");
}

// Scenario: qrxfil on random file generates PDF file
#[test]
fn file_to_pdf_happy() {
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

    // When running qrxfil in PDF exfil-mode with input filename + output folder
    let mut cmd = Command::cargo_bin("qrxfil").expect("Error find qrxfil command");
    // Then exit code is zero for success
    let args = [
        "pdfprint",
        input_file.path().to_str().unwrap(),
        output_folder.path().to_str().unwrap(),
    ];
    cmd.args(&args).assert().success();
    // Then a PDF is created
    let pdf_filepath = output_folder.child("output.pdf");
    pdf_filepath.assert(predicate::path::is_file());
    // And the PDF has more than 1 page
    let pdf_file = pdf::file::File::<Vec<u8>>::open(&pdf_filepath.path()).unwrap();
    assert!(
        pdf_file.num_pages() > 1,
        "PDF must have more than guard page"
    );
    // clean up the temp folder
    temp.close().expect("Error deleting temporary folder");
}

// Scenario: qrxfil restore --csv
#[test]
fn csv_restored_happy() {
    // Given a file already split in 5 parts
    let temp = assert_fs::TempDir::new().unwrap();
    temp.copy_from("tests/", &["5_part_readme.csv"]).unwrap();
    let input_file = temp.child("5_part_readme.csv");
    let restored_file = temp.child("README.org");
    // When I restore it as CSV
    let mut cmd = Command::cargo_bin("qrxfil").expect("Error find qrxfil command");
    let args = [
        "restore",
        "--csv",
        input_file.path().to_str().unwrap(),
        restored_file.path().to_str().unwrap(),
    ];
    // Then restore succeeds
    cmd.args(&args).assert().success();
    // And md5sum matches original
    let md5_out = Command::new("md5sum")
        .current_dir(temp.path())
        .args(&[restored_file.path().to_str().unwrap()])
        .output()
        .expect("Failed while running md5")
        .stdout;

    let md5_str: String = String::from_utf8(md5_out).unwrap();
    assert_eq!(
        md5_str.split("  ").collect::<Vec<&str>>()[0],
        "ed3ce8996583747ee27a7d626d0c8f63"
    );

    // clean up the temp folder
    temp.close().expect("Error deleting temporary folder");
}
