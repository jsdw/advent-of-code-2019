use itertools::Itertools;
use crate::error::Error;
use crate::support::digits;

pub fn part1(low: usize, high: usize) -> Result<(),Error> {
    let valid_count = (low..=high).filter(|&n| part1_test(n)).count();
    println!("Star 1: {}", valid_count);
    Ok(())
}

pub fn part2(low: usize, high: usize) -> Result<(),Error> {
    let valid_count = (low..=high).filter(|&n| part2_test(n)).count();
    println!("Star 2: {}", valid_count);
    Ok(())
}

fn part1_test(n: usize) -> bool {
    let mut has_pair = false;
    for (a,b) in digits(n).tuple_windows() {
        // Numbers cannot increase:
        if b > a {
            return false
        }
        // Look for at least one pair of numbers:
        if a == b {
            has_pair = true;
        }
    }
    if !has_pair {
        return false
    }

    true
}

fn part2_test(n: usize) -> bool {
    let mut current_pair_size = 1;
    let mut has_pair = false;
    for (a,b) in digits(n).tuple_windows() {
        // Numbers cannot increase:
        if b > a {
            return false
        }
        // Look for at least one pair of numbers
        // that is not part of a larger sequence:
        if a == b {
            current_pair_size += 1;
        } else {
            if current_pair_size == 2 {
                has_pair = true
            }
            current_pair_size = 1;
        }
    }
    if current_pair_size == 2 {
        has_pair = true
    }

    if !has_pair {
        return false
    }
    true
}
