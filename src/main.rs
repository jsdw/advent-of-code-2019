#[macro_use] mod error;
mod days;

use error::Error;
use std::path::PathBuf;
use structopt::StructOpt;

/// A table-of-contents of the subcommands and their
/// arguments for this program.
#[derive(Debug, StructOpt)]
#[structopt(name = "aoc2019", about = "AoC2019 solutions")]
enum Day {
    Day1 {
        #[structopt(name = "FILE", parse(from_os_str))]
        input: PathBuf,
    }
}

/// Act on the subcommands and such provided using
/// the `Day` enum.
fn day(day: Day) -> Result<(),Error> {
    use self::Day::*;
    match day {
        Day1 { input } => {
            let s = read(input)?;
            days::day01::part1(&s)?;
            days::day01::part2(&s)?;
        }
    };
    Ok(())
}

/// Parse the arguments, run the relevant code and
/// print any errors to stderr.
fn main() {
    if let Err(e) = day(Day::from_args()) {
        eprintln!("{}", e);
    }
}

/// A convenience function to read from a file.
fn read(path: PathBuf) -> Result<String,Error> {
    use std::io::Read;
    let mut file = std::fs::File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}