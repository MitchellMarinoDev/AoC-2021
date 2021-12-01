use crate::days::{Day, DAYS};

mod days;

fn main() {
    for (i, day) in DAYS.iter().enumerate() {
        let s = day();
        println!("d{}: \"{}\", \"{}\"", i+1, s.0, s.1);
    }
}
