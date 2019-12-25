use crate::error::Error;
use crate::support::{ Terminal, IntoReadIter, Uppercase };
use std::io::{ Read, sink, stdin, stdout };

pub fn both_parts(input: &str, interactive: bool) -> Result<(), Error> {

    if interactive {
        let mut terminal = Terminal::from_str(input, Uppercase::new(stdin()), stdout())?;
        if let Some(n) = terminal.step()? {
            println!("Result: {}", n);
        } else {
            println!("No result obtained");
        }
        return Ok(())
    }

    {
        let mut terminal = Terminal::from_str(input, prepare_commands(PART1), sink())?;
        if let Some(n) = terminal.step()? {
            println!("Star 1: {}", n);
        } else {
            println!("Star 1 unsuccessful");
        }
    }

    {
        let mut terminal = Terminal::from_str(input, prepare_commands(PART2), sink())?;
        if let Some(n) = terminal.step()? {
            println!("Star 2: {}", n);
        } else {
            println!("Star 2 unsuccessful");
        }
    }

    Ok(())
}

pub fn prepare_commands(commands: &str) -> impl Read {
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
    out.into_iter().into_reader()
}

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