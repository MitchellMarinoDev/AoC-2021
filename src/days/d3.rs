use crate::days::Day;

pub struct Day3;

const INPUT: &str = "inputs/d3.txt";
const BIT_WIDTH: u32 = 12;

impl Day for Day3 {
    fn solve() -> (String, String) {
        let raw = std::fs::read_to_string(INPUT).expect("input file");
        let report: Vec<_> = raw.lines()
            .map(|l| u32::from_str_radix(l, 2).unwrap()).collect();

        // let report: Vec<u32> = vec![
        //     u32::from_str_radix("00100", 2).unwrap(),
        //     u32::from_str_radix("11110", 2).unwrap(),
        //     u32::from_str_radix("10110", 2).unwrap(),
        //     u32::from_str_radix("10111", 2).unwrap(),
        //     u32::from_str_radix("10101", 2).unwrap(),
        //     u32::from_str_radix("01111", 2).unwrap(),
        //     u32::from_str_radix("00111", 2).unwrap(),
        //     u32::from_str_radix("11100", 2).unwrap(),
        //     u32::from_str_radix("10000", 2).unwrap(),
        //     u32::from_str_radix("11001", 2).unwrap(),
        //     u32::from_str_radix("00010", 2).unwrap(),
        //     u32::from_str_radix("01010", 2).unwrap(),
        // ];

        // Solve
        let mut values = [0u32; 12];
        for n in report.iter() {
            for i in 0..(BIT_WIDTH as usize) {
                if n & (1 << i) != 0 { // select bit i
                    values[i] += 1;
                }
            }
        }

        // The threshold for the most common value to be 1
        // This is 50 % of the # of lines
        let threshold = (report.len() / 2) as _;
        let mut gamma: u32 = 0;
        // Construct gamma
        for i in 0..(BIT_WIDTH as usize) {
            if values[i] >= threshold { // select bit i
                gamma |= 1 << i
            }
        }

        let mut epsilon = !gamma; // bitwise not, but only for 12 bits
        epsilon &= (!0) % (1 << BIT_WIDTH + 1);
        let p1 = gamma * epsilon;

        // Part 2
        // o2
        let mut i = 0;
        let mut o2 = report.clone();
        while o2.len() > 1 {
            // find most common bit
            let count: u32 = o2.iter().map(|n| (n & (1 << i) != 0) as u32).sum();
            let bit_criteria = count >= (o2.len()/2) as _;
            o2 = o2.into_iter().filter(|&n| (n & (1 << i) != 0) == bit_criteria).collect();

            i = (i + 1) % BIT_WIDTH;
        }

        // co2
        let mut i = 0;
        let mut co2 = report.clone();
        while co2.len() > 1 {
            // find most common bit
            let count: u32 = co2.iter().map(|n| (n & (1 << i) != 0) as u32).sum();
            let bit_criteria = count <= (co2.len()/2) as _;
            co2 = co2.into_iter().filter(|&n| (n & (1 << i) != 0) == bit_criteria).collect();

            i = (i + 1) % BIT_WIDTH;
        }

        let pt2 = o2[0] * co2[0];
        (p1.to_string(), pt2.to_string())
    }
}