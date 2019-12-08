use crate::error::Error;
use crate::support::intcode::{ Intcode, Outcome };

pub fn part1(input: &str) -> Result<(),Error> {
    let ops = parse_input(input)?;
    let intcode = Intcode::new(ops);
    println!("Star 1: {}", run_intcode_with_input(intcode, 1)?);
    Ok(())
}

pub fn part2(input: &str) -> Result<(),Error> {
    let ops = parse_input(input)?;
    let intcode = Intcode::new(ops);
    println!("Star 2: {}", run_intcode_with_input(intcode, 5)?);
    Ok(())
}

fn run_intcode_with_input(mut intcode: Intcode, input: i64) -> Result<i64,Error> {
    let mut output: i64 = 0;
    while let Some(outcome) = intcode.step()? {
        match outcome {
            Outcome::StepComplete => {
                /* Nothing to do */
            },
            Outcome::NeedsInput(inputter) => {
                inputter.provide(input);
            },
            Outcome::Output(value) => {
                output = value
            }
        }
    }
    Ok(output)
}

fn parse_input(input: &str) -> Result<Vec<i64>,Error> {
    let mut ns = vec![];
    for (idx,s) in input.split(",").enumerate() {
        let n = s
            .trim()
            .parse()
            .map_err(|_| err!("Cannot parse entry {} ('{}') into an integer", idx+1, s))?;
        ns.push(n)
    }
    Ok(ns)
}
