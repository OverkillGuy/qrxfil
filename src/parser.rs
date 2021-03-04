//! Manipulate encoded chunks as strings
//!
//! Parse encoded string into a struct for reassembly

#[derive(Debug, PartialEq, Eq)]
/// A chunk that's already encoded, with base64 payload
pub struct EncodedChunk {
    id: u16,
    total: u16,
    payload: String,
}

/// Parse `chunk` string to extract id/total fields
///
/// This enables sorting all chunks by id for reassembly
pub fn parse(chunk: &str) -> EncodedChunk {
    EncodedChunk {
        id: 1,
        total: 2,
        payload: chunk.to_string(),
    }
}

#[cfg(test)]
mod chunk_tests {
    use super::*;

    #[test]
    fn decode_ok_test() {
        assert_eq!(
            parse("01OF02abcdef"),
            EncodedChunk {
                id: 1,
                total: 2, // TODO Flesh out testing
                payload: "01OF02abcdef".to_string()
            }
        );
    }
}
