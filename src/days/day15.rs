use crate::error::Error;
use crate::support::intcode::{ parse_intcode_ops };
use self::droid::{ Droid, Outcome, Status, Direction };
use std::collections::{ HashMap, HashSet, VecDeque };
use std::io::Write;

pub fn both_parts(input: &str) -> Result<(), Error> {
    let ops = parse_intcode_ops(input)?;

    // Build a map of the area:
    let droid = Droid::new(ops);
    let map = build_map(droid)?;
    print_map(&map);

    // Find the oxygen station and calculate the distance from droid to it:
    let station_coords = map
        .iter()
        .find(|&(_,&f)| f == Feature::OxygenStation)
        .map(|(&xy,_)| xy)
        .unwrap();
    println!("Star 1: {}", route_between((0,0), station_coords, &map).count());

    // Calculate max distance between station and furthest reachable map pos:
    let dist = all_surrounding(station_coords, &map)
        .map(|(d,_)| d)
        .max()
        .unwrap();
    println!("Star 2: {}", dist);

    Ok(())
}

/// Build up a map of the area by moving the droid to all unknown coords
/// until there are none that are accessible.
fn build_map(mut droid: Droid) -> Result<HashMap<(i64,i64), Feature>,Error> {
    let mut map = HashMap::new();
    let mut coords = (0,0);
    let mut direction = Direction::North;
    map.insert(coords, Feature::Empty);
    while let Some(outcome) = droid.step()? {
        match outcome {
            Outcome::Move(p) => {
                if let Some(d) = pick_direction(coords, &map) {
                    direction = d;
                    droid.try_move(p.value(direction))?;
                } else {
                    return Ok(map)
                }
            },
            Outcome::Status(s) => {
                match s {
                    Status::Moved { found_oxygen } => {
                        coords = next_coords(coords, direction);
                        let f = if found_oxygen { Feature::OxygenStation } else { Feature::Empty };
                        map.insert(coords, f);
                    },
                    Status::HitWall => {
                        let wall_coords = next_coords(coords, direction);
                        map.insert(wall_coords, Feature::Wall);
                    }
                }
            }
        }
    }
    Err(err!("Unexpected program end"))
}

/// Print the map
fn print_map(map: &HashMap<(i64,i64), Feature>) -> Result<(),Error> {
    let [(x1,y1),(x2,y2)] = map.keys().fold([(0,0),(0,0)], |[(x1,y1),(x2,y2)], &(x,y)| {
        [(x1.min(x), y1.min(y)), (x2.max(x), y2.max(y))]
    });
    let stdout = std::io::stdout();
    let mut stdout = stdout.lock();
    for y in y1..=y2 {
        for x in x1..=x2 {
            match map.get(&(x,y)) {
                None => stdout.write_all(b" ")?,
                Some(Feature::Empty) => stdout.write_all(if (x,y) == (0,0) { b"d" } else { b"." })?,
                Some(Feature::Wall) => stdout.write_all(b"#")?,
                Some(Feature::OxygenStation) => stdout.write_all(b"o")?,
            };
        }
        stdout.write_all(b"\n")?;
    }
    Ok(())
}

/// The heart of the movement code; decide which square to try to move to next.
/// return None to not move the droid at all (ie we've seen everything we care about).
fn pick_direction(coords: (i64,i64), map: &HashMap<(i64,i64), Feature>) -> Option<Direction> {
    find_closest_unseen_coords(coords, map)
        .and_then(|end| route_between(coords, end, map).next())
}

/// Find the nearest unseen coords, None if no such coords are reachable.
fn find_closest_unseen_coords(coords: (i64,i64), map: &HashMap<(i64,i64), Feature>) -> Option<(i64,i64)> {
    all_surrounding(coords, map)
        .filter(|(_,c)| !map.contains_key(c))
        .map(|(_,c)| c)
        .next()
}

