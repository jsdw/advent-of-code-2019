use crate::error::Error;
use std::iter;

pub fn both_parts(input: &str) -> Result<(), Error> {
    let vals = parse_input(input);

    println!("Star 1: {}", stringify(&phases(vals.clone(), 100), 0, 8));

    println!("Star 2: {}", stringify(&phases(vals[0..10].repeat(10), 100), 0, 8));

    // let more_vals = vals.repeat(10_000);
    // let skip: usize = stringify(&more_vals, 0, 7).parse().unwrap();
    // println!("Star 2: {}", stringify(&phases(more_vals, 100), skip, 8));

    Ok(())
}

fn stringify(input: &[i32], offset: usize, limit: usize) -> String {
    input.iter().skip(offset).take(limit).map(|n| n.to_string()).collect()
}

fn phases(mut input: Vec<i32>, n: usize) -> Vec<i32> {
    for i in 0..n {
        // println!("{:?}", input[0..50].iter().map(|&n| if n == 0 { '#' } else { ' ' }).collect::<Vec<_>>() );
        input = phase(&input);
    }
    input
}

fn phase(input: &[i32]) -> Vec<i32> {
    (0..input.len()).map(|n| {
        let n: i32 = input.into_iter()
            .zip(repeating_pattern(n))
            .map(|(a,b)| a*b)
            .sum();
        (n % 10).abs()
    }).collect()
}

fn repeating_pattern(n: usize) -> impl Iterator<Item=i32> {
    iter::repeat(0).take(n+1)
        .chain(iter::repeat(1).take(n+1))
        .chain(iter::repeat(0).take(n+1))
        .chain(iter::repeat(-1).take(n+1))
        .cycle()
        .skip(1)
}

fn parse_input(input: &str) -> Vec<i32> {
    input.trim().bytes().map(|b| (b - 48) as i32).collect()
}