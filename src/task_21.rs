use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::FromIterator;
use std::str::FromStr;

struct CommandSequence {
    data: Vec<Command>,
}

impl FromIterator<Command> for CommandSequence {
    fn from_iter<I: IntoIterator<Item = Command>>(iter: I) -> Self {
        CommandSequence {
            data: iter.into_iter().collect(),
        }
    }
}

impl CommandSequence {
    fn apply(&self, s: &str) -> String {
        let mut chars = s.chars().collect::<Vec<char>>();
        for d in &self.data {
            d.apply(&mut chars);
        }
        chars.iter().collect()
    }
}

enum Command {
    SwapPosition { left: usize, right: usize },
    SwapLetter { left: char, right: char },
    RotateLeft(usize),
    RotateRight(usize),
    RotateWithPosition(char),
    ReversePositions { from: usize, to: usize },
    MoveToPosition { from: usize, to: usize },
}

impl Command {
    fn apply(&self, c: &mut Vec<char>) {
        match self {
            Command::SwapPosition { left, right } => {
                let b = c[*right];
                c[*right] = c[*left];
                c[*left] = b;
            }
            Command::SwapLetter { left, right } => {
                for i in 0..c.len() {
                    if c[i] == *left {
                        c[i] = *right;
                    } else if c[i] == *right {
                        c[i] = *left;
                    }
                }
            }
            Command::RotateLeft(s) => {
                let s = s % c.len();
                for _ in 0..s {
                    let a = c[0];
                    c.remove(0);
                    c.push(a);
                }
            }
            Command::RotateRight(s) => {
                let s = s % c.len();
                for _ in 0..s {
                    let a = c.pop().unwrap();
                    c.insert(0, a);
                }
            }
            Command::RotateWithPosition(t) => {
                let mut pos = c
                    .iter()
                    .enumerate()
                    .find_map(|i| if i.1 == t { Some(i.0) } else { None })
                    .unwrap();
                if pos >= 4 {
                    pos += 1;
                }
                pos += 1;
                let s = pos % c.len();
                for _ in 0..s {
                    let a = c.pop().unwrap();
                    c.insert(0, a);
                }
            }
            Command::ReversePositions { from, to } => {
                let mut f = *from;
                let mut t = *to;
                while f <= t {
                    let a = c[f];
                    c[f] = c[t];
                    c[t] = a;
                    f += 1;
                    t -= 1;
                }
            }
            Command::MoveToPosition { from, to } => {
                let a = c.remove(*from);
                c.insert(*to, a);
            }
        }
    }
}

impl FromStr for Command {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.split(" ").collect::<Vec<&str>>();
        match (s[0], s[1]) {
            ("swap", "position") => Ok(Command::SwapPosition {
                left: s[2].parse::<usize>().unwrap(),
                right: s[5].parse::<usize>().unwrap(),
            }),
            ("swap", "letter") => Ok(Command::SwapLetter {
                left: s[2].parse::<char>().unwrap(),
                right: s[5].parse::<char>().unwrap(),
            }),
            ("rotate", "left") => Ok(Command::RotateLeft(s[2].parse::<usize>().unwrap())),
            ("rotate", "right") => Ok(Command::RotateRight(s[2].parse::<usize>().unwrap())),
            ("rotate", "based") => Ok(Command::RotateWithPosition(s[6].parse::<char>().unwrap())),
            ("reverse", "positions") => Ok(Command::ReversePositions {
                from: s[2].parse::<usize>().unwrap(),
                to: s[4].parse::<usize>().unwrap(),
            }),
            ("move", "position") => Ok(Command::MoveToPosition {
                from: s[2].parse::<usize>().unwrap(),
                to: s[5].parse::<usize>().unwrap(),
            }),
            _ => Err(()),
        }
    }
}

pub fn run() {
    let input = File::open("input/task_21").unwrap();
    let input = BufReader::new(input);

    let seq = input
        .lines()
        .filter_map(|l| l.ok())
        .filter_map(|l| l.parse::<Command>().ok())
        .collect::<CommandSequence>();

    let result = seq.apply("abcdefgh");
    println!("Result: {}", result);
}

pub fn run_e() {}
