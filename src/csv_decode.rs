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

//! CSV decoder to read BinaryEye app's "CSV with semicolon delimiter"
//! export format. Inspired by
//! https://docs.rs/csv/1.1.6/csv/#example-with-serde

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Scan {
    _datetime: String,
    pub format: String,
    pub content: String,
    error_correction_level: String,
}
