use crate::error::Error;
use std::iter;
use rayon::prelude::*;

pub fn both_parts(input: &str) -> Result<(), Error> {
    let vals = parse_input(input);

    println!("Star 1: {}", stringify(&phases(vals.clone(), 100), 0, 8));

    let more_vals = vals.repeat(10_000);
    let skip: usize = stringify(&more_vals, 0, 7).parse().unwrap();
    println!("Star 2: {}", stringify(&phases(more_vals, 100), skip, 8));

    Ok(())
}

fn stringify(input: &[i8], offset: usize, limit: usize) -> String {
    input.iter().skip(offset).take(limit).map(|n| n.to_string()).collect()
}

fn phases(mut input: Vec<i8>, n: usize) -> Vec<i8> {
    for i in 0..n {
        input = phase(&input);
    }
    input
}

fn phase(input: &[i8]) -> Vec<i8> {
    (0..input.len()).into_par_iter().map(|n| {
        let out: i32 = repeating_indexes(n, input).map(|(arr,r)| {
            arr.into_iter().map(|&n| n as i32).sum::<i32>() * r
        }).sum();
        (out % 10).abs() as i8
    }).collect()
}

/// Hand back windows of values and n's that they need multiplying with.
/// Ignore ranges that need multiplying with 0 since they don't contribute.
/// Added complexity as we have to skip the first of the repeating pattern.
fn repeating_indexes(n: usize, input: &[i8]) -> impl Iterator<Item=(&[i8],i32)> {
    let first_chunk = iter::once(&input[0..n]).zip(iter::repeat(&0));
    let next_chunks = input[n..].chunks(n+1).zip([1,0,-1,0].into_iter().cycle());
    first_chunk.chain(next_chunks).filter_map(move |(chunk,&n)| {
        if n == 0 {
            None
        } else {
            Some((chunk, n))
        }
    })
}

fn parse_input(input: &str) -> Vec<i8> {
    input.trim().bytes().map(|b| (b - 48) as i8).collect()
}