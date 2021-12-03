pub trait Day {
    fn solve() -> (String, String);
}

pub mod d1;
pub mod d2;
pub mod d3;

use d1::Day1;
use d2::Day2;
use crate::days::d3::Day3;

pub const DAYS: [fn() -> (String, String); 3] = [
    Day1::solve,
    Day2::solve,
    Day3::solve,
];

