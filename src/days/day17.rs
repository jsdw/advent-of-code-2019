use crate::error::Error;
use crate::support::intcode::{ Intcode, Outcome, parse_intcode_ops };
use crate::support::digits;
use std::collections::HashMap;
use self::Direction::*;

pub fn both_parts(input: &str) -> Result<(), Error> {
    let ops = parse_intcode_ops(input)?;
    let map = draw_map(ops)?;

    let alignments: i64 = find_intersections(&map).map(|(x,y)| x*y).sum();
    println!("Star 1: {}", alignments);

    let commands = generate_robot_commands(&map);
    let (a, commands) = replace_longest_reused_sequence(&commands, Command::A);
    let (b, commands) = replace_longest_reused_sequence(&commands, Command::B);
    let (c, commands) = replace_longest_reused_sequence(&commands, Command::C);
    println!("A: {:?}", a);
    println!("B: {:?}", b);
    println!("C: {:?}", c);
    println!("Commands:\n{:?}", commands);

    Ok(())
}

fn draw_map(ops: Vec<i64>) -> Result<Map,Error> {
    let mut map = HashMap::new();
    let mut intcode = Intcode::new(ops);
    let mut x = 0;
    let mut y = 0;
    while let Some(outcome) = intcode.step()? {
        match outcome {
            Outcome::Output(val) => {
                let val = val as u8;
                if val == b'\n' {
                    y += 1;
                    x = 0;
                } else {
                    map.insert((x,y), Feature::from_u8(val));
                    x += 1;
                }
                print!("{}", val as char);
            },
            Outcome::NeedsInput(_) => {
                return Err(err!("Intcode program asked for input, which shouldn't happen"))
            }
        }
    }
    Ok(Map::new(map))
}

/// Where does the scaffolding cross?
fn find_intersections(map: &Map) -> impl Iterator<Item=(i64,i64)> + '_ {
    fn surrounded_with_scaffold(coords: (i64,i64), map: &Map) -> bool {
        map.is_scaffold_at(coords) && surrounding(coords).all(|c| map.is_scaffold_at(c))
    }
    fn surrounding((x,y): (i64,i64)) -> impl Iterator<Item=(i64,i64)> {
        static DIFFS: [(i64,i64);4] = [(0,-1), (1,0), (0,1), (-1,0)];
        DIFFS.into_iter().map(move |(xd,yd)| (x+xd,y+yd))
    }
    map.keys().filter(move |&&c| surrounded_with_scaffold(c,map)).map(|&c| c)
}

/// Have the robot traverse the map from its current position until it leaves
/// the map, and return the list of commands required for it to do so.
fn generate_robot_commands(map: &Map) -> Vec<Command> {
    let (mut coords, mut direction) = find_robot(map);
    let mut commands = vec![];
    let mut forward_steps = 0;
    let commit_forward = |commands: &mut Vec<_>, forward_steps: &mut _| {
        if *forward_steps > 0 {
            commands.push(Command::Forward(*forward_steps))
        }
        *forward_steps = 0;
    };
    loop {
        if map.is_scaffold_at(direction.step_coords(coords)) {
            forward_steps += 1;
            coords = direction.step_coords(coords)
        } else if map.is_scaffold_at(direction.left().step_coords(coords)) {
            commit_forward(&mut commands, &mut forward_steps);
            commands.push(Command::Left);
            direction = direction.left();
        } else if map.is_scaffold_at(direction.right().step_coords(coords)) {
            commit_forward(&mut commands, &mut forward_steps);
            commands.push(Command::Right);
            direction = direction.right();
        } else {
            commit_forward(&mut commands, &mut forward_steps);
            return commands
        }
    }
}