/// Find a route from one set of coords to another via known map.
fn route_between(start: (i64,i64), end: (i64,i64), map: &HashMap<(i64,i64),Feature>) -> impl Iterator<Item=Direction> {
    let mut curr = start;
    let distance_from_end: HashMap<(i64,i64),usize> = all_surrounding(end, map)
        .take_while(|&(_,c)| c != start)
        .map(|(d,c)| (c,d))
        .chain([(end,0)].into_iter().cloned())
        .collect();
    std::iter::from_fn(move || {
        if curr == end {
            None
        } else {
            let d = *[Direction::North, Direction::East, Direction::South, Direction::West]
                .into_iter()
                .min_by_key(|&&d| *distance_from_end.get(&next_coords(curr, d)).unwrap_or(&std::usize::MAX))
                .unwrap();
            curr = next_coords(curr, d);
            Some(d)
        }
    })
}

/// An iterator over all coords from those provided, ordered by distance, and taking
/// into account any walls that we know about in our map.
fn all_surrounding<'a>(coords: (i64,i64), map: &'a HashMap<(i64,i64),Feature>) -> impl Iterator<Item=(usize,(i64,i64))> + 'a {
    let mut tried: HashSet<(i64,i64)> = HashSet::new();
    let mut next: VecDeque<(usize,(i64,i64))> = VecDeque::new();
    next.push_back((0,coords));
    tried.insert(coords);
    std::iter::from_fn(move || {
        let (next_distance, next_coords)
            = if let Some(c) = next.pop_front() { c } else { return None };
        for c in surrounding(next_coords) {
            if tried.contains(&c) {
                continue
            }
            if map.get(&c).unwrap_or(&Feature::Empty) == &Feature::Wall {
                continue
            }
            next.push_back((next_distance+1,c));
            tried.insert(c);
        }
        next.get(0).cloned()
    })
}

/// Give back an iterator of the coords directly touching those provided.
fn surrounding((x,y): (i64,i64)) -> impl Iterator<Item=(i64,i64)> {
    static DIFFS: [(i64,i64);4] = [(0,-1), (1,0), (0,1), (-1,0)];
    DIFFS.into_iter().map(move |(xd,yd)| (x+xd,y+yd))
}

/// Use a direction to transform some coords into new coords based on it.
fn next_coords((x,y): (i64,i64), d: Direction) -> (i64,i64) {
    match d {
        Direction::North => (x,y-1),
        Direction::South => (x,y+1),
        Direction::East => (x+1,y),
        Direction::West => (x-1,y)
    }
}

#[derive(Debug,Clone,Copy,PartialEq,Eq)]
enum Feature {
    Empty,
    Wall,
    OxygenStation
}

/// Wrap our intcode interpreter into a repair droid, which
/// we can move about and provides feedback on what it sees.
mod droid {

    use crate::error::Error;
    use crate::support::intcode::{
        Intcode,
        Outcome as IntcodeOutcome,
        ProvideInput as IntcodeProvideInput,
        ProvideInputValue as IntcodeProvideInputValue
    };

    pub struct Droid {
        intcode: Intcode
    }

    impl Droid {
        pub fn new(ops: Vec<i64>) -> Droid {
            Droid { intcode: Intcode::new(ops) }
        }
        pub fn try_move(&mut self, value: ProvideInputValue) -> Result<(),Error> {
            self.intcode.provide_input(value.0)
        }
        pub fn step(&mut self) -> Result<Option<Outcome>,Error> {
            if let Some(outcome) = self.intcode.step()? {
                match outcome {
                    IntcodeOutcome::Output(n) => {
                        let s = match n {
                            1 => Status::Moved { found_oxygen: false },
                            2 => Status::Moved { found_oxygen: true },
                            _ => Status::HitWall,
                        };
                        Ok(Some(Outcome::Status(s)))
                    },
                    IntcodeOutcome::NeedsInput(p) => {
                        Ok(Some(Outcome::Move(ProvideInput(p))))
                    }
                }
            } else {
                Ok(None)
            }
        }
    }

    pub enum Outcome {
        Move(ProvideInput),
        Status(Status)
    }

    #[derive(Debug,Clone,Copy)]
    pub enum Direction {
        North = 1, South = 2, West = 3, East = 4
    }

    #[derive(Debug,Clone,Copy)]
    pub enum Status {
        Moved { found_oxygen: bool },
        HitWall,
    }

    pub struct ProvideInput(IntcodeProvideInput);
    pub struct ProvideInputValue(IntcodeProvideInputValue);

    impl ProvideInput {
        pub fn value(self, d: Direction) -> ProvideInputValue {
            ProvideInputValue(self.0.value(d as i64))
        }
    }

}