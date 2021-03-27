use std::io;
use std::io::{BufRead, Read};

#[allow(dead_code)] // Temporary while no consumer of this API
/// An iterator for reading `chunk_size` bytes off the given `reader`
pub struct BufferedIterator<T>
where
    T: Read,
{
    reader: T,
    chunk_size: u64,
}

impl<T> BufferedIterator<T>
where
    T: Read,
{
    #[allow(dead_code)] // Temporary while no consumer of this API
    /// Get a new iterator ready to read `chunk_size` bytes from `reader`
    pub fn new(reader: T, chunk_size: u64) -> Self {
        Self { reader, chunk_size }
    }
}

impl<T> Iterator for BufferedIterator<T>
where
    T: BufRead,
{
    type Item = io::Result<String>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut buf = String::new();
        match self
            .reader
            .by_ref()
            .take(self.chunk_size)
            .read_to_string(&mut buf)
        {
            Ok(0) => None,
            Ok(_n) => Some(Ok(buf)),
            Err(e) => Some(Err(e)),
        }
    }
}

/// An iterator for generating chunk strings
/// Taking total payload size and keeping count of chunk ID/total
pub struct ChunkIterator<T>
where
    T: BufRead,
{
    reader: BufferedIterator<T>,
    /// How big is the content that can be read
    _total_size: u64, // TODO use this to calculate chunk total!
    /// How many chunks are there in total
    chunk_total: u16,
    /// Chunk counter incremented on each next()
    current_chunk_id: u16,
}

impl<T> ChunkIterator<T>
where
    T: BufRead,
{
    #[allow(dead_code)] // Temporary while no consumer of this API
    /// Get a new iterator ready to read `chunk_size` bytes from `reader`, representing
    pub fn new(reader: BufferedIterator<T>, total_size: u64) -> Self {
        Self {
            reader,
            _total_size: total_size,
            chunk_total: 10,
            current_chunk_id: 1,
        }
    }
}

impl<T> Iterator for ChunkIterator<T>
where
    T: BufRead,
{
    type Item = io::Result<String>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.reader.next() {
            None => None,
            Some(Ok(buf)) => {
                let chunk_string = format!(
                    "{:03}OF{:03}{}",
                    self.current_chunk_id, self.chunk_total, buf
                );
                self.current_chunk_id += 1;
                Some(Ok(chunk_string))
            }
            Some(Err(e)) => Some(Err(e)),
        }
    }
}

#[cfg(test)]
mod iterator_tests {
    use super::*;

    #[test]
    fn read_ok_twice_noleftover_test() {
        // Given a 10 byte payload
        let payload: String = String::from("foobarbaz!");
        let cursor = io::Cursor::new(payload.clone());
        // And an iterator taking 5 bytes
        let mut iter = BufferedIterator::new(cursor, 5);

        // When I call iterator.next() three times
        let res = match iter.next() {
            None => panic!("iterator returned no data first time"),
            Some(Err(e)) => panic!(e),
            Some(Ok(buf)) => buf,
        };
        // Then the first two yield Some payload
        assert_eq!(res, payload[..5]);
        let res2 = match iter.next() {
            None => panic!("iterator returned no data"),
            Some(Err(e)) => panic!(e),
            Some(Ok(buf)) => buf,
        };
        assert_eq!(res2, payload[5..]);
        // But the third returns None
        let res3 = iter.next();
        assert!(res3.is_none());
    }

    #[test]
    fn read_ok_twice_leftover_test() {
        // Given a 11 byte payload
        let payload: String = String::from("foobarmore!");
        let cursor = io::Cursor::new(payload.clone());
        // And an iterator taking 6 bytes
        let mut iter = BufferedIterator::new(cursor, 6);

        // When I call iterator.next() thrice
        let res = match iter.next() {
            None => panic!("iterator returned no data first time"),
            Some(Err(e)) => panic!(e),
            Some(Ok(buf)) => buf,
        };
        // Then the first yields Some payload
        assert_eq!(res, payload[..6]);
        let res2 = match iter.next() {
            None => panic!("iterator returned no data"),
            Some(Err(e)) => panic!(e),
            Some(Ok(buf)) => buf,
        };
        // And the second returns leftover payload
        assert_eq!(res2, payload[6..]);
        // But the third returns None
        let res3 = iter.next();
        assert!(res3.is_none());
    }
}

#[cfg(test)]
mod chunk_iterator_tests {
    use super::*;
    use std::convert::TryInto;

    #[test]
    // Scenario: Reading 2 chunks with no leftover data
    fn read_ok_twice_noleftover_test() {
        // Given a 10 byte payload
        let payload = String::from("foobarbaz!");
        let cursor = io::Cursor::new(payload.clone());
        // And an iterator taking 5 bytes
        let str_iter = BufferedIterator::new(cursor, 5);
        let mut chunk_iter = ChunkIterator::new(str_iter, payload.len().try_into().unwrap());

        // When I call iterator.next() three times
        let res = match chunk_iter.next() {
            None => panic!("iterator returned no data first time"),
            Some(Err(e)) => panic!(e),
            Some(Ok(buf)) => buf,
        };
        // Then the first two yield Some payload
        assert_eq!(res, ["001OF010", &payload[..5]].concat());
        let res2 = match chunk_iter.next() {
            None => panic!("iterator returned no data"),
            Some(Err(e)) => panic!(e),
            Some(Ok(buf)) => buf,
        };
        assert_eq!(res2, ["002OF010", &payload[5..10]].concat());
        // But the third returns None for empty iterator
        let res3 = chunk_iter.next();
        assert!(res3.is_none());
    }

    #[test]
    // Scenario: Reading a chunk with leftover data on second read
    fn read_ok_twice_leftover_test() {
        // Given a 11 byte payload
        let payload = String::from("foobarmore!");
        let cursor = io::Cursor::new(payload.clone());
        // And an iterator taking 6 bytes
        let str_iter = BufferedIterator::new(cursor, 6);
        let mut chunk_iter = ChunkIterator::new(str_iter, payload.len().try_into().unwrap());

        // When I call iterator.next() three times
        let res = match chunk_iter.next() {
            None => panic!("iterator returned no data first time"),
            Some(Err(e)) => panic!(e),
            Some(Ok(buf)) => buf,
        };
        // Then the first yields Some payload
        assert_eq!(res, ["001OF010", &payload[..6]].concat());
        let res2 = match chunk_iter.next() {
            None => panic!("iterator returned no data"),
            Some(Err(e)) => panic!(e),
            Some(Ok(buf)) => buf,
        };
        // And the second returns the leftover payload
        assert_eq!(res2, ["002OF010", &payload[6..]].concat());
        // But the third returns None
        let res3 = chunk_iter.next();
        assert!(res3.is_none());
    }
}
