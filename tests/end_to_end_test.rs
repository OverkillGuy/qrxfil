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
use assert_fs::{fixture::ChildPath, prelude::*};
use predicates::prelude::*;
use rand::{prelude::SliceRandom, Rng};
use std::{
    fs,
    io::Write,
    path::{Path, PathBuf},
};
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

fn read_folder_sorted(folder: &Path) -> Vec<PathBuf> {
    let mut output_files: Vec<PathBuf> = std::fs::read_dir(folder)
        .expect("Could not list output directory")
        .map(Result::unwrap)
        .filter(|file| file.file_name().to_str().unwrap().ends_with("png"))
        .map(|e| e.path())
        .collect();
    // read_dir does not guarantee ordering => explicit sort chunk files
    output_files.sort();
    output_files
}

fn decode_qr_folder_to_file(files_to_decode: Vec<PathBuf>, decoded_filepath: &Path) {
    let mut decoded_file = match fs::File::create(decoded_filepath) {
        Ok(f) => f,
        Err(err) => panic!("File error: {}", err),
    };

    for qr_file in files_to_decode {
        let decoded_string = qr_decode(&qr_file);
        decoded_file
            .write_all(decoded_string.as_bytes())
            .expect("Error writing QR decode file");
        decoded_file
            .write_all("\n".as_bytes())
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
    let files_to_decode = read_folder_sorted(output_folder.path());
    decode_qr_folder_to_file(files_to_decode, decoded_filepath.path());
    // When running qrxfil in decode-mode
    let restored_file = temp.child("restored.bin");
    run_qrxfil_restore_assert_success(decoded_filepath.path(), restored_file.path());
    // Then a decoded file is created
    restored_file.assert(predicate::path::is_file());

    let (md5_restored, md5_reference) =
        md5sum_two_files(restored_file.path(), input_file.path(), temp.path());
    // And the decoded file is identical to original
    assert_eq!(
        md5_restored, md5_reference,
        "Restored md5sum didn't match reference file before exfil",
    );
    // clean up the temp folder
    temp.close().expect("Error deleting temporary folder");
}

#[test]
// Scenario: Out of order chunk during decode still works
fn out_of_order_chunk_scanning() {
    // Given a file with a few KB of random data
    let temp = assert_fs::TempDir::new().unwrap();
    let input_file = temp.child("to_send.bin");
    let output_folder = temp.child("output_qrs");

    // Fill input file with random data
    random_file_at(&input_file, 4 * 1024);

    // And qrxfil ran in exfil-mode with input filename + output folder
    run_qrxfil_assert_success(input_file.path(), output_folder.path());
    let decoded_filepath = temp.child("qr_decoded.txt");
    // But the files were decoded out of order
    let mut files_to_decode = read_folder_sorted(output_folder.path());
    let mut rng = rand::thread_rng();
    files_to_decode.shuffle(&mut rng);

    decode_qr_folder_to_file(files_to_decode, decoded_filepath.path());
    // When running qrxfil in decode-mode
    let restored_file = temp.child("restored.bin");
    run_qrxfil_restore_assert_success(decoded_filepath.path(), restored_file.path());
    // Then a decoded file is created
    restored_file.assert(predicate::path::is_file());

    let (md5_restored, md5_reference) =
        md5sum_two_files(restored_file.path(), input_file.path(), temp.path());
    // And the decoded file is identical to original
    assert_eq!(
        md5_restored, md5_reference,
        "Restored md5sum didn't match reference file before exfil",
    );
    // clean up the temp folder
    temp.close().expect("Error deleting temporary folder");
}

#[test]
// Scenario: Restoring with a missing chunk errors out
fn missing_chunk_error() {
    // Given a file with a few KB of random data
    let temp = assert_fs::TempDir::new().unwrap();
    let input_file = temp.child("to_send.bin");
    let output_folder = temp.child("output_qrs");

    // Fill input file with random data
    random_file_at(&input_file, 4 * 1024);

    // And qrxfil ran in exfil-mode with input filename + output folder
    run_qrxfil_assert_success(input_file.path(), output_folder.path());

    let decoded_filepath = temp.child("qr_decoded.txt");
    let mut files_to_decode = read_folder_sorted(output_folder.path());
    // But missing the first chunk
    files_to_decode.remove(0);

    decode_qr_folder_to_file(files_to_decode, decoded_filepath.path());
    // When running qrxfil in decode-mode
    let restored_file = temp.child("restored.bin");

    let mut cmd = Command::cargo_bin("qrxfil").expect("Error find qrxfil command");
    let args = [
        "restore",
        decoded_filepath.path().to_str().unwrap(),
        restored_file.path().to_str().unwrap(),
    ];
    cmd.args(&args)
        .assert()
        .failure()
        .stderr(predicate::str::contains("missing chunks: [1]"));

    // clean up the temp folder
    temp.close().expect("Error deleting temporary folder");
}

// TODO trigger parser::RestoreError's other enum cases (TooManyChunks, ChunkDecodeError, TotalMismatch) as unittest

#[test]
// Scenario: Restoring with a duplicate chunk succeeds
fn duplicate_chunk_skips() {
    // Given a file with a few KB of random data
    let temp = assert_fs::TempDir::new().unwrap();
    let input_file = temp.child("to_send.bin");
    let output_folder = temp.child("output_qrs");

    // Fill input file with random data
    random_file_at(&input_file, 4 * 1024);

    // And qrxfil ran in exfil-mode with input filename + output folder
    run_qrxfil_assert_success(input_file.path(), output_folder.path());

    let decoded_filepath = temp.child("qr_decoded.txt");
    let mut files_to_decode = read_folder_sorted(output_folder.path());
    // But with duplicate chunk
    files_to_decode.push(files_to_decode[0].clone());

    decode_qr_folder_to_file(files_to_decode, decoded_filepath.path());
    // When running qrxfil in decode-mode
    let restored_file = temp.child("restored.bin");

    let mut cmd = Command::cargo_bin("qrxfil").expect("Error find qrxfil command");
    let args = [
        "restore",
        decoded_filepath.path().to_str().unwrap(),
        restored_file.path().to_str().unwrap(),
    ];

    // Then it completes sucessfully
    cmd.args(&args).assert().success();

    let (md5_restored, md5_reference) =
        md5sum_two_files(restored_file.path(), input_file.path(), temp.path());
    // And decoded file is identical to original
    assert_eq!(
        md5_restored, md5_reference,
        "Restored md5sum didn't match reference file before exfil",
    );
    // clean up the temp folder
    temp.close().expect("Error deleting temporary folder");
}

#[test]
// Scenario: Restoring with a corrupted duplicate chunk fails
fn corrupt_duplicate_chunk_fails() {
    // Given a file with a few KB of random data
    let temp = assert_fs::TempDir::new().unwrap();
    let input_file = temp.child("to_send.bin");
    let output_folder = temp.child("output_qrs");

    // Fill input file with random data
    random_file_at(&input_file, 4 * 1024);

    // And qrxfil ran in exfil-mode with input filename + output folder
    run_qrxfil_assert_success(input_file.path(), output_folder.path());

    let decoded_filepath = temp.child("qr_decoded.txt");
    let files_to_decode = read_folder_sorted(output_folder.path());

    // But with corrupted duplicate chunk
    let decoded_string = fs::read_to_string(decoded_filepath.path()).unwrap();

    let mut file = fs::OpenOptions::new()
        .write(true)
        .append(true)
        .open(decoded_filepath.path())
        .unwrap();

    if let Err(e) = writeln!(file, "{}", &decoded_string[..30]) {
        eprintln!("Couldn't write to file: {}", e);
    }

    decode_qr_folder_to_file(files_to_decode, decoded_filepath.path());
    // When running qrxfil in decode-mode
    let restored_file = temp.child("restored.bin");

    let mut cmd = Command::cargo_bin("qrxfil").expect("Error find qrxfil command");
    let args = [
        "restore",
        decoded_filepath.path().to_str().unwrap(),
        restored_file.path().to_str().unwrap(),
    ];

    // Then it completes sucessfully
    cmd.args(&args)
        .assert()
        .failure()
        .stderr(predicate::str::contains("corrupt duplicate chunks: [1]"));

    let (md5_restored, md5_reference) =
        md5sum_two_files(restored_file.path(), input_file.path(), temp.path());
    // And decoded file is identical to original
    assert_eq!(
        md5_restored, md5_reference,
        "Restored md5sum didn't match reference file before exfil",
    );
    // clean up the temp folder
    temp.close().expect("Error deleting temporary folder");
}
