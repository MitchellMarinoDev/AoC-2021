use std::collections::HashMap;
use crate::days::Day;

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
            let start: (u32, u32) = (pos1.next().unwrap().parse().unwrap(), pos1.next().unwrap().parse().unwrap());
            let end: (u32, u32) =   (pos2.next().unwrap().parse().unwrap(), pos2.next().unwrap().parse().unwrap());
            vents.push((start, end));
        }

        // Solve!
        let mut points = HashMap::new();
        for ((x1, y1), (x2, y2)) in vents.iter() {
            // First only consider horizontal or vertical lines
            if x1 == x2 {
                let range = if y1 > y2 { *y2..=*y1 } else { *y1..=*y2 };
                for y in range {
                    let p = (*x1, y);
                    let count = *points.get(&p).unwrap_or(&0);
                    points.insert(p, count+1);
                }
            }
            if y1 == y2 {
                let range = if x1 > x2 { *x2..=*x1 } else { *x1..=*x2 };
                for x in range {
                    let p = (x, *y1);
                    let count = *points.get(&p).unwrap_or(&0);
                    points.insert(p, count+1);
                }
            }
        }

        let count = points.iter().filter(|(_, count)| **count >= 2).count();

        (count.to_string(), "".to_string())
    }
}