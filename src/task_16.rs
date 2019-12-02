use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::{BufReader, Read};
use std::ops::AddAssign;
use std::ops::{Deref, DerefMut};
use std::str::FromStr;

#[derive(Clone)]
struct Data(Vec<u8>);

impl Deref for Data {
    type Target = Vec<u8>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Display for Data {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = self.iter().map(|c| format!("{}", c)).collect::<String>();
        write!(f, "{}", s)
    }
}

impl DerefMut for Data {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl FromStr for Data {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut data = Data(vec![]);
        for c in s.chars() {
            data.0.push(c.to_digit(10).unwrap() as u8);
        }
        Ok(data)
    }
}

impl Data {
    fn reverse(&mut self) {
        self.0.reverse();
    }

    fn checksum(&mut self) {
        let mut i = 0;
        while i < self.len() {
            if self[i] == self[i + 1] {
                self[i] = 1;
            } else {
                self[i] = 0;
            }
            self.remove(i + 1);
            i += 1;
        }
    }

    fn invert(&mut self) {
        for i in 0..self.0.len() {
            if self.0[i] == 0 {
                self.0[i] = 1;
            } else {
                self.0[i] = 0;
            }
        }
    }

    fn increase(&mut self) {
        let mut right = self.clone();
        right.reverse();
        right.invert();
        *self += right;
    }
}

impl AddAssign for Data {
    fn add_assign(&mut self, other: Self) {
        self.0.push(0);
        self.0.append(&mut other.0.clone());
    }
}

pub fn run() {
    let mut input = File::open("input/task_16").unwrap();

    let mut buffer = String::new();

    input.read_to_string(&mut buffer).unwrap();

    let mut data = buffer.parse::<Data>().unwrap();

    let target_length = 272;
    while data.len() < target_length {
        data.increase();
    }

    data.truncate(target_length);

    while data.len() % 2 == 0 {
        data.checksum()
    }

    println!("Result: {}", data);
}

pub fn run_e() {
    let input = File::open("input/task_16").unwrap();
    let _input = BufReader::new(input);
}
