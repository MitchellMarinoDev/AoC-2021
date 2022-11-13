use crate::days::{Day, DAYS};
use std::time::Instant;

mod days;

fn main() {
    for (i, day) in DAYS.iter().enumerate() {
        let start = Instant::now();
        let s = day();
        println!(
            "d{}: \"{}\",\t\"{}\"\t{}ns",
            i + 1,
            s.0,
            s.1,
            start.elapsed().as_nanos()
        );
    }

    let i = 12;
}
