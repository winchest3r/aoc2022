mod first;
mod second;
mod third;

/// Basic trait for all solutions.
pub trait Solution {
    /// Solve part 1 of solution.
    fn part1(#[allow(unused)] input: &str) -> String {
        "unimpl".to_string()
    }

    /// Solve part 2 of solution.
    fn part2(#[allow(unused)] input: &str) -> String {
        "unimpl".to_string()
    }
}

pub mod prelude {
    pub use crate::aoc2022::Solution;
    pub use crate::aoc2022::first::FirstSolution;
    pub use crate::aoc2022::second::SecondSolution;
    pub use crate::aoc2022::third::ThirdSolution;
}
