use std::collections::{LinkedList, VecDeque};
use std::fs::File;
use std::io::Read;
use std::str::FromStr;

struct Game(VecDeque<usize>);

impl FromStr for Game {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let count = s.parse::<usize>().unwrap();
        Ok(Game((1..=count).collect()))
    }
}

impl Game {
    fn play(&mut self) -> usize {
        while self.0.len() > 1 {
            let first = self.0.pop_front().unwrap();
            self.0.pop_front();
            self.0.push_back(first);
        }
        self.0[0]
    }

    fn play_b(&mut self) -> usize {
        while self.0.len() > 1 {
            if self.0.len() % 1000 == 0 {
                println!("L: {}", self.0.len());
            }
            let length = self.0.len() / 2;
            let first = self.0.pop_front().unwrap();
            self.0.remove(length);
            self.0.push_back(first);
        }
        self.0[0]
    }
}

pub fn run() {
    let mut input = File::open("input/task_19").unwrap();
    let mut buffer = String::new();

    input.read_to_string(&mut buffer).unwrap();

    let mut game = buffer.parse::<Game>().unwrap();

    let result = game.play();
    println!("Result: {}", result);
}

pub fn run_e() {
    let mut input = File::open("input/task_19").unwrap();
    let mut buffer = String::new();

    input.read_to_string(&mut buffer).unwrap();

    let mut game = buffer.parse::<Game>().unwrap();

    let result = game.play_b();
    println!("Result: {}", result);
}
