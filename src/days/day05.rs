use crate::error::Error;
use crate::support::intcode::{ Intcode, Outcome, parse_intcode_ops };

pub fn part1(input: &str) -> Result<(),Error> {
    let ops = parse_intcode_ops(input)?;
    let intcode = Intcode::new(ops);
    println!("Star 1: {}", run_intcode_with_input(intcode, 1)?);
    Ok(())
}

pub fn part2(input: &str) -> Result<(),Error> {
    let ops = parse_intcode_ops(input)?;
    let intcode = Intcode::new(ops);
    println!("Star 2: {}", run_intcode_with_input(intcode, 5)?);
    Ok(())
}

fn run_intcode_with_input(mut intcode: Intcode, input: i64) -> Result<i64,Error> {
    let mut output: i64 = 0;
    while let Some(outcome) = intcode.step()? {
        match outcome {
            Outcome::NeedsInput(provider) => {
                intcode.provide_input(provider.value(input))?;
            },
            Outcome::Output(value) => {
                output = value
            }
        }
    }
    Ok(output)
}
