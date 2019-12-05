use std::collections::VecDeque;
use std::fs::File;
use std::io::Read;
use std::ops::Index;
use std::str::FromStr;

struct Game {
    first_half: VecDeque<usize>,
    second_half: VecDeque<usize>,
}

impl Index<usize> for Game {
    type Output = usize;
    fn index(&self, i: usize) -> &Self::Output {
        if i < self.first_half.len() {
            &self.first_half[i]
        } else {
            &self.second_half[i - self.first_half.len()]
        }
    }
}

impl FromStr for Game {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let count = s.parse::<usize>().unwrap();
        let mid_point = count / 2;
        Ok(Game {
            first_half: (1..=mid_point).collect(),
            second_half: (mid_point + 1..=count).collect(),
        })
    }
}

impl Game {
    fn pop_front(&mut self) -> Option<usize> {
        if self.first_half.is_empty() {
            self.second_half.pop_front()
        } else {
            self.first_half.pop_front()
        }
    }

    fn push_back(&mut self, value: usize) {
        self.second_half.push_back(value);
    }

    fn len(&self) -> usize {
        self.first_half.len() + self.second_half.len()
    }

    fn stabilize(&mut self) {
        while self.first_half.len() < self.second_half.len() {
            self.first_half
                .push_back(self.second_half.pop_front().unwrap())
        }
    }

    fn play(&mut self) -> usize {
        while self.len() > 1 {
            let first = self.pop_front().unwrap();
            self.pop_front();
            self.push_back(first);
        }
        self[0]
    }

    fn remove(&mut self, i: usize) -> Option<usize> {
        if i < self.first_half.len() {
            self.first_half.remove(i)
        } else {
            self.second_half.remove(i - self.first_half.len())
        }
    }

    fn play_b(&mut self) -> usize {
        while self.len() > 1 {
            let length = self.len() / 2;
            self.remove(length);
            let first = self.pop_front().unwrap();
            self.push_back(first);
            self.stabilize()
        }
        self[0]
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
