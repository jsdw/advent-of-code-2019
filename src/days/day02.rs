use crate::error::Error;
use crate::support::intcode::{ Intcode, Outcome, parse_intcode_ops };

pub fn part1(input: &str) -> Result<(),Error> {
    let mut ops = parse_intcode_ops(input)?;
    ops[1] = 12;
    ops[2] = 2;
    println!("Star 1: {}", run_program(ops)?);
    Ok(())
}

pub fn part2(input: &str) -> Result<(),Error> {
    let ops = parse_intcode_ops(input)?;
    let answer = run_programs(ops, 19690720)
        .map(|(a,b)| 100 * a + b)
        .map(|n| n.to_string())?;
    println!("Star 2: {}", answer);
    Ok(())
}

fn run_programs(ops: Vec<i64>, answer: i64) -> Result<(i64,i64),Error> {
    for a in 0..=99 {
        for b in 0..=99 {
            let mut ops = ops.clone();
            ops[1] = a;
            ops[2] = b;
            let result = run_program(ops)?;
            if result == answer {
                return Ok((a,b))
            }
        }
    }
    Err(err!("No answer found"))
}

fn run_program(ops: Vec<i64>) -> Result<i64,Error> {

    let mut intcode = Intcode::new(ops);
    while let Some(outcome) = intcode.step()? {
        if let Outcome::StepComplete = outcome {
            /* Do nothing; just carry on! */
        } else {
            panic!("Unexpected input: cannot handle")
        }
    }
    Ok(intcode.ops()[0])
}
