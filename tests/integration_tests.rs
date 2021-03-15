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

extern crate assert_cmd;
extern crate assert_fs;
extern crate predicates;
extern crate rand;

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

// Scenario: qrxfil restore is md5-identical
#[test]
fn qr_folder_restore_happy() {
    // Given a file with qrxfil encoded strings
    let temp = assert_fs::TempDir::new().unwrap();
    let encoded_filename = "encoded_data.txt";
    temp.copy_from("./tests/", &["*.txt"]).unwrap();
    let encoded_file = temp.child(encoded_filename);
    let decoded_file = temp.child("decode_output.txt");

    // When running qrxfil in decode-mode with input folder + restore file
    let mut cmd = Command::cargo_bin("qrxfil").expect("Error find qrxfil command");
    // Then exit code is zero for success
    let args = [
        "restore",
        encoded_file.path().to_str().unwrap(),
        decoded_file.path().to_str().unwrap(),
    ];
    // println!("{} {} {}", &args[0], &args[1], &args[2]);
    cmd.args(&args).assert().success();
    // Then a decoded file is created
    decoded_file.assert(predicate::path::is_file());
    // And decoded file matches md5 of original
    Command::new("md5sum")
        .current_dir(temp.path())
        .args(&["decode_output.txt", "reference_file.txt"])
        .assert()
        .stdout(predicate::eq(
            "379abac9ff01fe015da6d1fd033ae9f3  \
             decode_output.txt\n379abac9ff01fe015da6d1fd033ae9f3  reference_file.txt\n",
        ));
    // clean up the temp folder
    temp.close().expect("Error deleting temporary folder");
}
