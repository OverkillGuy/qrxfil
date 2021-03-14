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

//! Manipulate encoded chunks as strings
//!
//! Parse encoded string into a struct for reassembly

use std::collections::HashSet;
// use std::iter::FromIterator;

#[derive(Debug, PartialEq, Eq, Clone)]
/// A chunk that's already encoded, with base64 payload
pub struct EncodedChunk {
    pub id: u16,
    pub total: u16,
    pub payload: String,
}

#[derive(Debug, PartialEq, Eq)]
/// Things that can go wrong when restoring a chunked file
pub enum RestoreError {
    /// Not enough chunks for the expected total
    MissingChunk {
        /// How many we thought we'd find
        expected_total: u16,
        /// The ones we don't have
        missing_chunk_ids: Vec<u16>,
    },
    /// Unexpectedly too many chunks ("52 of 51")
    TooManyChunks {
        /// How many we thought we'd find
        expected_total: u16,
        /// IDs of chunks we got beyond `expected_total`
        unexpected_chunk_ids: Vec<u16>,
    },
    /// A chunk's total doesn't match the expected total
    /// "Expected" total is set from first decoded chunks as reference
    TotalMismatch {
        /// The original chunk we used as reference for total
        reference_chunk: EncodedChunk,
        /// The chunk we found a different total than reference
        clashing_chunk: EncodedChunk,
    },
    /// A chunk could not be parsed
    ChunkDecodeError {
        /// What went wrong
        error: ChunkParseError,
        /// Chunk as string before parsing
        raw_chunk: String,
    },
}

#[derive(Debug, PartialEq, Eq)]
/// Result type for errors of Chunk parsing
pub enum ChunkParseError {
    IdMissing,
    TotalMissing,
    PayloadMissing,
    BadSeparator,
}

/// Check the given chunks contain all the pieces to restore
///
/// Ensures that all chunks between 1 and `total`] are found in `chunks`
pub fn check_chunk_range(chunks: &Vec<EncodedChunk>) -> Result<(), RestoreError> {
    let expected_total: u16 = chunks[0].total;
    let mut actual_chunk_ids = HashSet::<u16>::with_capacity(expected_total as usize);
    for chunk in chunks {
        if chunk.total != expected_total {
            return Err(RestoreError::TotalMismatch {
                reference_chunk: chunks[0].clone(),
                clashing_chunk: chunk.clone(),
            });
        }
        actual_chunk_ids.insert(chunk.id);
    }
    let expected_chunk_ids: HashSet<u16> = (1..expected_total + 1).collect();
    if actual_chunk_ids == expected_chunk_ids {
        return Ok(());
    }
    if actual_chunk_ids.is_subset(&expected_chunk_ids) {
        let missing_ids = expected_chunk_ids
            .difference(&actual_chunk_ids)
            .cloned()
            .collect::<Vec<u16>>();
        return Err(RestoreError::MissingChunk {
            expected_total: expected_total,
            missing_chunk_ids: missing_ids,
        });
    }
    if actual_chunk_ids.is_superset(&expected_chunk_ids) {
        let too_many_ids = actual_chunk_ids
            .difference(&expected_chunk_ids)
            .cloned()
            .collect::<Vec<u16>>();
        return Err(RestoreError::TooManyChunks {
            expected_total: expected_total,
            unexpected_chunk_ids: too_many_ids,
        });
    }
    Ok(())
}

/// Parse `chunk` string to extract id/total fields
///
/// This enables sorting all chunks by id for reassembly
/// Chunk format: 003OF008andthenbase64payloadhere
///
/// Ordered as:
/// - chunk ID: 3 chars forming a u16
/// - "OF": hardcoded string used as separator/magic string (offset 3)
/// - chunk total: 3 chars forming a u16 for how many chunks exist
/// - payload: base64 encoded string payload of the chunk

