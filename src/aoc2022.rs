mod eighteenth;
mod eighth;
mod eleventh;
mod fifteenth;
mod fifth;
mod first;
mod fourteenth;
mod fourth;
mod nineteenth;
mod ninth;
mod second;
mod seventeenth;
mod seventh;
mod sixteenth;
mod sixth;
mod tenth;
mod third;
mod thirteenth;
mod twelfth;
mod twentieth;
mod twentyfirst;

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
    pub use crate::aoc2022::eighteenth::EighteenthSolution;
    pub use crate::aoc2022::eighth::EighthSolution;
    pub use crate::aoc2022::eleventh::EleventhSolution;
    pub use crate::aoc2022::fifteenth::FifteenthSolution;
    pub use crate::aoc2022::fifth::FifthSolution;
    pub use crate::aoc2022::first::FirstSolution;
    pub use crate::aoc2022::fourteenth::FourteenthSolution;
    pub use crate::aoc2022::fourth::FourthSolution;
    pub use crate::aoc2022::nineteenth::NineteenthSolution;
    pub use crate::aoc2022::ninth::NinthSolution;
    pub use crate::aoc2022::second::SecondSolution;
    pub use crate::aoc2022::seventeenth::SeventeenthSolution;
    pub use crate::aoc2022::seventh::SeventhSolution;
    pub use crate::aoc2022::sixteenth::SixteenthSolution;
    pub use crate::aoc2022::sixth::SixthSolution;
    pub use crate::aoc2022::tenth::TenthSolution;
    pub use crate::aoc2022::third::ThirdSolution;
    pub use crate::aoc2022::thirteenth::ThirteenthSolution;
    pub use crate::aoc2022::twelfth::TwelfthSolution;
    pub use crate::aoc2022::twentieth::TwentiethSolution;
    pub use crate::aoc2022::twentyfirst::TwentyFirstSolution;
}
