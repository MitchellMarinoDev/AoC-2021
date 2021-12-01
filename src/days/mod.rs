pub trait Day {
    fn solve() -> (String, String);
}

pub const DAYS: [fn() -> (String, String); 1] = [
    Day1::solve,
];

pub mod d1;

pub use d1::Day1;
