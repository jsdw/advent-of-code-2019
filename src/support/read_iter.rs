use std::io::{ Read, Result as IoResult };

/// Convert an iterator over u8's into a Reader
pub struct ReadIter<I> {
    iter: I
}

impl <I: Iterator<Item=u8>> ReadIter<I> {
    pub fn new(iter: I) -> ReadIter<I> {
        ReadIter { iter }
    }
}

impl <I: Iterator<Item=u8>> Read for ReadIter<I> {
    fn read(&mut self, buf: &mut [u8]) -> IoResult<usize> {
        for i in 0..buf.len() {
            if let Some(byte) = self.iter.next() {
                buf[0] = byte
            } else {
                return Ok(i)
            }
        }
        Ok(buf.len())
    }
}

pub trait IntoReadIter {
    type Reader;
    fn into_reader(self) -> ReadIter<Self::Reader>;
}

impl <I: Iterator<Item=u8>> IntoReadIter for I {
    type Reader = Self;
    fn into_reader(self) -> ReadIter<Self::Reader> {
        ReadIter::new(self)
    }
}