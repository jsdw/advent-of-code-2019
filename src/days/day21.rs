use crate::error::Error;
use crate::support::{ Terminal };
use std::io::{ sink };

static PART1: &str = "
# If there's a hole in A, B or C, set T to true:
NOT A T
NOT B J
OR J T
NOT C J
OR J T

# Set J to true if there's *not* a hole in D:
NOT D J
NOT J J

# If there's a hole in A/B/C and no hole in D, J = true:
AND T J

WALK
";

static PART2: &str = "
# If we would jump in part 1, set T to true:
NOT A T
NOT B J
OR J T
NOT C J
OR J T
NOT D J
NOT J J
AND J T # use T, not J, so J is free below.

# If a jump to D would leave us unable to move or jump,
# (E is false and H is false), don't jump:
NOT E J # J = false if no ground at E
NOT J J # J = true if ground at E
OR H J # J = true if ground at E *or* H

# Jump if ground at E or H plus part1 conditions:
AND T J

RUN
";

pub fn both_parts(input: &str) -> Result<(), Error> {

    {
        let commands: Vec<u8> = prepare_commands(PART1);
        let mut terminal = Terminal::from_str(input, commands.as_slice(), sink())?;
        if let Some(n) = terminal.step()? {
            println!("Star 1: {}", n);
        } else {
            println!("Star 1 unsuccessful");
        }
    }

    {
        let commands: Vec<u8> = prepare_commands(PART2);
        let mut terminal = Terminal::from_str(input, commands.as_slice(), sink())?;
        if let Some(n) = terminal.step()? {
            println!("Star 2: {}", n);
        } else {
            println!("Star 2 unsuccessful");
        }
    }

    Ok(())
}

pub fn prepare_commands(commands: &str) -> Vec<u8> {
    let mut out = vec![];
    for line in commands.trim().lines() {
        let s = if let Some(n) = line.find('#') {
            line[0..n].trim()
        } else {
            line
        };
        if s.len() > 0 {
            out.extend_from_slice(s.as_bytes());
            out.push(b'\n');
        }
    }
    out
}