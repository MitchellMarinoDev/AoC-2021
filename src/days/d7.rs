use crate::days::Day;

pub struct Day7;

const INPUT: &str = "inputs/d7.txt";

impl Day for Day7 {
    fn solve() -> (String, String) {
        let raw = std::fs::read_to_string(INPUT).expect(&*format!("Check input file {}", INPUT));
        // let raw = "16,1,2,0,4,2,7,1,2,14";
        let mut pos: Vec<i32> = raw.trim().split(',').map(|s| s.parse().unwrap()).collect();
        pos.sort();
        let median = pos[(pos.len()-1)/2];
        let p1: i32 = pos.iter().map(|p| (p - median).abs()).sum();

        // Brute force
        let mut min = i32::MAX;
        let mut target = pos[0];
        let mut next = i32::MAX-1;
        while next < min {
            min = next;
            target += 1;
            next = pos.iter().map(|x| cost_p2(*x, target)).sum();
        }
        target -= 1;
        println!("{}", target);

        (p1.to_string(), min.to_string())
    }

}

fn cost_p2(x: i32, y: i32) -> i32 {
    let d = (x-y).abs();
    (d + d*d)/2
}
