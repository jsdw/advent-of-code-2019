use crate::error::Error;
use crate::support::intcode::{ parse_intcode_ops };
use self::robot::{ Robot, Colour, Outcome, Direction as TurnDirection };
use std::collections::HashMap;
use std::io::Write;

pub fn both_parts(input: &str) -> Result<(), Error> {
    let ops = parse_intcode_ops(input)?;
    let r = Robot::new(ops);

    let mut canvas: HashMap<(i64,i64),Colour> = HashMap::new();
    run_robot(r.clone(), &mut canvas)?;
    println!("Star 1: {}", canvas.len());

    let mut canvas: HashMap<(i64,i64),Colour> = HashMap::new();
    canvas.insert((0,0), Colour::White);
    run_robot(r.clone(), &mut canvas)?;
    println!("Star 2:");
    print_canvas(&canvas);

    Ok(())
}

/// Display the current canvas
fn print_canvas(canvas: &HashMap<(i64,i64),Colour>) {
    let [(x1,y1),(x2,y2)] = canvas.keys().fold([(std::i64::MAX,std::i64::MAX),(std::i64::MIN,std::i64::MIN)], |[(x1,y1),(x2,y2)], &(x,y)| {
        [(x1.min(x), y1.min(y)), (x2.max(x), y2.max(y))]
    });
    let stdout = std::io::stdout();
    let mut stdout = stdout.lock();
    for y in y1..=y2 {
        for x in x1..=x2 {
            if let Some(Colour::White) = canvas.get(&(x,y)) {
                stdout.write_all(b"#").unwrap();
            } else {
                stdout.write_all(b" ").unwrap();
            }
        }
        stdout.write_all(b"\n").unwrap();
    }
}

/// Run a robot given some canvas (starting the robot at 0,0). It will paint onto
/// the canvas, and we'll return an Error if something goes wrong.
fn run_robot(mut r: Robot, canvas: &mut HashMap<(i64,i64),Colour>) -> Result<(),Error> {
    let mut coords: (i64,i64) = (0,0);
    let mut direction: Direction = Direction::Up;
    while let Some(outcome) = r.step()? {
        match outcome {
            Outcome::PaintPanel(c) => {
                canvas.insert(coords,c);
            },
            Outcome::Turn(d) => {
                match d {
                    TurnDirection::Left => {
                        direction.rotate_left();
                    },
                    TurnDirection::Right => {
                        direction.rotate_right();
                    }
                }
                coords = direction.move_coords(coords);
            },
            Outcome::ProvidePanelColour(p) => {
                let c = canvas.get(&coords).map(|c| *c).unwrap_or(Colour::Black);
                p.provide(c);
            }
        }
    }
    Ok(())
}

/// What direction are we moving in
enum Direction {
    Up,
    Down,
    Left,
    Right
}
impl Direction {
    fn rotate_left(&mut self) {
        *self = match self {
            Direction::Up => Direction::Left,
            Direction::Left => Direction::Down,
            Direction::Down => Direction::Right,
            Direction::Right => Direction::Up,
        }
    }
    fn rotate_right(&mut self) {
        self.rotate_left();
        self.rotate_left();
        self.rotate_left();
    }
    fn move_coords(&self, (x,y): (i64,i64)) -> (i64,i64) {
        match self {
            Direction::Up => (x,y-1),
            Direction::Left => (x-1,y),
            Direction::Down => (x,y+1),
            Direction::Right => (x+1,y),
        }
    }
}

/// A robot that turns and moves, paints panels, and asks for panel colours.
pub mod robot {

    use crate::error::Error;
    use crate::support::intcode::{ Intcode, ProvideInput as IntcodeProvideInput, Outcome as IntcodeOutcome};

    #[derive(Clone)]
    pub struct Robot {
        intcode: Intcode,
        is_second_output: bool
    }

    impl Robot {
        pub fn new(ops: Vec<i64>) -> Robot {
            Robot { intcode: Intcode::new(ops), is_second_output: false }
        }
        pub fn step(&mut self) -> Result<Option<Outcome>,Error> {
            if let Some(outcome) = self.intcode.step()? {
                match outcome {
                    IntcodeOutcome::NeedsInput(provider) => {
                        Ok(Some(Outcome::ProvidePanelColour(ProvideInput(provider))))
                    },
                    IntcodeOutcome::Output(val) => {
                        if !self.is_second_output {
                            let c = if val == 0 { Colour::Black } else { Colour::White };
                            self.is_second_output = true;
                            Ok(Some(Outcome::PaintPanel(c)))
                        } else {
                            let d = if val == 0 { Direction::Left } else { Direction::Right };
                            self.is_second_output = false;
                            Ok(Some(Outcome::Turn(d)))
                        }
                    }
                }
            } else {
                Ok(None)
            }
        }
    }

    /// The robot can provide back these outcomes each step
    pub enum Outcome<'a> {
        PaintPanel(Colour),
        Turn(Direction),
        ProvidePanelColour(ProvideInput<'a>)
    }

    /// The robot can ask what colour panel it's over.
    pub struct ProvideInput<'a>(IntcodeProvideInput<'a>);
    impl <'a> ProvideInput<'a> {
        pub fn provide(self, p: Colour) {
            match p {
                Colour::Black => self.0.provide(0),
                Colour::White => self.0.provide(1)
            }
        }
    }

    /// The robot can rutn either left or right.
    #[derive(Clone,Copy,PartialEq,Eq)]
    pub enum Direction {
        Left,
        Right
    }

    /// The robot can paint panels either black or white.
    #[derive(Clone,Copy,PartialEq,Eq)]
    pub enum Colour {
        Black,
        White
    }

}