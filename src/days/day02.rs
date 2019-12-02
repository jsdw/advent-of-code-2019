use crate::error::Error;

pub fn part1(input: &str) -> Result<(),Error> {
    let mut ops = parse_input(input)?;
    ops[1] = 12;
    ops[2] = 2;
    println!("Star 1: {}", run_program(ops));
    Ok(())
}

pub fn part2(input: &str) -> Result<(),Error> {
    let ops = parse_input(input)?;
    let answer = run_programs(ops, 19690720)
        .map(|(a,b)| 100 * a + b)
        .map(|n| n.to_string())
        .unwrap_or("No answer found".to_owned());
    println!("Star 2: {}", answer);
    Ok(())
}

fn run_programs(ops: Vec<usize>, answer: usize) -> Option<(usize,usize)> {
    for a in 0..=99 {
        for b in 0..=99 {
            let mut ops = ops.clone();
            ops[1] = a;
            ops[2] = b;
            let result = run_program(ops);
            if result == answer {
                return Some((a,b))
            }
        }
    }
    None
}

fn run_program(mut ops: Vec<usize>) -> usize {
    let mut current_op = 0;
    loop {
        match ops.get(current_op) {
            Some(1) => {
                let a = ops[current_op+1];
                let b = ops[current_op+2];
                let c = ops[current_op+3];
                ops[c] = ops[a] + ops[b];
            },
            Some(2) => {
                let a = ops[current_op+1];
                let b = ops[current_op+2];
                let c = ops[current_op+3];
                ops[c] = ops[a] * ops[b];
            },
            _ => break
        }
        current_op += 4;
    }
    ops[0]
}

fn parse_input(input: &str) -> Result<Vec<usize>,Error> {
    let mut ns = vec![];
    for (idx,s) in input.split(",").enumerate() {
        let n = s
            .trim()
            .parse()
            .map_err(|e| err!("Cannot parse entry {} ('{}') into an integer", idx+1, s))?;
        ns.push(n)
    }
    Ok(ns)
}