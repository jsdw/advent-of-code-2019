use std::io::Read;

/// convert to ASCII uppercase any bytes read from the
/// provided Reader before handing them on.
pub struct Uppercase<R> {
    reader: R
}

impl <R: Read> Uppercase<R> {
    pub fn new(reader: R) -> Uppercase<R> {
        Uppercase { reader }
    }
}

impl <R: Read> Read for Uppercase<R> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let n = self.reader.read(buf)?;
        for b in &mut buf[0..n] {
            b.make_ascii_uppercase();
        }
        Ok(n)
    }
}