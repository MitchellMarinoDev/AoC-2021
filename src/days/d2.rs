use crate::Day;
use std::convert::{TryFrom, TryInto};

pub struct Day2;

const INPUT: &str = "inputs/d2.txt";

#[derive(Copy, Clone, Eq, PartialEq)]
enum Direction {
    Forward,
    Up,
    Down,
}

impl Day for Day2 {
    fn solve() -> (String, String) {
        let raw = std::fs::read_to_string(INPUT).expect(&*format!("Check input file {}", INPUT));
        let parsed: Vec<(Direction, i32)> = raw
            .lines()
            .map(|l| {
                let mut s = l.split(" ");
                let dir = s.next().unwrap().try_into().unwrap();
                let dist = s.next().unwrap().parse().unwrap();
                (dir, dist)
            })
            .collect();

        // pt 1
        let mut hor = 0;
        let mut depth = 0;
        for (dir, dist) in parsed.iter() {
            match dir {
                Direction::Forward => hor += dist,
                Direction::Up => depth -= dist,
                Direction::Down => depth += dist,
            }
        }
        let pt1 = hor * depth;

        // pt2
        let mut aim = 0;
        let mut hor = 0;
        let mut depth = 0;
        for (dir, dist) in parsed.iter() {
            match dir {
                Direction::Forward => {
                    hor += dist;
                    depth += aim * dist;
                }
                Direction::Up => aim -= dist,
                Direction::Down => aim += dist,
            }
        }
        let pt2 = hor * depth;

        (pt1.to_string(), pt2.to_string())
    }
}

// must be lowercase
impl TryFrom<&str> for Direction {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "forward" => Ok(Direction::Forward),
            "down" => Ok(Direction::Down),
            "up" => Ok(Direction::Up),
            _ => Err(()),
        }
    }
}
