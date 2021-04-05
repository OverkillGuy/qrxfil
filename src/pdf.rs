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

//! Create PDFs for printed paper backup

use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

use itertools::Itertools;

fn read_folder_sorted(folder: &Path) -> Vec<PathBuf> {
    std::fs::read_dir(folder)
        .expect("Could not list output directory")
        .map(Result::unwrap)
        .map(|file| file.path())
        .filter(|path| path.extension() == Some("png".as_ref()))
        // read_dir does not guarantee ordering => explicit sort chunk files
        .sorted()
        .collect()
}

pub fn genpandoc(output_folder: &Path) {
    let chunk_image_filenames = read_folder_sorted(output_folder);
    let chunks_total = chunk_image_filenames.len();

    let markdown_path = output_folder.join("input.md");
    let mut markdown_file = File::create(markdown_path.clone()).unwrap();

    write!(
        markdown_file,
        "% qrxfil export ({} chunks)
% Visit [Github.com/OverkillGuy/qrxfil](https://github.com/OverkillGuy/qrxfil) \
         for details

",
        chunks_total,
    )
    .unwrap();

    for (chunk_id_minus1, chunk_image) in chunk_image_filenames.iter().enumerate() {
        let chunk_id = chunk_id_minus1 + 1;
        write!(
            markdown_file,
            "

\\newpage
![Chunk {:3} of {} ]({} \"Chunk {:3}/{}\")
",
            chunk_id,
            chunks_total,
            chunk_image.to_str().unwrap(),
            chunk_id,
            chunks_total,
        )
        .unwrap();
    }
    let mut pandoc = pandoc::new();
    pandoc.add_option(pandoc::PandocOption::Meta(
        "geometry".to_string(),
        Some("margin=2cm".to_string()),
    ));
    pandoc.add_input(&markdown_path);
    pandoc.set_output(pandoc::OutputKind::File(output_folder.join("output.pdf")));
    pandoc.execute().unwrap();
}
