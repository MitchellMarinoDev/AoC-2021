use crate::days::Day;

pub struct Day1;

const INPUT: &str = "inputs/d1.txt";

impl Day for Day1 {
    fn solve() -> (String, String) {
        let input = std::fs::read_to_string(INPUT).expect("Check input file");
        let parsed: Vec<_> = input.lines().map(|s| s.parse::<u32>().unwrap()).collect();

        // Part 1
        let mut old = u32::MAX;
        let mut p1 = 0;
        for &n in parsed.iter() {
            if n > old {
                p1 += 1;
            }
            old = n;
        }

        // Part 2
        let iter = parsed.iter()
            .zip(parsed.iter().skip(1))
            .zip(parsed.iter().skip(2))
            .map(|((x, y), z)| (x, y, z));

        let mut old = u32::MAX;
        let mut p2 = 0;
        for (x, y, z) in iter {
            let s = x + y + z;
            if s > old {
                p2 += 1;
            }
            old = s;
        }

        (p1.to_string(), p2.to_string())
    }
}