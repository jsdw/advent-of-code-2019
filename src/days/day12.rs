use crate::error::Error;
use once_cell::sync::Lazy;
use regex::Regex;
use num::integer::lcm;

pub fn both_parts(input: &str) -> Result<(), Error> {
    let moons = parse_moons(input);

    // step 1000 cycles and see what the total energy is
    let mut moons = moons.clone();
    for _ in 0..1000 {
        step_moons(&mut moons);
    }
    println!("Star 1: {}", calculate_energy(&moons));

    // each dimension is independent, and will repeat from 0, so
    // find the cycles taken for each and then find the lowest
    // common multiple to find out when they will all repeat together.
    let repeats: Vec<_> = [Dimension::X, Dimension::Y, Dimension::Z]
        .into_iter()
        .map(|&d| find_dimension_repeating(moons.clone(), d))
        .collect();
    println!("Star 2: {}", repeats.into_iter().fold(1,lcm));

    Ok(())
}

fn find_dimension_repeating(mut sim: Vec<Moon>, dimension: Dimension) -> usize {
    let dim_values = |m: &Moon| match dimension {
            Dimension::X => (m.position.x, m.velocity.x),
            Dimension::Y => (m.position.y, m.velocity.y),
            Dimension::Z => (m.position.z, m.velocity.z)
    };
    let orig: Vec<(i64,i64)> = sim.iter().map(dim_values).collect();
    let mut n = 0;
    loop {
        step_moons(&mut sim);
        n += 1;
        let pvs: Vec<_> = sim.iter().map(dim_values).collect();
        if pvs == orig { return n }
    }
}

fn calculate_energy(moons: &[Moon]) -> i64 {
    moons.iter().map(|moon| {
        let p = moon.position.x.abs() + moon.position.y.abs() + moon.position.z.abs();
        let k = moon.velocity.x.abs() + moon.velocity.y.abs() + moon.velocity.z.abs();
        p * k
    }).sum()
}

fn step_moons(moons: &mut Vec<Moon>) {
    for a in 0..moons.len() {
        for b in a+1..moons.len() {
            let (moons_a, moons_b) = moons.split_at_mut(b);
            let moon_a = &mut moons_a[a];
            let moon_b = &mut moons_b[0];
            apply_gravity(moon_a, moon_b);
        }
    }
    for moon in moons {
        moon.position.x += moon.velocity.x;
        moon.position.y += moon.velocity.y;
        moon.position.z += moon.velocity.z;
    }
}

fn apply_gravity(a: &mut Moon, b: &mut Moon) {
    apply_gravity_single(&a.position.x, &mut a.velocity.x, &b.position.x, &mut b.velocity.x);
    apply_gravity_single(&a.position.y, &mut a.velocity.y, &b.position.y, &mut b.velocity.y);
    apply_gravity_single(&a.position.z, &mut a.velocity.z, &b.position.z, &mut b.velocity.z);
}

fn apply_gravity_single(a_pos: &i64, a_vel: &mut i64, b_pos: &i64, b_vel: &mut i64) {
    if a_pos == b_pos { return }
    let (a_vel,b_vel) = if a_pos < b_pos { (a_vel,b_vel) } else { (b_vel,a_vel) };
    *a_vel += 1;
    *b_vel -= 1;
}

fn parse_moons(input: &str) -> Vec<Moon> {
    static RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"<x=(-?\d+),\s*y=(-?\d+),\s*z=(-?\d+)>").unwrap());
    let mut moons: Vec<Moon> = vec![];
    for cap in RE.captures_iter(input) {
        moons.push(Moon {
            position: Point {
                x: cap[1].parse().unwrap(),
                y: cap[2].parse().unwrap(),
                z: cap[3].parse().unwrap(),
            },
            velocity: Point {
                x: 0,
                y: 0,
                z: 0,
            }
        })
    }
    moons
}

#[derive(Debug,Clone,Copy,PartialEq,Eq,Hash)]
struct Moon {
    position: Point,
    velocity: Point
}

#[derive(Debug,Clone,Copy,PartialEq,Eq,Hash)]
struct Point {
    x: i64, y: i64, z: i64
}

#[derive(Debug,Clone,Copy,PartialEq,Eq,Hash)]
enum Dimension {
    X, Y, Z
}