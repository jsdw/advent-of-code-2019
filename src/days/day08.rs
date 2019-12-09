use crate::error::Error;

pub fn part1(input: &str, width: usize, height: usize) -> Result<(), Error> {
    let layers = parse_data(input, width, height)?;

    // Find layer with least 0's:
    let mut layer_with_least_zeros = 0;
    let mut zeros = std::usize::MAX;
    for (idx, layer) in layers.iter().enumerate() {
        let zs = count_digit(layer, 0);
        if zs < zeros {
            layer_with_least_zeros = idx;
            zeros = zs;
        }
    }

    // Multiply the 1's and 2's of this layer
    let layer = &layers[layer_with_least_zeros];
    let ones = count_digit(layer, 1);
    let twos = count_digit(layer, 2);
    println!("Star 1: {}", ones * twos);

    Ok(())
}

pub fn part2(input: &str, width: usize, height: usize) -> Result<(), Error> {
    let layers = parse_data(input, width, height)?;
    let size = width * height;

    // Merge layers (0 black, 1 white, 2 transparent):
    let mut output_layer = vec![2; size];
    for i in 0..size {
        let mut n = output_layer[i];
        for layer in &layers {
            if n == 2 {
                n = layer[i];
            }
        }
        output_layer[i] = n;
    }

    // Print output to console:
    println!("Star 2:");
    for row in output_layer.chunks(width) {
        let s: String = row.iter().map(|n| {
            match n {
                0 => '#',
                1 => '.',
                _ => ' '
            }
        }).collect();
        println!("{}", s);
    }

    Ok(())
}

fn count_digit(layer: &[u8], digit: u8) -> usize {
    layer.iter().filter(|&&n| n == digit).count()
}

fn parse_data(input: &str, width: usize, height: usize) -> Result<Vec<Vec<u8>>,Error> {
    let input = input.trim();
    let size = width * height;
    if input.len() % size != 0 {
        return Err(err!("Size of data is not an exact multiple of layer size"));
    }

    let mut layers = Vec::with_capacity(input.len() / size);
    let mut layer = Vec::with_capacity(size);
    for (idx,c) in input.chars().enumerate() {
        let n = c.to_digit(10).ok_or_else(|| err!("Character {} in input is not a number", idx+1))?;
        layer.push(n as u8);
        if layer.len() == size {
            layers.push(layer);
            layer = Vec::with_capacity(size);
        }
    }

    Ok(layers)
}