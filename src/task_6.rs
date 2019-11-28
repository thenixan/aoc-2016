use std::fs::File;
use std::io::{BufRead, BufReader};

struct CharCounter {
    counts: Vec<usize>,
}

impl CharCounter {
    fn new() -> Self {
        let base = 'a' as usize;
        let count = 'z' as usize - base + 1;
        CharCounter {
            counts: vec![0; count],
        }
    }

    fn inc(&mut self, c: char) {
        let base = 'a' as usize;
        let pos = c as usize - base;
        self.counts[pos] += 1;
    }

    fn most_frequent(&self) -> char {
        let (max, _) = self
            .counts
            .iter()
            .enumerate()
            .max_by(|a, b| a.1.cmp(&b.1))
            .unwrap();
        ('a' as u8 + max as u8) as char
    }

    fn least_frequent(&self) -> char {
        let (min, _) = self
            .counts
            .iter()
            .enumerate()
            .filter(|(_, c)| **c != 0)
            .min_by(|a, b| a.1.cmp(&b.1))
            .unwrap();
        ('a' as u8 + min as u8) as char
    }
}

pub fn run() {
    let input = File::open("input/task_6").unwrap();
    let input = BufReader::new(input);

    let result = input
        .lines()
        .filter_map(|l| l.ok())
        .fold(vec![], |mut acc, l| {
            if acc.is_empty() {
                for _ in 0..l.len() {
                    acc.push(CharCounter::new());
                }
            }
            l.chars().enumerate().for_each(|(i, c)| {
                acc[i].inc(c);
            });
            acc
        })
        .iter()
        .map(|c| c.most_frequent())
        .collect::<String>();

    println!("Result: {}", result);
}

pub fn run_e() {
    let input = File::open("input/task_6").unwrap();
    let input = BufReader::new(input);

    let result = input
        .lines()
        .filter_map(|l| l.ok())
        .fold(vec![], |mut acc, l| {
            if acc.is_empty() {
                for _ in 0..l.len() {
                    acc.push(CharCounter::new());
                }
            }
            l.chars().enumerate().for_each(|(i, c)| {
                acc[i].inc(c);
            });
            acc
        })
        .iter()
        .map(|c| c.least_frequent())
        .collect::<String>();

    println!("Result: {}", result);
}
