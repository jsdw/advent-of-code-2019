use crate::error::Error;
use crate::support::intcode::{ parse_intcode_ops, Intcode, Outcome };

pub fn part1(input: &str) -> Result<(), Error> {
    println!("Star 1: {}", run_with_input(input, 1)?);
    Ok(())
}

pub fn part2(input: &str) -> Result<(), Error> {
    println!("Star 2: {}", run_with_input(input, 2)?);
    Ok(())
}

pub fn run_with_input(ops: &str, input: i64) -> Result<i64, Error> {
    let ops = parse_intcode_ops(ops)?;
    let mut intcode = Intcode::new(ops);
    while let Some(outcome) = intcode.step()? {
        match outcome {
            Outcome::NeedsInput(provider) => {
                provider.provide(input);
            },
            Outcome::Output(val) => {
                return Ok(val)
            }
        }
    }
    Err(err!("Expected an output but program finished first"))
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn output_large_middle_number() {
        let input = "104,1125899906842624,99";
        let out = test_intcode(input).unwrap();
        assert_eq!(out.get(0), Some(&1125899906842624));
    }

    #[test]
    fn output_16digit_number() {
        let input = "1102,34915192,34915192,7,4,7,99,0";
        let out = test_intcode(input).unwrap();
        assert_eq!(out.get(0), Some(&1219070632396864));
    }

    #[test]
    fn output_itself() {
        let input = "109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99";
        let out = test_intcode(input).unwrap();
        let out_str = out.into_iter().map(|v| v.to_string()).collect::<Vec<_>>().join(",");
        assert_eq!(out_str, input);
    }

    fn test_intcode(input: &str) -> Result<Vec<i64>,Error> {
        let ops = parse_intcode_ops(input)?;
        let mut intcode = Intcode::new(ops);
        let mut out = vec![];
        while let Some(outcome) = intcode.step()? {
            if let Outcome::Output(val) = outcome { out.push(val); }
            else { panic!("Unexpected op"); }
        }
        Ok(out)
    }

}