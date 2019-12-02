use iterslide::SlideIterator;
use md5;
use std::fs::File;
use std::io::Read;
use std::iter::Iterator;

struct PasswordGenerator {
    salt: String,
}

impl PasswordGenerator {
    fn new(salt: String) -> Self {
        PasswordGenerator { salt }
    }

    fn hash(&self, step: usize) -> String {
        let digest = md5::compute(format!("{}{}", self.salt, step).as_bytes());
        format!("{:x}", digest)
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

    fn try_with_fives(&mut self, c: char) -> Option<usize> {
        let min = self
            .waits
            .iter()
            .enumerate()
            .filter(|t| t.1.c == c)
            .min_by(|l, r| l.1.pos.cmp(&r.1.pos))
            .map(|t| (t.0, t.1.pos))
            .clone();
        match min {
            Some((pos, t)) => {
                // self.waits.remove(pos);
                Some(t)
            }
            _ => None,
        }
    }

    fn trigger(&self, step: usize) -> Option<Trigger> {
        let hash = self.password.hash(step);
        let threes = hash.chars().slide(3).find_map(|c| {
            if c[0] == c[1] && c[1] == c[2] {
                Some(c[0])
            } else {
                None
            }
        });
        let fives = hash.chars().slide(5).find_map(|c| {
            if c[0] == c[1] && c[1] == c[2] && c[2] == c[3] && c[3] == c[4] {
                Some(c[0])
            } else {
                None
            }
        });
        if self.step == 22728 {
            println!("DDD {:?} {:?}", threes, fives);
        }
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
                    None
                }
                _ => None,
            };
            if self.step == 22728 {
                println!("DDD: {:?}", fives);
            }
            match fives {
                Some(position) => self.results.push(position + 1000),
                _ => (),
            };
            result = self.pop_result();
            self.step += 1;
        }
        println!("Result: {:?}", result);
        result
    }
}

pub fn run() {
    let mut input = File::open("input/task_14").unwrap();
    let mut buffer = String::new();
    input.read_to_string(&mut buffer).unwrap();

    let password = PasswordGenerator::new(buffer);
    let result = password.generator().take(64).last();
    println!("Result: {:?}", result);
}

pub fn run_e() {
    let mut input = File::open("input/task_13").unwrap();
    let mut buffer = String::new();
    input.read_to_string(&mut buffer).unwrap();
}
