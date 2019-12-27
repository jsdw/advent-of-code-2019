use crate::error::Error;
use once_cell::sync::Lazy;
use regex::Regex;
use num::{ BigInt, FromPrimitive, ToPrimitive };

pub fn both_parts(input: &str) -> Result<(), Error> {

    let techniques = parse_input(input)?;

    // Part 1: transform the location by applying the shuffle techniques
    // one after the other to it. I'll need to reverse this for part 2 so
    // I'm factoring out the `%` step and using bg numbers to represent:
    {
        let mut loc: BigInt = 2019.into();
        let len: BigInt = 10007.into();
        for t in &techniques {
            loc = t.apply_to_location_no_mod(&loc, &len);
        }
        println!("Star 1: {}", positive_mod(&loc,&len).to_usize().unwrap());
    }

    // Part 2: we need to implement `unapply_location_no_mod` in such a way
    // that we can avoid iterating it many times. Modular division may be key.

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
    Cut(i128),
    DealWithIncrement(i128)
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
    fn apply_to_location_no_mod(&self, location: &BigInt, _len: &BigInt) -> BigInt {
        match *self {
            Technique::DealIntoNewStack => {
                // We want to reverse the order. Assume we'll apply `% len`
                // later and come up with a formula that works anyway. A little
                // experimentation reveals that this works:
                -1 - location
            },
            Technique::Cut(n) => {
                // Here we are basically rotating by n, so we just add or minus it.
                // When we apply mod later that will get us back to locations in
                // the range we care about:
                location - n
            },
            Technique::DealWithIncrement(n) => {
                // Here we are taking each location and multiplying it. We'll use mod
                // to perfectly map these onto unique locations (`n` is constrained
                // to make this be the case).
                location * n
            }
        }
    }
    #[allow(unused)]
    fn unapply_to_location_no_mod(&self, location: &BigInt, len: &BigInt) -> BigInt {
        match *self {
            Technique::DealIntoNewStack => {
                // Reverse again to undo the reverse
                -1 - location
            },
            Technique::Cut(n) => {
                // Add rather than minus to undo cut
                location + n
            },
            Technique::DealWithIncrement(n) => {
                // This is not as obvious. We need to invert a thing we multiplied
                // by n, but we can't just divide. This is the naive approach for
                // undoing the multiply + modulo:
                let mut a = location.clone();
                while &a % n != 0.into() {
                    a += len;
                }
                a / n
            }
        }
    }
}

/// If the number is positive this is the same as `%`. If the number is negative
/// that won't get the number back into the range we want (it'll still be negative)
/// so we handle that here too.
fn positive_mod(n: &BigInt, len: &BigInt) -> BigInt {
    if n >= &BigInt::from_usize(0).unwrap() {
        n % len
    } else {
        (len + (n % len)) % len
    }
}

/// Make sure we're on the right track..
#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_part1s() {
        let in_outs = vec![
            (
            "
            deal with increment 7
            ",
            vec![0,3,6,9,2,5,8,1,4,7]
            ),
            (
            "
            deal with increment 7
            deal into new stack
            deal into new stack
            ",
            vec![0,3,6,9,2,5,8,1,4,7]
            ),
            (
            "
            deal into new stack
            ",
            vec![9,8,7,6,5,4,3,2,1,0]
            ),
            (
            "
            deal into new stack
            deal into new stack
            ",
            vec![0,1,2,3,4,5,6,7,8,9]
            ),
            (
            "
            cut 3
            ",
            vec![3,4,5,6,7,8,9,0,1,2]
            ),
            (
            "
            cut -3
            ",
            vec![7,8,9,0,1,2,3,4,5,6]
            ),
            (
            "
            cut 0
            deal with increment 7
            ",
            vec![0,3,6,9,2,5,8,1,4,7]
            ),
            (
            "
            cut 6
            deal with increment 7
            deal into new stack
            ",
            vec![3,0,7,4,1,8,5,2,9,6]
            ),
            (
            "
            deal into new stack
            cut -2
            deal with increment 7
            cut 8
            cut -4
            deal with increment 7
            cut 3
            deal with increment 9
            deal with increment 3
            cut -1
            ",
            vec![9,2,5,8,1,4,7,0,3,6]
            ),
        ];

        for (idx,(input,expected)) in in_outs.into_iter().enumerate() {
            let techniques = parse_input(input).unwrap();
            let inputs: Vec<_> = (0..expected.len()).collect();
            let actual = apply_techniques_to_vec(&inputs, &techniques);
            assert_eq!(actual, expected, "(index: {})", idx);
        }
    }

    #[test]
    fn test_unapply_to_location_no_mod() {
        let inputs = vec![
            "deal into new stack",
            "cut 3",
            "cut -3",
            "deal with increment 7",
            "deal with increment 3",
            "deal with increment 9",
            "
            deal into new stack
            deal into new stack
            ",
            "
            deal with increment 7
            deal into new stack
            deal into new stack
            ",
        ];

        for (idx,input) in inputs.into_iter().enumerate() {
            let techniques = parse_input(input).unwrap();
            let cards: Vec<_> = (0..10).collect();

            let applied = apply_techniques_to_vec(&cards, &techniques);
            println!("After applying forwards: {:?}", applied);
            let unapplied = unapply_techniques_to_vec(&applied, &techniques);

            assert_eq!(cards, unapplied, "(index: {})", idx);
        }
    }

    fn apply_techniques_to_vec(
        input: &[usize],
        techniques: &[Technique],
    ) -> Vec<usize> {
        let len_u = input.len();
        let len: BigInt = len_u.into();
        let mut actual = vec![0; len_u];
        (0..len_u)
            .map(|n| n.into())
            .map(|n: BigInt| techniques.iter().fold(n, |n,t| t.apply_to_location_no_mod(&n,&len)))
            .map(|l| positive_mod(&l, &len))
            .enumerate()
            .for_each(|(n, l)| actual[l.to_usize().unwrap()] = input[n]);
        actual
    }

    fn unapply_techniques_to_vec(
        input: &[usize],
        techniques: &[Technique],
    ) -> Vec<usize> {
        let len_u = input.len();
        let len: BigInt = len_u.into();
        let mut actual = vec![0; len_u];
        (0..len_u)
            .map(|n| n.into())
            .map(|n: BigInt| techniques.iter().rev().fold(n, |n,t| t.unapply_to_location_no_mod(&n,&len)))
            .map(|l| positive_mod(&l, &len))
            .enumerate()
            .for_each(|(n, l)| actual[l.to_usize().unwrap()] = input[n]);
        actual
    }

}