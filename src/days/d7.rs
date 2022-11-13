use crate::days::Day;

pub struct Day7;

const INPUT: &str = "inputs/d7.txt";

impl Day for Day7 {
    fn solve() -> (String, String) {
        let raw = std::fs::read_to_string(INPUT).expect(&*format!("Check input file {}", INPUT));
        let raw = "16,1,2,0,4,2,7,1,2,14";
        let mut pos: Vec<i32> = raw.trim().split(',').map(|s| s.parse().unwrap()).collect();
        pos.sort();
        let median = pos[(pos.len() - 1) / 2];
        let p1: i32 = pos.iter().map(|p| (p - median).abs()).sum();

        let avg = pos.iter().sum::<i32>() as f32 / pos.len() as f32;
        let target = (avg / 2.0) as i32;
        println!("target {target}");
        let p2 = pos.iter().map(|x| cost_p2(*x, target)).sum::<i32>();

        (p1.to_string(), p2.to_string())
    }
}

fn cost_p2(x: i32, y: i32) -> i32 {
    let d = (x - y).abs();
    (d + d * d) / 2
}
