use std::io::{BufRead, Read};

use std::io;

pub struct ChunkIterator<T>
where
    T: Read,
{
    reader: T,
    chunk_size: u64,
}

impl<T> ChunkIterator<T>
where
    T: Read,
{
    pub fn new(reader: T, chunk_size: u64) -> Self {
        Self { reader, chunk_size }
    }
}

impl<T> Iterator for ChunkIterator<T>
where
    T: BufRead,
{
    type Item = io::Result<Vec<u8>>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut buf = Vec::default();
        match self
            .reader
            .by_ref()
            .take(self.chunk_size)
            .read_to_end(&mut buf)
        {
            Ok(0) => None,
            Ok(_n) => Some(Ok(buf)),
            Err(e) => Some(Err(e)),
        }
    }
}