pub fn parse(chunk: &str) -> Result<EncodedChunk, ChunkParseError> {
    let chunk_id = chunk[..3]
        .parse::<u16>()
        .map_err(|_| ChunkParseError::IdMissing)?;
    if chunk[3..5] != *"OF" {
        return Err(ChunkParseError::BadSeparator);
    }

    let chunk_total = chunk[5..8]
        .parse::<u16>()
        .map_err(|_| ChunkParseError::TotalMissing)?;

    if chunk[8..].is_empty() {
        return Err(ChunkParseError::PayloadMissing);
    }

    Ok(EncodedChunk {
        id: chunk_id,
        total: chunk_total,
        payload: chunk[8..].to_string(),
    })
}

#[cfg(test)]
mod parse_tests {
    use super::*;

    #[test]
    fn decode_ok_test() {
        let expected = Ok::<EncodedChunk, ChunkParseError>(EncodedChunk {
            id: 1,
            total: 2,
            payload: "abcdef".to_string(),
        });
        assert_eq!(parse("001OF002abcdef"), expected);
    }

    #[test]
    fn decode_no_id_test() {
        let expected = Err(ChunkParseError::IdMissing);
        assert_eq!(parse("aaaOF002abcdef"), expected);
    }

    #[test]
    fn decode_no_total_test() {
        let expected = Err(ChunkParseError::TotalMissing);
        assert_eq!(parse("003OFBAAabcdef"), expected);
    }

    #[test]
    fn decode_no_payload_test() {
        let expected = Err(ChunkParseError::PayloadMissing);
        assert_eq!(parse("003OF008"), expected);
    }

    #[test]
    fn decode_bad_separator_test() {
        let expected = Err(ChunkParseError::BadSeparator);
        assert_eq!(parse("011BA002abcdef"), expected);
    }
}

#[cfg(test)]
mod range_tests {
    use super::*;

    #[test]
    fn range_ok_test() {
        let total_number_chunks: u16 = 37;
        let first_chunk = EncodedChunk {
            id: 1,
            total: total_number_chunks,
            payload: String::from("payload1"),
        };
        // Create many chunks with proper data
        let chunks: Vec<EncodedChunk> = (1..total_number_chunks + 1)
            .map(|i| EncodedChunk {
                id: i,
                payload: format!("payload{}", i),
                ..first_chunk
            })
            .collect();

        assert!(check_chunk_range(&chunks).is_ok());
    }

    #[test]
    fn range_missing_chunk_test() {
        let chunks: Vec<EncodedChunk> = vec![
            EncodedChunk {
                id: 1,
                total: 3,
                payload: String::from("payload1"),
            },
            EncodedChunk {
                id: 2,
                total: 3,
                payload: String::from("payload2"),
            }, // missing third chunk
        ];
        let range_check = check_chunk_range(&chunks);
        let error = Err(RestoreError::MissingChunk {
            expected_total: 3,
            missing_chunk_ids: vec![3],
        });
        assert_eq!(range_check, error);
    }

    #[test]
    fn range_too_many_chunks_test() {
        let chunks: Vec<EncodedChunk> = vec![
            EncodedChunk {
                id: 1,
                total: 1,
                payload: String::from("payload1"),
            },
            // "Chunk 2 of 1"
            EncodedChunk {
                id: 2,
                total: 1,
                payload: String::from("payload2"),
            },
        ];
        let range_check = check_chunk_range(&chunks);
        let error = Err(RestoreError::TooManyChunks {
            expected_total: 1,
            unexpected_chunk_ids: vec![2],
        });
        assert_eq!(range_check, error);
    }

    #[test]
    fn range_total_mismatch_test() {
        let reference = EncodedChunk {
            id: 1,
            total: 3,
            payload: String::from("payload1"),
        };
        let clashing = EncodedChunk {
            id: 2,
            total: 4,
            payload: String::from("payload2"),
        };
        let chunks: Vec<EncodedChunk> = vec![reference.clone(), clashing.clone()];
        let range_check = check_chunk_range(&chunks);
        let error = Err(RestoreError::TotalMismatch {
            reference_chunk: reference,
            clashing_chunk: clashing,
        });
        assert_eq!(range_check, error);
    }
}
