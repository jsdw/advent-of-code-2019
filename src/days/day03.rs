use crate::error::Error;
use std::collections::HashMap;
use self::Direction::*;

pub fn part1(input: &str) -> Result<(),Error> {
    let all_paths = parse_wires(input)?;

    // Draw wires on grid:
    let mut seen = HashMap::new();
    for (id,paths) in all_paths.iter().enumerate() {
        let mut x: i32 = 0;
        let mut y: i32 = 0;
        for path in paths {
            for _ in 0..path.count {
                match path.direction {
                    Up => { y -= 1 },
                    Down => { y += 1 },
                    Left => { x -= 1 },
                    Right => { x += 1 }
                }
                let v = seen.entry((x,y)).or_insert(Vec::new());
                v.push(id);
                v.sort();
                v.dedup();
            }
        }
    }

    // Find closest intersection:
    let mut dist: i32 = std::i32::MAX;
    for ((x,y), values) in seen {
        let this_dist = x.abs() + y.abs();
        if values.len() > 1 && this_dist < dist {
            dist = this_dist
        }
    }

    println!("Star 1: {}", dist);
    Ok(())
}

pub fn part2(input: &str) -> Result<(),Error> {
    let all_paths = parse_wires(input)?;

    // Draw wires on grid, tracking current distance travelled
    // by each wire as we go (keeping smallest only):
    let mut seen = HashMap::new();
    for (id,paths) in all_paths.iter().enumerate() {
        let mut x: i32 = 0;
        let mut y: i32 = 0;
        let mut d: usize = 0;
        for path in paths {
            for _ in 0..path.count {
                d += 1;
                match path.direction {
                    Up => { y -= 1 },
                    Down => { y += 1 },
                    Left => { x -= 1 },
                    Right => { x += 1 }
                }
                let m = seen.entry((x,y)).or_insert(HashMap::new());
                m.entry(id).or_insert(d);
            }
        }
    }

    // What's the smallest total distance travelled when intersection:
    let min_intersection_d = seen
        .into_iter()
        .filter(|(_,v)| v.len() > 1)
        .map(|(_,v)| v.values().sum::<usize>())
        .min()
        .map(|v| v.to_string())
        .unwrap_or("Unknown".to_owned());
    println!("Star 2: {}", min_intersection_d);

    Ok(())
}

fn parse_wires(input: &str) -> Result<Vec<Vec<Path>>,Error> {
    let mut all_paths = vec![];
    for line in input.trim().lines() {
        let mut paths = vec![];
        for path in line.trim().split(",") {
            let (direction, count) = path.split_at(1);
            let count = count.parse()?;
            let direction = match direction {
                "U" => Ok(Up),
                "D" => Ok(Down),
                "L" => Ok(Left),
                "R" => Ok(Right),
                _ => Err(err!("Invalid direction"))
            }?;
            paths.push(Path {
                direction,
                count
            })
        }
        all_paths.push(paths);
    }
    Ok(all_paths)
}

#[derive(Clone,Copy,Debug)]
struct Path {
    direction: Direction,
    count: usize
}

#[derive(Clone,Copy,Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}
