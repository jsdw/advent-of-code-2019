use crate::error::Error;
use std::collections::{ HashMap, HashSet };
use std::f64::consts::{ FRAC_PI_2, PI };

pub fn both_parts(input: &str) -> Result<(), Error> {
    let coords = parse_asteroid_coords(input);

    // Where do we want our station?
    let (best_coords, best_n) = find_best_asteroid_for_station(&coords);
    println!("Star 1: {} ({},{})", best_n, best_coords.0, best_coords.1);

    // Now, which coords are hit first by a laser?
    let visible_from_best: Vec<_> = coords_encountered_by_laser(best_coords, &coords);
    let t = visible_from_best[199];
    println!("Star 2: {}", t.0 * 100 + t.1);

    Ok(())
}

fn coords_encountered_by_laser((x,y): (i64,i64), coords: &HashSet<(i64,i64)>) -> Vec<(i64,i64)> {
    let mut visible: Vec<_> = find_visible_asteroids_for((x,y), &coords).collect();
    // sort by what will be hit first.
    visible.sort_by(|&(x1,y1),&(x2,y2)| {
        let (x1,y1) = (x1 - x, y1 - y);
        let (x2,y2) = (x2 - x, y2 - y);
        let a1 = angleish(x1,y1);
        let a2 = angleish(x2,y2);
        a1.partial_cmp(&a2).unwrap()
    });
    visible
}

fn angleish(x: i64, y: i64) -> f64 {
    let atan = f64::atan2(y as f64,x as f64);
    if atan < -FRAC_PI_2 {
        atan + PI + PI
    } else {
        atan
    }
}

fn find_best_asteroid_for_station(coords: &HashSet<(i64,i64)>) -> ((i64,i64),usize) {
    let mut visible_counts: HashMap<(i64,i64),usize> = HashMap::new();
    for &xy in coords {
        let visible = find_visible_asteroids_for(xy, coords).count();
        visible_counts.insert(xy, visible);
    }
    visible_counts.into_iter().max_by_key(|(_,n)| *n).unwrap()
}

fn find_visible_asteroids_for<'a>((x,y): (i64,i64), coords: &'a HashSet<(i64,i64)>) -> impl Iterator<Item=(i64,i64)> + 'a {
    coords.iter().filter_map(move |&(x2,y2)| {
        // ignore self.
        if x == x2 && y == y2 { return None }
        // is this blocked by some other coords?
        let is_blocked = get_coords_between((x,y), (x2,y2)).any(|(xf,yf)| coords.contains(&(xf,yf)));
        // If not, count it as visible.
        if is_blocked { None } else { Some((x2,y2)) }
    })
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
            ("
            #.#...#.#.
            .###....#.
            .#....#...
            ##.#.#.#.#
            ....#.#.#.
            .##..###.#
            ..#...##..
            ..##....##
            ......#...
            .####.###.
            ",
            (1,2), 35),
            ("
            .#..#..###
            ####.###.#
            ....###.#.
            ..###.##.#
            ##.##.#.#.
            ....###..#
            ..#.#..#.#
            #..#.#.###
            .##...##.#
            .....#.#..
            ",
            (6,3), 41),
            ("
            .#..##.###...#######
            ##.############..##.
            .#.######.########.#
            .###.#######.####.#.
            #####.##.#.##.###.##
            ..#####..#.#########
            ####################
            #.####....###.#.#.##
            ##.#################
            #####.##.###..####..
            ..######..##.#######
            ####.##.####...##..#
            .#####..#.######.###
            ##...#.##########...
            #.##########.#######
            .####.#.###.###.#.##
            ....##.##.###..#####
            .#.#.###########.###
            #.#.#.#####.####.###
            ###.##.####.##.#..##
            ",
            (11,13), 210)
        ];

        for (input, pos, n) in inputs {
            let coords = parse_asteroid_coords(input);
            let (best_coords, best_n) = find_best_asteroid_for_station(&coords);
            assert_eq!(best_coords, pos);
            assert_eq!(best_n, n);
        }
    }

}