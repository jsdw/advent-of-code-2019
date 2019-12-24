use std::collections::HashSet;
use crate::error::Error;
use self::bugs::Bugs;
use self::recursive_bugs::RecursiveBugs;

pub fn both_parts(input: &str) -> Result<(), Error> {

    {
        let mut bugs = Bugs::from_str(input);
        let mut seen = HashSet::new();
        while !seen.contains(&bugs) {
            seen.insert(bugs);
            bugs = bugs.step();
        }
        println!("Star 1: {}", bugs.biodiversity());
    }

    {

        let mut bugs = RecursiveBugs::from_str(input);
        for _ in 0..200 {
            bugs = bugs.step();
        }
        let count: u32 = bugs.values().map(|grid| grid.count_true()).sum();
        println!("Star 2: {}", count);
    }

    Ok(())
}

/// Recursive bugs for part 2
pub mod recursive_bugs {

    use std::collections::HashMap;
    use super::grid::Grid;

    #[derive(Debug,Clone)]
    pub struct RecursiveBugs(HashMap<i64,Grid>);

    impl RecursiveBugs {
        pub fn from_str(input: &str) -> RecursiveBugs {
            let mut m = HashMap::new();
            m.insert(0, Grid::from_str(input));
            RecursiveBugs(m)
        }
        pub fn step(&self) -> RecursiveBugs {
            let (top,bottom) = self.populated_range();
            let mut new_map = HashMap::new();
            for level in top-1..=bottom+1 {
                let grid = self.get_level(level);
                let mut new_grid = grid;
                for (x,y) in Grid::coords() {
                    // Ignore the middle; it's not a square now:
                    if y == 2 && x == 2 { continue }
                    let b = grid.get(x,y);
                    let c = adjacent(x,y,level)
                        .map(|((x,y),level)| self.get_level(level).get(x,y))
                        .filter(|&b| b)
                        .count();
                    if b && c != 1 {
                        new_grid.set(x, y, false);
                    } else if !b && (c == 1 || c == 2) {
                        new_grid.set(x, y, true);
                    }
                }
                new_map.insert(level, new_grid);
            }
            RecursiveBugs(new_map)
        }
        #[allow(unused)]
        pub fn print(&self) {
            let (top,bottom) = self.populated_range();
            for level in top..=bottom {
                println!("Level {}:", level);
                self.0.get(&level).unwrap_or(&Grid::empty()).print();
                println!();
            }
        }
        fn get_level(&self, level: i64) -> Grid {
            *self.0.get(&level).unwrap_or(&Grid::empty())
        }
        fn populated_range(&self) -> (i64,i64) {
            self.0.iter()
                .filter(|(_,grid)| grid.count_true() > 0)
                .fold((0,0), |(top,bottom),(&level,_)| (top.min(level), bottom.max(level)))
        }
    }

    impl std::ops::Deref for RecursiveBugs {
        type Target = HashMap<i64,Grid>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    fn adjacent(x: usize, y: usize, level: i64) -> impl Iterator<Item=((usize,usize),i64)> {
        let mut locations = vec![];

        // left:
        if x == 3 && y == 2 {
            (0..5).for_each(|y| locations.push(((4,y),level+1)));
        } else if x == 0 {
            locations.push(((1,2),level-1));
        } else {
            locations.push(((x-1,y),level));
        }

        // right:
        if x == 1 && y == 2 {
            (0..5).for_each(|y| locations.push(((0,y),level+1)));
        } else if x == 4 {
            locations.push(((3,2),level-1));
        } else {
            locations.push(((x+1,y),level));
        }

        // up:
        if x == 2 && y == 3 {
            (0..5).for_each(|x| locations.push(((x,4),level+1)));
        } else if y == 0 {
            locations.push(((2,1),level-1));
        } else {
            locations.push(((x,y-1),level));
        }

        // down:
        if x == 2 && y == 1 {
            (0..5).for_each(|x| locations.push(((x,0),level+1)));
        } else if y == 4 {
            locations.push(((2,3),level-1));
        } else {
            locations.push(((x,y+1),level));
        }

        locations.into_iter()
    }

}

/// Simple bugs for part 1
pub mod bugs {

    use super::grid::Grid;

    #[derive(Debug,Clone,Copy,Hash,PartialEq,Eq,PartialOrd,Ord)]
    pub struct Bugs(Grid);

    impl Bugs {
        pub fn from_str(input: &str) -> Bugs {
            Bugs(Grid::from_str(input))
        }
        pub fn step(&self) -> Bugs {
            let grid = self.0;
            let mut next = grid;
            for (x,y) in Grid::coords() {
                let b = grid.get(x,y);
                let c = adjacent(x,y)
                    .map(|(x,y)| grid.get(x,y))
                    .filter(|&b| b)
                    .count();
                if b && c != 1 {
                    next.set(x, y, false);
                } else if !b && (c == 1 || c == 2) {
                    next.set(x, y, true);
                }
            }
            Bugs(next)
        }
        pub fn biodiversity(&self) -> u32 {
            ***self
        }

    }

    impl std::ops::Deref for Bugs {
        type Target = Grid;
        fn deref(&self) -> &Grid {
            &self.0
        }
    }

    fn adjacent(x: usize, y: usize) -> impl Iterator<Item=(usize,usize)> {
        let left = if x > 0 { Some((x-1,y)) } else { None };
        let right = if x < 4 { Some((x+1,y)) } else { None };
        let up = if y > 0 { Some((x,y-1)) } else { None };
        let down = if y < 4 { Some((x,y+1)) } else { None };
        std::iter::empty().chain(left).chain(right).chain(up).chain(down)
    }

}

/// A 5x5 boolean grid stored in a u32.
pub mod grid {

    #[derive(Debug,Clone,Copy,Hash,PartialEq,Eq,PartialOrd,Ord)]
    pub struct Grid(u32);

    impl Grid {
        pub fn empty() -> Grid {
            Grid(0)
        }
        pub fn from_str(input: &str) -> Grid {
            let mut grid = Grid::empty();
            for (y,line) in input.trim().lines().enumerate() {
                for (x,c) in line.trim().chars().enumerate() {
                    if c == '#' { grid.set(x, y, true) }
                }
            }
            grid
        }
        pub fn set(&mut self, x: usize, y: usize, value: bool) {
            assert!(x < 5);
            assert!(y < 5);
            let idx = y * 5 + x;
            if value {
                self.0 = self.0 | 1u32 << idx;
            } else {
                self.0 = self.0 & !(1u32 << idx);
            }
        }
        pub fn get(&self, x: usize, y: usize) -> bool {
            assert!(x < 5);
            assert!(y < 5);
            let idx = y * 5 + x;
            (self.0 >> idx) & 1u32 == 1
        }
        pub fn count_true(&self) -> u32 {
            self.0.count_ones()
        }
        pub fn coords() -> impl Iterator<Item=(usize,usize)> {
            (0..5).flat_map(|y| (0..5).map(move |x| (x,y)))
        }
        #[allow(unused)]
        pub fn print(&self) {
            for y in 0..5 {
                for x in 0..5 {
                    print!("{}", if self.get(x,y) { '#' } else { '.' });
                }
                println!();
            }
        }
    }

    impl std::ops::Deref for Grid {
        type Target = u32;
        fn deref(&self) -> &u32 {
            &self.0
        }
    }

}