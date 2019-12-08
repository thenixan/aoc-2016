use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

struct CommandSequence {
    data: Vec<Command>,
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
}

pub fn run_e() {}
