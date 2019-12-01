#[macro_use] mod error;
mod days;

use error::Error;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "aoc2019", about = "AoC2019 solutions")]
enum Day {
    Day1 {
        #[structopt(name = "FILE", parse(from_os_str))]
        input: PathBuf,
    }
}

fn main() {
    if let Err(e) = day(Day::from_args()) {
        eprintln!("{}", e);
    }
}

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

fn read(path: PathBuf) -> Result<String,Error> {
    use std::io::Read;
    let mut file = std::fs::File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}