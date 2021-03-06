use crate::error::Error;
use crate::support::intcode::parse_intcode_ops;
use self::breakout::{ Breakout, Outcome, Tile, Direction };
use std::collections::HashMap;

pub fn both_parts(input: &str) -> Result<(), Error> {
    let ops = parse_intcode_ops(input)?;

    // Star 1: run the game and count the blocks left:
    {
        let mut game = Breakout::new(ops.clone());
        let mut image = HashMap::new();
        while let Some(output) = game.step()? {
            if let Outcome::Draw { x, y, tile } = output {
                image.insert((x, y), tile);
            }
        }
        let blocks_left = image.values().filter(|&&v| v == Tile::Block).count();
        println!("Star 1: {}", blocks_left);
    }

    // Star 2: run the game, keep the paddle under the ball and
    // see what score we have when the game finishes:
    {
        let mut ops = ops;
        ops[0] = 2;
        let mut game = Breakout::new(ops);
        let mut score = 0;
        let mut ball_x: i64 = 0;
        let mut paddle_x: i64 = 0;
        while let Some(outcome) = game.step()? {
            match outcome {
                Outcome::Draw { x, tile, .. } => {
                    if let Tile::Ball = tile {
                        ball_x = x;
                    } else if let Tile::Paddle = tile {
                        paddle_x = x;
                    }
                },
                Outcome::Score(s) => {
                    score = s;
                },
                Outcome::MoveJoystick(provider) => {
                    let m = if ball_x < paddle_x {
                        Direction::Left
                    } else if ball_x > paddle_x {
                        Direction::Right
                    } else {
                        Direction::Neutral
                    };
                    game.move_joystick(provider.value(m))?;
                }
            }
        }
        println!("Star 2: {}", score);
    }

    Ok(())
}

/// This module implements a game of breakout using the provided intcode ops
pub mod breakout {

    use crate::error::Error;
    use crate::support::intcode::{
        Intcode,
        Outcome as IntcodeOutcome,
        ProvideInput as IntcodeProvideInput,
        ProvideInputValue as IntcodeProvideInputValue
    };

    pub struct Breakout {
        intcode: Intcode
    }

    impl Breakout {
        pub fn new(ops: Vec<i64>) -> Breakout {
            Breakout { intcode: Intcode::new(ops) }
        }
        pub fn move_joystick(&mut self, value: ProvideInputValue) -> Result<(),Error> {
            self.intcode.provide_input(value.0)
        }
        pub fn step(&mut self) -> Result<Option<Outcome>,Error> {
            let outcome = if let Some(outcome) = self.intcode.step()? {
                outcome
            } else {
                return Ok(None)
            };

            match outcome {
                IntcodeOutcome::Output(n1) => {
                    let n2 = if let Some(IntcodeOutcome::Output(n2)) = self.intcode.step()? {
                        n2
                    } else {
                        return Err(err!("Expected 3 output values in a row, but only got 1"))
                    };
                    let n3 = if let Some(IntcodeOutcome::Output(n3)) = self.intcode.step()? {
                        n3
                    } else {
                        return Err(err!("Expected 3 output values in a row, but only got 2"))
                    };

                    if n1 == -1 && n2 == 0 {
                        Ok(Some(Outcome::Score(n3)))
                    } else {
                        Ok(Some(Outcome::Draw {
                            x: n1,
                            y: n2,
                            tile: Tile::from_i64(n3)
                        }))
                    }
                },
                IntcodeOutcome::NeedsInput(provider) => {
                    Ok(Some(Outcome::MoveJoystick(ProvideInput(provider))))
                }
            }
        }
    }

    /// From running Breakout, we are either told to draw a tile,
    /// provided a score, or asked for joystick input.
    pub enum Outcome {
        Draw { x: i64, y: i64, tile: Tile },
        Score(i64),
        MoveJoystick(ProvideInput)
    }

    /// If we are asked to provide input, we pass a value to the
    /// thing handed back (this) and then hand that back to BReakout.
    pub struct ProvideInput(IntcodeProvideInput);
    pub struct ProvideInputValue(IntcodeProvideInputValue);

    impl ProvideInput {
        pub fn value(self, direction: Direction) -> ProvideInputValue {
            ProvideInputValue(match direction {
                Direction::Left => self.0.value(-1),
                Direction::Right => self.0.value(1),
                Direction::Neutral => self.0.value(0),
            })
        }
    }


    /// What direction would we like to move the joystick?
    #[derive(Clone,Copy,Eq,PartialEq)]
    pub enum Direction {
        Left,
        Right,
        Neutral
    }

    /// The type of tile to draw
    #[derive(Clone,Copy,Eq,PartialEq)]
    pub enum Tile {
        Empty,
        Wall,
        Block,
        Paddle,
        Ball
    }
    impl Tile {
        pub fn from_i64(n: i64) -> Tile {
            match n {
                1 => Tile::Wall,
                2 => Tile::Block,
                3 => Tile::Paddle,
                4 => Tile::Ball,
                _ => Tile::Empty
            }
        }
    }

}