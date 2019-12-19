use crate::error::Error;
use crate::support::intcode::{ Intcode, Outcome, parse_intcode_ops };
use crate::support::digits;
use std::collections::HashMap;
use std::iter::once;
use self::Direction::*;

pub fn both_parts(input: &str) -> Result<(), Error> {
    let ops = parse_intcode_ops(input)?;
    let map = draw_map(ops.clone())?;

    let alignments: i64 = find_intersections(&map).map(|(x,y)| x*y).sum();
    println!("Star 1: {}", alignments);

    let mut ops = ops;
    ops[0] = 2;
    let commands = generate_robot_commands(&map);
    let dust_collected = run_commands(ops, replace_all_commands(&commands)?)?;
    println!("Star 2: {}", dust_collected);

    Ok(())
}

/// Draw the map we'll be traversing.
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
            },
            Outcome::NeedsInput(_) => {
                return Err(err!("Intcode program asked for input, which shouldn't happen"))
            }
        }
    }
    Ok(Map::new(map))
}

/// Given slightly modified ops, this feeds in the movement commands and such
/// that we need to run in order to move the robot to the end of the scaffolding,
/// and returns the final value given back.
fn run_commands(
    ops: Vec<i64>,
    cmds: Replacements
) -> Result<i64,Error> {
    let mut intcode = Intcode::new(ops);
    let mut last_output = 0;
    let mut input = Command::to_ascii(&cmds.main)
        .chain(once(b'\n'))
        .chain(Command::to_ascii(&cmds.a))
        .chain(once(b'\n'))
        .chain(Command::to_ascii(&cmds.b))
        .chain(once(b'\n'))
        .chain(Command::to_ascii(&cmds.c))
        .chain(once(b'\n'))
        .chain(once(b'n'))
        .chain(once(b'\n'));
    while let Some(outcome) = intcode.step()? {
        match outcome {
            Outcome::NeedsInput(p) => {
                if let Some(c) = input.next() {
                    intcode.provide_input(p.value(c as i64))?;
                } else {
                    return Err(err!("No more input to provide but input asked for"))
                }
            }
            Outcome::Output(i) => {
                last_output = i;
            }
        }
    }
    Ok(last_output)
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

/// Give back every possible subslice from some slice of items.
fn subslices<T>(slice: &[T]) -> impl Iterator<Item=&[T]> + '_ {
    (0..slice.len()).flat_map(move |start| {
        (start+1..slice.len()).map(move |end| {
            &slice[start..end]
        })
    })
}

/// Find all non overlapping indexes of some subslice.
fn indexof<'a,T: PartialEq>(slice: &'a [T], subslice: &'a [T]) -> impl Iterator<Item=usize> + 'a {
    let mut index = 0;
    let len = slice.len();
    let sublen = subslice.len();
    std::iter::from_fn(move || {
        let thisindex = index;
        index += 1;
        if thisindex + sublen > len {
            return None
        }
        if &slice[thisindex..thisindex+sublen] == subslice {
            Some(Some(thisindex))
        } else {
            Some(None)
        }
    }).filter_map(|i| i)
}

/// Is the list of commands given valid as a movement function?
fn is_valid_movement_function(commands: &[Command]) -> bool {
    if commands.iter().any(|c| c.is_movement_func()) {
        false
    } else if Command::ascii_len(commands) > 20 {
        false
    } else {
        true
    }
}

/// Given some sequence of commands, very inefficiently find the three
/// mvoement functions such that the main command only references movement
/// functions, and nothing is greater than 20 ascii chars in length.
fn replace_all_commands(commands: &[Command]) -> Result<Replacements,Error> {
    for (a, commands) in possible_replacements(commands, Command::A) {
        for (b, commands) in possible_replacements(&commands, Command::B) {
            for (c, commands) in possible_replacements(&commands, Command::C) {
                if Command::ascii_len(&commands) > 20 {
                    continue
                }
                if commands.iter().any(|c| !c.is_movement_func()) {
                    continue
                }
                return Ok(Replacements {
                    a: a.to_vec(),
                    b: b.to_vec(),
                    c: c.to_vec(),
                    main: commands
                })
            }
        }
    }
    Err(err!("No suitable combinations found"))
}

struct Replacements {
    a: Vec<Command>,
    b: Vec<Command>,
    c: Vec<Command>,
    main: Vec<Command>
}

/// Find all subslices of the commands given that would be valid movement
/// functions, returning the new commands and slice used as the movement
/// function as a result.
fn possible_replacements(commands: &[Command], replace_with: Command) -> impl Iterator<Item=(&[Command], Vec<Command>)> + '_ {
    subslices(commands)
        .filter(|&c| is_valid_movement_function(c))
        .map(move |sub| {
            let mut start_idx = 0;
            let mut next = vec![];
            for idx in indexof(&commands, sub) {
                for n in start_idx..idx {
                    next.push(commands[n])
                }
                start_idx = idx + sub.len();
                next.push(replace_with);
            }
            for n in start_idx..commands.len() {
                next.push(commands[n]);
            }
            (sub, next)
        })
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
    fn to_ascii(commands: &[Command]) -> impl Iterator<Item=u8> {
        let mut out = vec![];
        for c in commands {
            match c {
                Command::A => out.push(b'A'),
                Command::B => out.push(b'B'),
                Command::C => out.push(b'C'),
                Command::Left => out.push(b'L'),
                Command::Right => out.push(b'R'),
                Command::Forward(n) => {
                    n.to_string().chars().for_each(|c| out.push(c as u8))
                }
            }
            out.push(b',');
        }
        out.pop(); // remove trailing comma
        out.into_iter()
    }
    fn ascii_len(commands: &[Command]) -> usize {
        if commands.len() == 0 { return 0 }
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