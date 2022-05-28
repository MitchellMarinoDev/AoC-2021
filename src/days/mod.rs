pub trait Day {
    fn solve() -> (String, String);
}

pub mod d1;
pub mod d2;
pub mod d3;
pub mod d5;

use d1::Day1;
use d2::Day2;
use d3::Day3;

use d5::Day5;

pub const DAYS: [fn() -> (String, String); 4] = [
    Day1::solve,
    Day2::solve,
    Day3::solve,

    Day5::solve,
];

