pub trait Day {
    fn solve() -> (String, String);
}

pub mod d1;
pub mod d2;
pub mod d3;
pub mod d4;
pub mod d5;
pub mod d6;
pub mod d7;

use d1::Day1;
use d2::Day2;
use d3::Day3;
use d4::Day4;
use d5::Day5;
use d6::Day6;
use d7::Day7;

pub const DAYS: [fn() -> (String, String); 7] = [
    Day1::solve,
    Day2::solve,
    Day3::solve,
    Day4::solve,
    Day5::solve,
    Day6::solve,
    Day7::solve,
];
