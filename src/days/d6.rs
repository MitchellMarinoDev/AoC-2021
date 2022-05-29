use crate::days::Day;

pub struct Day6;

const INPUT: &str = "inputs/d6.txt";

impl Day for Day6 {
    fn solve() -> (String, String) {
        let raw = std::fs::read_to_string(INPUT).expect(&*format!("Check input file {}", INPUT));

        // The number of fish in each state.
        let mut counts = [0; 9];
        for s in raw.trim().split(',') {
            let n: usize = s.parse().unwrap();
            counts[n] += 1;
        }

        for _ in 0..80 {
            simulate(&mut counts);
        }
        let p1: u64 = counts.iter().sum();

        for _ in 80..256 {
            simulate(&mut counts);
        }
        let p2: u64 = counts.iter().sum();

        (p1.to_string(), p2.to_string())
    }
}

/// Preforms one simulation tick on the buffer.
fn simulate(counts: &mut [u64; 9]) {
    let mut new_counts = [0; 9];
    // Each fish at 0 creates a new fish at 8 and sets their count to 6
    new_counts[8] = counts[0];
    new_counts[6] = counts[0];
    // Every other fish decreased their count by 1;
    new_counts[0] += counts[1];
    new_counts[1] += counts[2];
    new_counts[2] += counts[3];
    new_counts[3] += counts[4];
    new_counts[4] += counts[5];
    new_counts[5] += counts[6];
    new_counts[6] += counts[7];
    new_counts[7] += counts[8];

    // Update the counts.
    *counts = new_counts;
}
