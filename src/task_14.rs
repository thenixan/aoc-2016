use iterslide::SlideIterator;
use md5;
use std::fs::File;
use std::io::Read;
use std::iter::Iterator;

struct PasswordGenerator {
    salt: String,
    times: usize,
}

impl PasswordGenerator {
    fn new(salt: String) -> Self {
        PasswordGenerator { salt, times: 0 }
    }

    fn new_iterative(salt: String, times: usize) -> Self {
        PasswordGenerator { salt, times }
    }

    fn hash(&self, step: usize) -> String {
        let mut input = format!("{}{}", self.salt, step);
        for _ in 0..=self.times {
            input = format!("{:x}", md5::compute(input.as_bytes()));
        }
        input
    }

    fn generator(&self) -> PasswordGeneratorIterator {
        PasswordGeneratorIterator::new(self)
    }
}

struct Triplet {
    c: char,
    pos: usize,
}

impl PartialEq for Triplet {
    fn eq(&self, other: &Self) -> bool {
        self.c == other.c && self.pos == other.pos
    }
}
impl Eq for Triplet {}

struct PasswordGeneratorIterator<'a> {
    password: &'a PasswordGenerator,
    step: usize,
    waits: Vec<Triplet>,
    results: Vec<usize>,
}

enum Trigger {
    Triplet(char),
    Five(char),
    Both(char, char),
}

impl<'a> PasswordGeneratorIterator<'a> {
    fn new(generator: &'a PasswordGenerator) -> Self {
        PasswordGeneratorIterator {
            password: generator,
            step: 0,
            waits: vec![],
            results: vec![],
        }
    }

    fn pop_result(&mut self) -> Option<usize> {
        let index = self.results.iter().enumerate().find_map(|v| {
            if v.1 == &self.step {
                Some(v.0)
            } else {
                None
            }
        });
        if index.is_none() {
            None
        } else {
            let result = self.results.remove(index.unwrap());
            Some(result - 1000)
        }
    }

    fn clean_up(&mut self) {
        let mut i = 0;
        while i < self.waits.len() {
            if self.waits[i].pos + 1000 < self.step {
                self.waits.remove(i);
            } else {
                i += 1;
            }
        }
    }

    fn add_triplet(&mut self, c: char) {
        self.waits.push(Triplet { c, pos: self.step });
    }

    fn try_with_fives(&mut self, c: char) -> Vec<usize> {
        let triplets = self
            .waits
            .iter()
            .enumerate()
            .filter(|t| t.1.c == c)
            .map(|t| (t.0, t.1.pos))
            .collect::<Vec<(usize, usize)>>();
        for i in 0..triplets.len() {
            self.waits.remove(triplets[i].0 - i);
        }
        triplets.into_iter().map(|t| t.1).collect()
    }

    fn trigger(&self, step: usize) -> Option<Trigger> {
        let hash = self.password.hash(step);
        let fives = hash.chars().slide(5).find_map(|c| {
            if c[0] == c[1] && c[1] == c[2] && c[2] == c[3] && c[3] == c[4] {
                Some(c[0])
            } else {
                None
            }
        });
        let threes = hash.chars().slide(3).find_map(|c| {
            if c[0] == c[1] && c[1] == c[2] {
                Some(c[0])
            } else {
                None
            }
        });
        match (threes, fives) {
            (Some(t), Some(f)) => Some(Trigger::Both(t, f)),
            (Some(t), None) => Some(Trigger::Triplet(t)),
            (None, Some(f)) => Some(Trigger::Five(f)),
            _ => None,
        }
    }
}

impl<'a> Iterator for PasswordGeneratorIterator<'a> {
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {
        let mut result = None;
        while result.is_none() {
            self.clean_up();
            let fives = match self.trigger(self.step) {
                Some(Trigger::Both(t, f)) => {
                    let r = self.try_with_fives(f);
                    self.add_triplet(t);
                    r
                }
                Some(Trigger::Five(f)) => self.try_with_fives(f),
                Some(Trigger::Triplet(t)) => {
                    self.add_triplet(t);
                    vec![]
                }
                _ => vec![],
            };
            for f in fives {
                self.results.push(f + 1000);
            }
            result = self.pop_result();
            self.step += 1;
        }
        result
    }
}

pub fn run() {
    let mut input = File::open("input/task_14").unwrap();
    let mut buffer = String::new();
    input.read_to_string(&mut buffer).unwrap();

    let password = PasswordGenerator::new(buffer);
    let result = password.generator().take(64).last().unwrap();
    println!("Result: {}", result);
}

pub fn run_e() {
    let mut input = File::open("input/task_14").unwrap();
    let mut buffer = String::new();
    input.read_to_string(&mut buffer).unwrap();

    let password = PasswordGenerator::new_iterative(buffer, 2016);
    let result = password.generator().take(64).last().unwrap();
    println!("Result: {}", result);
}
