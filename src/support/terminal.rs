use std::io::prelude::*;
use crate::error::Error;
use crate::support::intcode::{
    Intcode,
    Outcome
};

/// Wrap an Intcode interpreter with a Reader and Writer that can provide
/// and return ASCII to the interpreter (and otherwise hands back non-ASCII).
/// Can easily be hooked up to stdin/stdout.
pub struct Terminal<R,W> {
    intcode: Intcode,
    reader: R,
    writer: W
}

impl <R: Read, W: Write> Terminal<R,W> {

    pub fn from_str(input: &str, reader: R, writer: W) -> Result<Terminal<R,W>,Error> {
        let intcode = Intcode::from_str(input)?;
        Ok(Terminal::new(intcode, reader, writer))
    }

    pub fn new(intcode: Intcode, reader: R, writer: W) -> Terminal<R,W> {
        Terminal { intcode, reader, writer }
    }

    /// Runs the Intcode interpreter until either there is an issue with
    /// the Reader or Writer, a non ASCII value is handed back, or the
    /// program finishes.
    pub fn step(&mut self) -> Result<Option<i64>,Error> {
        loop {
            if let Some(outcome) = self.intcode.step()? {
                match outcome {
                    Outcome::Output(c) => {
                        if c >= 0 && c <= 127 {
                            // ASCII: push to the writer:
                            self.writer.write_all(&[c as u8][..])?;
                        } else {
                            // Non ASCII: output it:
                            return Ok(Some(c))
                        }
                    },
                    Outcome::NeedsInput(p) => {
                        // Pull ASCII from our reader:
                        let mut buf = [0;1];
                        self.reader.read_exact(&mut buf)?;
                        self.intcode.provide_input(p.value(buf[0] as i64))?;
                    }
                }
            } else {
                return Ok(None)
            }
        }
    }

}