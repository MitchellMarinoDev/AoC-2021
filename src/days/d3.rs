use crate::days::Day;

pub struct Day3;

const INPUT: &str = "inputs/d3.txt";
const BIT_WIDTH: u32 = 12;

impl Day for Day3 {
    fn solve() -> (String, String) {
        let raw = std::fs::read_to_string(INPUT).expect(&*format!("Check input file {}", INPUT));
        let report: Vec<_> = raw
            .lines()
            .map(|l| u32::from_str_radix(l, 2).unwrap())
            .collect();

        // Solve
        let mut bit_counts = [0u32; 12];
        for n in report.iter() {
            for i in 0..(BIT_WIDTH as usize) {
                if n & (1 << i) != 0 {
                    // select bit i
                    bit_counts[i] += 1;
                }
            }
        }

        // The threshold for the most common value to be 1
        // This is 50 % of the # of lines
        let threshold = (report.len() / 2) as _;
        let mut gamma: u32 = 0;
        // Construct gamma
        for i in 0..(BIT_WIDTH as usize) {
            if bit_counts[i] >= threshold {
                // select bit i
                gamma |= 1 << i;
            }
        }

        println!("{:#014b}", gamma);

        let mut epsilon = !gamma; // bitwise not, but only for BIT_WIDTH bits
        epsilon &= (!0) % (1 << BIT_WIDTH);
        let p1 = gamma * epsilon;

        // Part 2
        // o2
        let mut mask = 1 << BIT_WIDTH - 1;
        let mut o2 = report.clone();
        while o2.len() > 1 {
            // find most common bit
            let count: u32 = o2.iter().map(|n| (n & mask != 0) as u32).sum();
            let bit_criteria = count as f32 / o2.len() as f32 >= 0.5;
            o2 = o2
                .into_iter()
                .filter(|&n| (n & mask != 0) == bit_criteria)
                .collect();

            mask >>= 1;
        }

        // co2
        let mut mask = 1 << BIT_WIDTH - 1;
        let mut co2 = report.clone();
        while co2.len() > 1 {
            // find most common bit
            let count: u32 = co2.iter().map(|n| (n & mask != 0) as u32).sum();
            let bit_criteria = (count as f32 / co2.len() as f32) < 0.5;
            co2 = co2
                .into_iter()
                .filter(|&n| (n & mask != 0) == bit_criteria)
                .collect();

            mask >>= 1;
        }

        println!("{}, {}", o2[0], co2[0]);
        let pt2 = o2[0] * co2[0];
        (p1.to_string(), pt2.to_string())
    }
}
