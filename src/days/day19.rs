use crate::error::Error;
use self::drones::Drones;
use std::io::Write;

pub fn both_parts(input: &str) -> Result<(), Error> {

    let drones = Drones::new(input)?;

    // First, display the beam and count pulled points:
    let mut pulled_points = 0;
    {
        let stdout = std::io::stdout();
        let mut stdout = stdout.lock();
        for y in 0..50 {
            for x in 0..50 {
                if drones.is_pulled_at(x,y)? {
                    pulled_points += 1;
                    stdout.write_all(b"#")?;
                } else {
                    stdout.write_all(b".")?;
                }
            }
            stdout.write_all(b"\n")?;
        }
    }
    println!("Star 1: {}", pulled_points);

    // Next, work diagonally down and scan for a diagonal
    // large enough to host a 100x100 square.
    let mut best_topright = (0,0);
    let mut best_bottomleft = (0,0);
    let mut best_manhatten = 0;
    let mut xy = (49,49);
    while best_manhatten < 200 {

        // My beam is slightly above horizontal so adjust up to it:
        while !drones.is_pulled_at(xy.0,xy.1)? {
            xy = (xy.0+1,xy.1-1)
        }

        // Now keep going up until we leave the beam again:
        let mut topright = xy;
        while drones.is_pulled_at(topright.0+1,topright.1-1)? {
            topright = (topright.0+1,topright.1-1);
        }

        let dist = manhatten(topright, xy) + 2;
        if dist > best_manhatten {
            best_manhatten = dist;
            best_topright = topright;
            best_bottomleft = xy;
        }

        xy.0 += 1;
        xy.1 += 1;
    }
    let topleft = (best_bottomleft.0,best_topright.1);
    println!("Star 2: {}", topleft.0 * 10_000 + topleft.1);

    Ok(())
}

fn manhatten((x1,y1): (usize,usize), (x2,y2): (usize,usize)) -> usize {
    let xd = if x1 < x2 { x2 - x1 } else { x1 - x2 };
    let yd = if y1 < y2 { y2 - y1 } else { y1 - y2 };
    xd + yd
}

/// Our drones program.
mod drones {

    use std::iter;
    use crate::error::Error;
    use crate::support::intcode::{ parse_intcode_ops, Intcode, Outcome };

    pub struct Drones {
        ops: Vec<i64>,
    }
    impl Drones {
        pub fn new(input: &str) -> Result<Drones,Error> {
            let ops = parse_intcode_ops(input)?;
            Ok(Drones { ops })
        }
        pub fn is_pulled_at(&self, x: usize, y: usize) -> Result<bool,Error> {
            let mut intcode = Intcode::new(self.ops.clone());
            let mut input = iter::once(x).chain(iter::once(y));
            loop {
                if let Some(outcome) = intcode.step()? {
                    match outcome {
                        Outcome::NeedsInput(p) => {
                            if let Some(i) = input.next() {
                                intcode.provide_input(p.value(i as i64))?;
                            } else {
                                return Err(err!("Unexpected input requirement"))
                            }
                        },
                        Outcome::Output(v) => {
                            return Ok(v != 0)
                        }
                    }
                }
            }
        }
    }

}