fn replace_longest_reused_sequence(commands: &[Command], replace_with: Command) -> (Vec<Command>, Vec<Command>) {
    let mut best_dist = 0;
    let mut best_starts = (0,0);
    let len = commands.len();
    for a in 0..len {
        for b in a+1..len {
            for dist in 1..=(b-a).min(len-b).min(10) {
                if dist <= best_dist {
                    continue
                }
                let a_slice = &commands[a..a+dist];
                let b_slice = &commands[b..b+dist];
                if a_slice != b_slice {
                    continue
                }
                if a_slice.iter().any(|c| c.is_movement_func())
                || b_slice.iter().any(|c| c.is_movement_func()) {
                    continue
                }
                if Command::ascii_len(a_slice) > 20
                || Command::ascii_len(b_slice) > 20 {
                    continue
                }
                best_dist = dist;
                best_starts = (a,b);
            }
        }
    }
    let mut out = vec![];
    let mut replaced_a = false;
    let mut replaced_b = false;
    for (idx,c) in commands.iter().enumerate() {
        if idx >= best_starts.0 && idx < best_starts.0 + best_dist {
            if !replaced_a { out.push(replace_with); }
            replaced_a = true;
        } else if idx >= best_starts.1 && idx < best_starts.1 + best_dist {
            if !replaced_b { out.push(replace_with); }
            replaced_b = true;
        } else {
            out.push(*c);
        }
    }
    (commands[best_starts.0..best_starts.0+best_dist].to_vec(), out)
}

/// Find the robot on the map
fn find_robot(map: &Map) -> ((i64,i64), Direction) {
    map.iter()
        .filter_map(|(&c,&f)| if let Feature::Robot(d) = f { Some((c,d)) } else { None })
        .next()
        .unwrap()
}

/// A Map can contain these features
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
enum Feature {
    Scaffold,
    Robot(Direction),
    Space
}

impl Feature {
    fn from_u8(c: u8) -> Feature {
        match c {
            b'#'  => Feature::Scaffold,
            b'^'  => Feature::Robot(Direction::North),
            b'>'  => Feature::Robot(Direction::East),
            b'v'  => Feature::Robot(Direction::South),
            b'<'  => Feature::Robot(Direction::West),
            _     => Feature::Space
        }
    }
}

/// The robot is facing one of these directions
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
enum Direction {
    North,
    South,
    East,
    West
}

impl Direction {
    fn left(self) -> Direction {
        match self {
            North => West,
            West  => South,
            South => East,
            East  => North
        }
    }
    fn right(self) -> Direction {
        match self {
            North => East,
            East  => South,
            South => West,
            West  => North
        }
    }
    fn step_coords(self, (x,y): (i64,i64)) -> (i64,i64) {
        match self {
            North => (x,y-1),
            South => (x,y+1),
            East  => (x+1,y),
            West  => (x-1,y),
        }
    }
}

/// the robot can be issued these commands
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
enum Command {
    A,
    B,
    C,
    Left,
    Right,
    Forward(usize)
}

impl Command {
    fn ascii_len(commands: &[Command]) -> usize {
        commands.iter().map(|c| c.len()).sum::<usize>() + commands.len() - 1
    }
    fn len(self) -> usize {
        if let Command::Forward(n) = self {
            digits(n).count()
        } else {
            1
        }
    }
    fn is_movement_func(self) -> bool {
        self == Command::A || self == Command::B || self == Command::C
    }
}

/// A map showing where things are, with handy utility functions
/// for oft asked-for queries (but otherwise looking like a HashMap).
#[derive(Clone,Debug)]
struct Map {
    map: HashMap<(i64,i64), Feature>
}

impl Map {
    fn new(map: HashMap<(i64,i64), Feature>) -> Map {
        Map { map }
    }
    fn is_scaffold_at(&self, coords: (i64,i64)) -> bool {
        match self.get(&coords).unwrap_or(&Feature::Space) {
            Feature::Scaffold => true,
            Feature::Robot(_) => true,
            Feature::Space => false
        }
    }
}

impl std::ops::Deref for Map {
    type Target = HashMap<(i64,i64), Feature>;
    fn deref(&self) -> &Self::Target {
        &self.map
    }
}