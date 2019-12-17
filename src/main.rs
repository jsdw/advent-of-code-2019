#[macro_use] mod error;
mod days;
mod support;

use error::Error;
use std::path::PathBuf;
use structopt::StructOpt;

/// A table-of-contents of the subcommands and their
/// arguments for this program.
#[derive(Debug, StructOpt)]
#[structopt(name = "aoc2019", about = "AoC2019 solutions")]
enum Day {
    Day1(FileInput),
    Day2(FileInput),
    Day3(FileInput),
    Day4 {
        #[structopt(help = "The first number in the range")]
        low: usize,
        #[structopt(help = "The last number in the range")]
        high: usize
    },
    Day5(FileInput),
    Day6(FileInput),
    Day7(FileInput),
    Day8 {
        #[structopt(name = "FILE", parse(from_os_str))]
        input: PathBuf,
        #[structopt(long, default_value = "25", help = "The image width")]
        image_width: usize,
        #[structopt(long, default_value = "6",  help = "The image height")]
        image_height: usize
    },
    Day9(FileInput),
    Day10(FileInput),
    Day11(FileInput),
    Day12(FileInput),
    Day13(FileInput),
    Day14(FileInput),
    Day15(FileInput),
    Day16(FileInput),
}

/// Days that take a file as input take one input arg:
#[derive(Debug, StructOpt)]
struct FileInput {
    #[structopt(name = "FILE", parse(from_os_str))]
    input: PathBuf,
}

/// Act on the subcommands and such provided using
/// the `Day` enum.
fn day(day: Day) -> Result<(),Error> {
    use self::Day::*;
    match day {
        Day1(FileInput { input }) => {
            let s = read(input)?;
            days::day01::part1(&s)?;
            days::day01::part2(&s)?;
        },
        Day2(FileInput { input }) => {
            let s = read(input)?;
            days::day02::part1(&s)?;
            days::day02::part2(&s)?;
        },
        Day3(FileInput { input }) => {
            let s = read(input)?;
            days::day03::part1(&s)?;
            days::day03::part2(&s)?;
        },
        Day4 { low, high } => {
            days::day04::part1(low, high)?;
            days::day04::part2(low, high)?;
        },
        Day5(FileInput { input }) => {
            let s = read(input)?;
            days::day05::part1(&s)?;
            days::day05::part2(&s)?;
        },
        Day6(FileInput { input }) => {
            let s = read(input)?;
            days::day06::both_parts(&s)?;
        },
        Day7(FileInput { input }) => {
            let s = read(input)?;
            days::day07::both_parts(&s)?;
        },
        Day8 { input, image_width, image_height } => {
            let s = read(input)?;
            days::day08::part1(&s, image_width, image_height)?;
            days::day08::part2(&s, image_width, image_height)?;
        },
        Day9(FileInput { input }) => {
            let s = read(input)?;
            days::day09::part1(&s)?;
            days::day09::part2(&s)?;
        },
        Day10(FileInput { input }) => {
            let s = read(input)?;
            days::day10::both_parts(&s)?;
        },
        Day11(FileInput { input }) => {
            let s = read(input)?;
            days::day11::both_parts(&s)?;
        },
        Day12(FileInput { input }) => {
            let s = read(input)?;
            days::day12::both_parts(&s)?;
        },
        Day13(FileInput { input }) => {
            let s = read(input)?;
            days::day13::both_parts(&s)?;
        },
        Day14(FileInput { input }) => {
            let s = read(input)?;
            days::day14::both_parts(&s)?;
        },
        Day15(FileInput { input }) => {
            let s = read(input)?;
            days::day15::both_parts(&s)?;
        },
        Day16(FileInput { input }) => {
            let s = read(input)?;
            days::day16::both_parts(&s)?;
        },
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
    Ok(std::fs::read_to_string(path)?)
}