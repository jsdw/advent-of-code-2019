use crate::error::Error;

pub fn part1(input: &str, width: usize, height: usize) -> Result<(), Error> {
    // Find layer with least 0's:
    let ls = parse_data(input, width, height);
    let l = ls
        .iter()
        .min_by_key(|l| count_digit(l,0))
        .unwrap();

    println!("Star 1: {}", count_digit(l, 1) * count_digit(l, 2));
    Ok(())
}

pub fn part2(input: &str, width: usize, height: usize) -> Result<(), Error> {
    let layers = parse_data(input, width, height);
    let size = width * height;

    // Merge layers (0 black, 1 white, 2 transparent):
    let l: Vec<u8> = (0..size)
        .map(|i| layers.iter().map(|l| l[i]).filter(|&n| n != 2).next().unwrap())
        .collect();

    // Print output to console:
    println!("Star 2:");
    for row in l.chunks_exact(width) {
        row.iter().for_each(|&n| print!("{}", if n == 1 { '■' } else { ' '}));
        println!();
    }

    Ok(())
}

fn count_digit(layer: &[u8], digit: u8) -> usize {
    layer.iter().filter(|&&n| n == digit).count()
}

fn parse_data(input: &str, width: usize, height: usize) -> Vec<Vec<u8>> {
    input.trim()
         .as_bytes()
         .chunks_exact(width*height)
         .map(|c| c.into_iter().map(|b| b-48).collect())
         .collect()
}