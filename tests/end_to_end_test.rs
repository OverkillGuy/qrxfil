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
use assert_fs::fixture::ChildPath;
use assert_fs::prelude::*;
use predicates::prelude::*;
use rand::Rng;
use std::fs;
use std::io::Write;
use std::path::Path;
use test_case::test_case;

fn random_file_at(file_path: &ChildPath, file_size_bytes: i32) {
    let mut rng = rand::thread_rng();
    let random_chars: Vec<u8> = (0..file_size_bytes)
        .map(|_| rng.gen_range(0..255))
        .collect();
    file_path
        .write_binary(&random_chars)
        .expect("Could not write random to file for test seeding");
}

fn qr_decode(file_path: &Path) -> String {
    let img = image::open(file_path).unwrap().to_luma8();
    // Prepare for detection
    let mut img = rqrr::PreparedImage::prepare(img);
    // Search for grids, without decoding
    let grids = img.detect_grids();
    assert_eq!(grids.len(), 1);
    // Decode the grid
    let (_meta, content) = grids[0].decode().unwrap();
    println!("{}", &content);
    content
}

fn decode_qr_folder_to_file(output_folder: &Path, decoded_filepath: &Path) {
    let output_files = std::fs::read_dir(output_folder)
        .expect("Could not list output directory")
        .map(Result::unwrap)
        .filter(|file| file.file_name().to_str().unwrap().ends_with("png"));

    let mut decoded_file = match fs::File::create(decoded_filepath) {
        Ok(f) => f,
        Err(err) => panic!("File error: {}", err),
    };

    for qr_file in output_files {
        let decoded_string = qr_decode(&qr_file.path());
        decoded_file
            .write_all(decoded_string.as_bytes())
            .expect("Error writing QR decode file");
    }
}

fn run_qrxfil_assert_success(input_file: &Path, output_folder: &Path) {
    let mut cmd = Command::cargo_bin("qrxfil").expect("Error find qrxfil command");
    let args = [
        "exfil",
        input_file.to_str().unwrap(),
        output_folder.to_str().unwrap(),
    ];

    cmd.args(&args).assert().success();
}

fn run_qrxfil_restore_assert_success(input_file: &Path, restored_file: &Path) {
    let mut cmd = Command::cargo_bin("qrxfil").expect("Error find qrxfil command");
    let args = [
        "restore",
        input_file.to_str().unwrap(),
        restored_file.to_str().unwrap(),
    ];
    cmd.args(&args).assert().success();
}

fn md5sum_two_files(file1: &Path, file2: &Path, curdir: &Path) -> (String, String) {
    // And decoded file matches md5 of original
    let md5_out = Command::new("md5sum")
        .current_dir(curdir)
        .args(&[file1.to_str().unwrap(), file2.to_str().unwrap()])
        .output()
        .expect("Failed while running md5")
        .stdout;

    let md5_str: String = String::from_utf8(md5_out).unwrap();

    let lines: Vec<&str> = md5_str.lines().collect();
    let md5_restored_split: Vec<&str> = lines[0].split("  ").collect();

    println!(
        "restored: '{}' for {}",
        &md5_restored_split[0], &md5_restored_split[1],
    );
    let md5_reference_split: Vec<&str> = lines[1].split("  ").collect();
    println!(
        "line2: '{}' for {}",
        &md5_reference_split[0], &md5_reference_split[1],
    );

    (
        String::from(md5_restored_split[0]),
        String::from(md5_reference_split[0]),
    )
}

// Scenario: exfil and restore an random file properly

#[test_case(200 ; "Single chunk encoding")]
#[test_case(4 * 1024 ; "Half dozen chunks")]
// #[test_case(1024 * 500; "force overhead chunks to trigger issue 3")]
fn file_to_qr_and_back(file_size_bytes: i32) {
    // Given a file with a few KB of random data
    let temp = assert_fs::TempDir::new().unwrap();
    let input_file = temp.child("to_send.bin");
    let output_folder = temp.child("output_qrs");

    // Fill input file with random data
    random_file_at(&input_file, file_size_bytes);

    // And qrxfil ran in exfil-mode with input filename + output folder
    run_qrxfil_assert_success(input_file.path(), output_folder.path());
    let decoded_filepath = temp.child("qr_decoded.txt");
    decode_qr_folder_to_file(output_folder.path(), decoded_filepath.path());
    // When running qrxfil in decode-mode
    let restored_file = temp.child("restored.bin");
    run_qrxfil_restore_assert_success(decoded_filepath.path(), restored_file.path());
    // Then a decoded file is created
    restored_file.assert(predicate::path::is_file());

    let (md5_restored, md5_reference) =
        md5sum_two_files(restored_file.path(), input_file.path(), temp.path());
    assert_eq!(
        md5_restored, md5_reference,
        "Restored md5sum didn't match reference file before exfil",
    );
    // clean up the temp folder
    temp.close().expect("Error deleting temporary folder");
}
