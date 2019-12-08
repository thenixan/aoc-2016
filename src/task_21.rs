use std::fs::File;
use std::io::{BufRead, BufReader};

enum Commands {
    SwapPosition { left: usize, right: usize },
    SwapLetter { left: char, right: char },
    RotateLeft(usize),
    RotateRight(usize),
    RotateWithPosition(char),
    ReversePositions { from: usize, to: usize },
    MoveToPosition { from: usize, to: usize },
}

pub fn run() {
    let input = File::open("input/task_21").unwrap();
    let input = BufReader::new(input);
}

pub fn run_e() {}
