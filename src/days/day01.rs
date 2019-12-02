use crate::error::Error;

pub fn part1(input: &str) -> Result<(),Error> {
    let masses = parse_masses(input)?;
    let sum_fuel_reqs: i64 = masses.into_iter().map(fuel_req).sum();
    println!("Star 1: {}", sum_fuel_reqs);
    Ok(())
}

pub fn part2(input: &str) -> Result<(),Error> {
    let masses = parse_masses(input)?;
    let sum_fuel_reqs: i64 = masses.into_iter().map(recursive_fuel_req).sum();
    println!("Star 2: {}", sum_fuel_reqs);
    Ok(())
}

fn parse_masses(input: &str) -> Result<Vec<i64>,Error> {
    let mut masses = Vec::new();
    for (idx, line) in input.lines().enumerate() {
        let n = line
            .parse()
            .map_err(|e| format!("Error on line {} of input: {}", idx+1, e))?;
        masses.push(n);
    }
    Ok(masses)
}

fn fuel_req(mass: i64) -> i64 {
    let fuel = mass / 3 - 2;
    if fuel < 0 { 0 } else { fuel }
}

fn recursive_fuel_req(mass: i64) -> i64 {
    let extra_mass = fuel_req(mass);
    extra_mass + if extra_mass > 0 { recursive_fuel_req(extra_mass) } else { 0 }
}

