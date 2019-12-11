use crate::error::Error;
use std::collections::{ HashMap, HashSet };

pub fn part1(input: &str) -> Result<(), Error> {
    let coords = parse_asteroid_coords(input);
    let (best_coords, best_n) = find_best_asteroid_for_station(&coords);
    println!("Star 1: {} ({},{})", best_n, best_coords.0, best_coords.1);
    Ok(())
}

pub fn part2(input: &str) -> Result<(), Error> {
    Ok(())
}

fn find_best_asteroid_for_station(coords: &HashSet<(i64,i64)>) -> ((i64,i64),usize) {
    let mut visible_counts: HashMap<(i64,i64),usize> = HashMap::new();
    for &(x,y) in coords {
        let mut visible = 0;
        for &(x2,y2) in coords {
            // ignore self.
            if x == x2 && y == y2 { continue }

            // is this blocked by some other coords?
            let is_blocked = get_coords_between((x,y), (x2,y2)).any(|(xf,yf)| coords.contains(&(xf,yf)));

            // If not, count it as visible.
            if !is_blocked { visible += 1 }
        }
        //println!("{},{}: {}", x, y, visible);
        visible_counts.insert((x,y), visible);
    }
    visible_counts.into_iter().max_by_key(|(_,n)| *n).unwrap()
}

fn get_coords_between((x1,y1): (i64,i64), (x2,y2): (i64,i64)) -> impl Iterator<Item=(i64,i64)> {
    // normalize to (0,0):
    let (x,y) = (x2-x1, y2-y1);
    // if x == 0, swap coords (remember to swap back):
    let (swapped, x, y) = if x == 0 { (true,y,x) } else { (false,x,y) };
    // get range of x coords inbetween 0 and x:
    let range = if x < 0 { x+1..0 } else { 1..x };
    // filter x's that don't lead to nice round y's and make coords from the rest:
    range
        .filter(move |xa| {
            (xa * y) % x == 0
        }).map(move |xa| {
            let out_x = xa;
            let out_y = xa * y / x;
            if swapped { (out_y,out_x) } else { (out_x,out_y) }
        }).map(move |(x,y)| {
            (x+x1, y+y1)
        })
}

fn parse_asteroid_coords(input: &str) -> HashSet<(i64,i64)> {
    let mut coords = HashSet::new();
    for (y,line) in input.trim().lines().enumerate() {
        for (x,c) in line.trim().chars().enumerate() {
            if c != '.' {
                coords.insert((x as i64, y as i64));
            }
        }
    }
    coords
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_get_coords_between () {
        let ns = vec![
            ([(0,0), (10,0)], vec![(1,0),(2,0),(3,0),(4,0),(5,0),(6,0),(7,0),(8,0),(9,0)]),
            ([(0,0), (-10,0)], vec![(-9,0),(-8,0),(-7,0),(-6,0),(-5,0),(-4,0),(-3,0),(-2,0),(-1,0)]),
            ([(1,1), (11,1)], vec![(2,1),(3,1),(4,1),(5,1),(6,1),(7,1),(8,1),(9,1),(10,1)]),
            ([(2,2), (5,5)], vec![(3,3),(4,4)]),
            ([(0,0), (3,9)], vec![(1,3),(2,6)]),
            ([(0,2), (3,11)], vec![(1,5),(2,8)]),
            ([(0,-2), (-3,-11)], vec![(-2,-8),(-1,-5)]),
            ([(0,0), (0,4)], vec![(0,1),(0,2),(0,3)]),
            ([(8,2), (8,6)], vec![(8,3),(8,4),(8,5)]),
        ];
        for ([a,b], expected) in ns {
            let actual: Vec<_> = get_coords_between(a,b).collect();
            assert_eq!(actual, expected, "failed on {:?} - {:?}", a,b);
        }
    }

    #[test]
    fn test_find_match() {
        let inputs = vec![
            ("
            .#..#
            .....
            #####
            ....#
            ...##
            ",
            (3,4), 8),
            ("
            ......#.#.
            #..#.#....
            ..#######.
            .#.#.###..
            .#..#.....
            ..#....#.#
            #..#....#.
            .##.#..###
            ##...#..#.
            .#....####
            ",
            (5,8), 33),
        ];

        for (input, pos, n) in inputs {
            let coords = parse_asteroid_coords(input);
            let (best_coords, best_n) = find_best_asteroid_for_station(&coords);
            assert_eq!(best_coords, pos);
            assert_eq!(best_n, n);
        }
    }

}