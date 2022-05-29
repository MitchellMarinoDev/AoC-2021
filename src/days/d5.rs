use crate::days::Day;

pub struct Day5;

const INPUT: &str = "inputs/d5.txt";

impl Day for Day5 {
    fn solve() -> (String, String) {
        let raw = std::fs::read_to_string(INPUT).expect(&*format!("Check input file {}", INPUT));
        let mut vents = vec![];
        for l in raw.lines() {
            let mut split = l.split("->");
            let mut pos1 = split.next().unwrap().trim().split(",");
            let mut pos2 = split.next().unwrap().trim().split(",");
            let start: (u32, u32) = (pos1.next().unwrap().parse().unwrap(), pos1.next().unwrap().parse().unwrap());
            let end: (u32, u32) =   (pos2.next().unwrap().parse().unwrap(), pos2.next().unwrap().parse().unwrap());
            vents.push((start, end));
        }

        // Solve!
        // let mut field = HashMap::new();
        // for (start, end) in vents {
        //     if start.0 == end.0 {
        //         for i in start.1..end.1 {
        //
        //         }
        //     } else if start.1 == end.1 {
        //
        //     }
        // }


        ("".to_string(), "".to_string())
    }
}