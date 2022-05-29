use crate::days::Day;

pub struct Day4;

const INPUT: &str = "inputs/d4.txt";

impl Day for Day4 {
    fn solve() -> (String, String) {
        let raw = std::fs::read_to_string(INPUT).expect(&*format!("Check input file {}", INPUT));
        let draw: Vec<u32> = raw.split("\n").next().unwrap().split(',').map(|s| s.parse().unwrap()).collect();
        let mut boards = vec![];
        for board in raw.split("\n\n").skip(1) {
            let mut nums = [[0; 5]; 5];
            for (row_i, row) in board.lines().enumerate() {
                for (n_i, n) in row.split(" ").filter(|e| !e.is_empty()).enumerate() {
                    nums[row_i][n_i] = n.parse::<u32>().unwrap();
                }
            }
            let board = Board {
                nums,
                checked: [[false; 5]; 5],
            };
            boards.push(board);
        }

        // Solve
        let mut p1 = 0;
        'outer:
        for d in draw.iter() {
            for b in boards.iter_mut() {
                b.check(*d);
                if b.win() {
                    p1 = b.score(*d);
                    break 'outer;
                }
            }
        }

        // pt2
        let mut p2 = 0;
        for d in draw.iter() {
            let mut remove = vec![];
            for (i, b) in boards.iter_mut().enumerate() {
                b.check(*d);
                if b.win() {
                    remove.push(i);
                }
            }
            for i in remove.into_iter().rev() {
                let b = boards.remove(i);
                if boards.len() == 0 {
                    p2 = b.score(*d);
                }
            }
        }

        (p1.to_string(), p2.to_string())
    }
}

#[derive(Clone, Eq, PartialEq, Debug)]
struct Board {
    nums: [[u32; 5]; 5],
    checked: [[bool; 5]; 5],
}

impl Board {
    /// Checks a number if it is on the board.
    pub fn check(&mut self, n: u32) {
        for r in 0..5 {
            for c in 0..5 {
                if self.nums[r][c] == n {
                    self.checked[r][c] = true;
                }
            }
        }
    }

    /// Checks if the board is in the win state.
    pub fn win(&self) -> bool {
        let mut row_wins = [true; 5];
        let mut col_wins = [true; 5];

        for r in 0..5 {
            for c in 0..5 {
                if !self.checked[r][c] {
                    row_wins[r] = false;
                    col_wins[c] = false;
                }
            }
        }

        row_wins.iter().chain(col_wins.iter()).any(|w| *w)
    }

    /// Gets the score of the card
    pub fn score(&self, last: u32) -> u32 {
        let mut sum = 0;
        for r in 0..5 {
            for c in 0..5 {
                if !self.checked[r][c] {
                    sum += self.nums[r][c];
                }
            }
        }
        sum * last
    }
}
