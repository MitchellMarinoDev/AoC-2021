pub trait Day {
    fn solve() -> (String, String);
}

pub mod d1;
pub mod d2;

use d1::Day1;
use d2::Day2;

pub const DAYS: [fn() -> (String, String); 2] = [
    Day1::solve,
    Day2::solve,
];

