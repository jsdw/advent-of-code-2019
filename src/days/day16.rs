use crate::error::Error;
use std::iter;
use std::ops::Range;
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
    for _ in 0..n {
        input = phase(&input);
    }
    input
}

fn phase(input: &[i8]) -> Vec<i8> {
    let rolling_sums: Vec<i32> = rolling_sum_iter(input).collect();
    let len = input.len();
    (0..input.len()).into_par_iter().map(move |n| {
        let mut sum = 0;
        // Add all +1s:
        repeating_plus_ranges(n, len).for_each(|r| { sum += rolling_sums[r.end] - rolling_sums[r.start]; });
        // Minus all -1s:
        repeating_minus_ranges(n, len).for_each(|r| { sum -= rolling_sums[r.end] - rolling_sums[r.start]; });
        (sum % 10).abs() as i8
    }).collect()
}

fn rolling_sum_iter(input: &[i8]) -> impl Iterator<Item=i32> + '_ {
    iter::once(0).chain(input.into_iter().scan(0, |state, &n| {
        *state = *state + n as i32;
        Some(*state)
    }))
}

fn repeating_plus_ranges(n: usize, len: usize) -> impl Iterator<Item=Indexes> {
    repeating_digit_ranges(n, 1, len)
}

fn repeating_minus_ranges(n: usize, len: usize) -> impl Iterator<Item=Indexes> {
    repeating_digit_ranges(n, 3, len)
}

fn repeating_digit_ranges(n: usize, offset: usize, len: usize) -> impl Iterator<Item=Indexes> {
    let n_plus_one = n + 1;
    let mut curr = offset * n_plus_one - 1;
    iter::from_fn(move || {
        let last = (curr + n_plus_one).min(len);
        if last <= curr { return None }
        let out =  Range { start: curr, end: last };
        curr = curr + n_plus_one * 4;
        Some(out)
    })
}

fn parse_input(input: &str) -> Vec<i8> {
    input.trim().bytes().map(|b| (b - 48) as i8).collect()
}

type Indexes = Range<usize>;