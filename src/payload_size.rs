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

//! Calculate size of a message before sending it

/// Size of a header in bytes
/// Three digits twice (id / total) plus delimiter string "OF" e.g.
/// 013OF078
pub const HEADER_SIZE_BYTES: u64 = 8;

#[allow(dead_code)] // TODO No need for this func anymore?
/// How many chunks of `chunk_size_bytes` to send for a given payload of `payload_size_bytes`
/// Taking into account the overhead of HEADER_SIZE_BYTES per chunk
pub fn number_chunks_overhead(payload_size_bytes: u64, chunk_size_bytes: u16) -> u64 {
    let chunk_payload_size_bytes: u64 = (chunk_size_bytes as u64) - HEADER_SIZE_BYTES;
    ((payload_size_bytes as f64) / (chunk_payload_size_bytes as f64)).ceil() as u64
}

#[cfg(test)]
mod payload_size_tests {
    use super::*;

    #[test]
    fn test_no_overhead() {
        // Scenario: Single chunk without overhead
        assert_eq!(number_chunks_overhead(1024, 2048), 1);
        // Scenario: Three chunks without overhead
        assert_eq!(number_chunks_overhead(2 * 1024, 1000), 3);
    }
    #[test]
    fn test_exact_align_overhead() {
        // Scenario: 4 KB payload split over 1024 bytes content forces a fifth chunk of overhead
        assert_eq!(number_chunks_overhead(4 * 1024, 1024), 5);
        // Scenario: 120 bytes payload split over 30 bytes content forces a fifth chunk too
        assert_eq!(number_chunks_overhead(120, 30), 6);
    }

    #[test]
    fn test_large_payload_causes_overhead() {
        // Scenario: 500KB payload split over 1KB gives 4 overhead chunks
        // 500 chunks * 8 bytes overhead per chunk = 4000 bytes of overhead = 4 chunks
        assert_eq!(number_chunks_overhead(500 * 1024, 1024), 500 + 4);
    }
}
