use crate::days::Day;
use std::collections::HashMap;

pub struct Day5;

const INPUT: &str = "inputs/d5.txt";

impl Day for Day5 {
    fn solve() -> (String, String) {
        let raw = std::fs::read_to_string(INPUT).expect(&*format!("Check input file {}", INPUT));
        let mut vents = vec![];
        for l in raw.trim().lines() {
            let mut split = l.split(" -> ");
            let mut pos1 = split.next().unwrap().trim().split(",");
            let mut pos2 = split.next().unwrap().trim().split(",");
            let start: (u32, u32) = (
                pos1.next().unwrap().parse().unwrap(),
                pos1.next().unwrap().parse().unwrap(),
            );
            let end: (u32, u32) = (
                pos2.next().unwrap().parse().unwrap(),
                pos2.next().unwrap().parse().unwrap(),
            );
            vents.push((start, end));
        }

        // Solve!
        let mut points = HashMap::new();
        for ((x1, y1), (x2, y2)) in vents.iter() {
            // First only consider horizontal or vertical lines
            if x1 == x2 {
                for y in range(*y1, *y2) {
                    let p = (*x1, y);
                    let count = *points.get(&p).unwrap_or(&0);
                    points.insert(p, count + 1);
                }
            } else if y1 == y2 {
                for x in range(*x1, *x2) {
                    let p = (x, *y1);
                    let count = *points.get(&p).unwrap_or(&0);
                    points.insert(p, count + 1);
                }
            }
        }

        let p1 = points.iter().filter(|(_, count)| **count >= 2).count();

        // P2
        for ((x1, y1), (x2, y2)) in vents.iter() {
            // Now, only handle the diagonals
            if x1 != x2 && y1 != y2 {
                let range_x = range(*x1, *x2);
                let range_y = range(*y1, *y2);
                for point in range_x.into_iter().zip(range_y) {
                    let count = *points.get(&point).unwrap_or(&0);
                    points.insert(point, count + 1);
                }
            }
        }

        let p2 = points.iter().filter(|(_, count)| **count >= 2).count();

        (p1.to_string(), p2.to_string())
    }
}

/// Gets an inclusive range that can have the either number be bigger.
fn range(n1: u32, n2: u32) -> Vec<u32> {
    if n1 > n2 {
        (n2..=n1).rev().collect()
    } else {
        (n1..=n2).collect()
    }
}
