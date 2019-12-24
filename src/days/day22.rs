use crate::error::Error;
use once_cell::sync::Lazy;
use regex::Regex;

pub fn both_parts(input: &str) -> Result<(), Error> {

    let techniques = parse_input(input)?;

    {
        let mut loc = 2019;
        for t in &techniques {
            loc = t.apply_to_location(loc, 10007);
        }
        println!("Star 1: {:?}", loc);
    }

    Ok(())
}

fn parse_input(input: &str) -> Result<Vec<Technique>,Error> {
    input
        .trim()
        .lines()
        .map(Technique::from_str)
        .collect()
}

#[derive(Debug,Clone,Copy,PartialEq,Eq)]
enum Technique {
    DealIntoNewStack,
    Cut(i32),
    DealWithIncrement(u32)
}

impl Technique {
    fn from_str(line: &str) -> Result<Technique,Error> {
        static NUMBER: Lazy<Regex>
            = Lazy::new(|| Regex::new(r"-?[0-9]+").unwrap());
        let line = line.trim();
        if line == "deal into new stack" {
            Ok(Technique::DealIntoNewStack)
        } else if line.starts_with("deal with increment") {
            let n = NUMBER.find(line)
                .ok_or(err!("number not found in {}", line))?
                .as_str()
                .parse()?;
            Ok(Technique::DealWithIncrement(n))
        } else if line.starts_with("cut") {
            let n = NUMBER.find(line)
                .ok_or(err!("number not found in {}", line))?
                .as_str()
                .parse()?;
            Ok(Technique::Cut(n))
        } else {
            Err(err!("No matching technique for '{}'", line))
        }
    }
    fn apply_to_location(&self, location: usize, len: usize) -> usize {
        match *self {
            Technique::DealIntoNewStack => {
                len - 1 - location
            },
            Technique::Cut(n) => {
                let n = if n < 0 {
                    len - n.abs() as usize
                } else {
                    n as usize
                };
                if location < n {
                    len - (n - location)
                } else {
                    location - n
                }
            },
            Technique::DealWithIncrement(n) => {
                location * n as usize % len
            }
        }
    }
}
