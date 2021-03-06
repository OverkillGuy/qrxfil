//! Manipulate encoded chunks as strings
//!
//! Parse encoded string into a struct for reassembly

#[derive(Debug, PartialEq, Eq)]
/// A chunk that's already encoded, with base64 payload
pub struct EncodedChunk {
    pub id: u16,
    pub total: u16,
    pub payload: String,
}

#[derive(Debug, PartialEq, Eq)]
/// Result type for errors of Chunk parsing
pub enum ChunkParseError {
    IdMissing,
    TotalMissing,
    PayloadMissing,
    BadSeparator,
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
mod chunk_tests {
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
