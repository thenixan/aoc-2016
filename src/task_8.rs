use std::fmt::Debug;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

struct Board {
    content: Vec<Vec<bool>>,
}

impl Board {
    fn new(wide: usize, tall: usize) -> Self {
        Board {
            content: vec![vec![false; wide]; tall],
        }
    }

    fn apply_command(&mut self, command: Command) {
        match command {
            Command::DrawRect { wide: w, tall: t } => {
                for i in 0..t {
                    for j in 0..w {
                        self.content[i][j] = true;
                    }
                }
            }
            Command::RotateCol { col: c, times: t } => {
                let size = self.content.len();
                for _ in 0..t {
                    for i in 0..size - 1 {
                        let b = self.content[size - i - 2][c];
                        self.content[size - i - 2][c] = self.content[size - i - 1][c];
                        self.content[size - i - 1][c] = b;
                    }
                }
            }
            Command::RotateRow { row: r, times: t } => {
                let size = self.content[0].len();
                for _ in 0..t {
                    for i in 0..size - 1 {
                        let b = self.content[r][size - i - 2];
                        self.content[r][size - i - 2] = self.content[r][size - i - 1];
                        self.content[r][size - i - 1] = b;
                    }
                }
            }
        }
    }

    fn count_enabled(&self) -> usize {
        self.content
            .iter()
            .flatten()
            .filter(|c| **c)
            .collect::<Vec<_>>()
            .len()
    }
}

impl Debug for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for r in &self.content {
            for c in r {
                write!(f, "{}", if *c { '#' } else { '.' }).unwrap();
            }
            writeln!(f).unwrap();
        }
        Ok(())
    }
}

enum Command {
    DrawRect { wide: usize, tall: usize },
    RotateRow { row: usize, times: usize },
    RotateCol { col: usize, times: usize },
}

impl FromStr for Command {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split(" ").collect::<Vec<_>>();
        if parts[0] == "rect" {
            let size = parts[1]
                .split("x")
                .filter_map(|s| s.parse::<usize>().ok())
                .take(2)
                .collect::<Vec<_>>();
            Ok(Command::DrawRect {
                wide: size[0],
                tall: size[1],
            })
        } else if parts[0] == "rotate" && parts[1] == "row" {
            let row_num = parts[2].split("=").collect::<Vec<&str>>()[1]
                .parse::<usize>()
                .unwrap();
            let times = parts[4].parse::<usize>().unwrap();
            Ok(Command::RotateRow {
                row: row_num,
                times: times,
            })
        } else if parts[0] == "rotate" && parts[1] == "column" {
            let col_num = parts[2].split("=").collect::<Vec<&str>>()[1]
                .parse::<usize>()
                .unwrap();
            let times = parts[4].parse::<usize>().unwrap();
            Ok(Command::RotateCol {
                col: col_num,
                times: times,
            })
        } else {
            Err(())
        }
    }
}

pub fn run() {
    let input = File::open("input/task_8").unwrap();
    let input = BufReader::new(input);

    let board = input
        .lines()
        .filter_map(|l| l.ok())
        .filter_map(|l| l.parse::<Command>().ok())
        .fold(Board::new(50, 6), |mut board, c| {
            board.apply_command(c);
            board
        });
    println!("{:?}", board.count_enabled());
}

pub fn run_e() {
    let input = File::open("input/task_8").unwrap();
    let input = BufReader::new(input);

    let board = input
        .lines()
        .filter_map(|l| l.ok())
        .filter_map(|l| l.parse::<Command>().ok())
        .fold(Board::new(50, 6), |mut board, c| {
            board.apply_command(c);
            board
        });
    println!("{:?}", board);
}
