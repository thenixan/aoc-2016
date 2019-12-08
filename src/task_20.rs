use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::Add;
use std::str::FromStr;

#[derive(Debug)]
struct IpRange {
    from: u32,
    to: u32,
}
impl IpRange {
    fn contains(&self, target: u32) -> bool {
        self.from <= target && target <= self.to
    }
}

#[derive(Debug)]
struct IpRangeFilter {
    content: Vec<IpRange>,
}

impl IpRangeFilter {
    fn not_contains(&self, other: u32) -> bool {
        self.content.iter().all(|range| !range.contains(other))
    }

    fn fold(&mut self) {
        let len = self.content.len();
        let mut i = 0;
        while i < self.content.len() - 1 {
            let mut j = i + 1;
            while j < self.content.len() {
                let left = &self.content[i];
                let right = &self.content[j];
                if left.contains(right.from) && !left.contains(right.to) {
                    self.content[i].to = self.content[j].to;
                    self.content.remove(j);
                } else if right.contains(left.from) && !right.contains(left.to) {
                    self.content[i].from = self.content[j].from;
                    self.content.remove(j);
                } else if !left.contains(right.from) && left.contains(right.to) {
                    self.content[i].from = self.content[j].from;
                    self.content.remove(j);
                } else if !right.contains(left.from) && right.contains(left.to) {
                    self.content[i].to = self.content[j].to;
                    self.content.remove(j);
                } else if left.contains(right.from) && left.contains(right.to) {
                    self.content.remove(j);
                } else if right.contains(left.from) && right.contains(left.to) {
                    self.content[i].from = self.content[j].from;
                    self.content[i].to = self.content[j].to;
                    self.content.remove(j);
                } else {
                    j += 1;
                }
            }
            i += 1;
        }
        if len != self.content.len() {
            self.fold();
        }
    }

    fn append(&mut self, other: IpRange) {
        let mut contains = false;
        for i in 0..self.content.len() {
            if self.content[i].contains(other.from) && self.content[i].contains(other.to) {
                contains = true;
            }
        }
        if !contains {
            self.content.push(other);
        }
        self.fold();
    }
}

pub fn run() {
    let input = File::open("input/task_20").unwrap();
    let input = BufReader::new(input);

    let filter = input
        .lines()
        .filter_map(|l| l.ok())
        .map(|l| {
            l.split("-")
                .filter_map(|s| s.parse::<u32>().ok())
                .collect::<Vec<u32>>()
        })
        .map(|l| IpRange {
            from: l[0],
            to: l[1],
        })
        .fold(IpRangeFilter { content: vec![] }, |mut acc, i| {
            acc.append(i);
            acc
        });

    let result = (0..std::u32::MAX)
        .find(|i| filter.not_contains(*i))
        .unwrap();

    println!("Result: {}", result);
}

pub fn run_e() {
    let input = File::open("input/task_20").unwrap();
    let input = BufReader::new(input);

    let filter = input
        .lines()
        .filter_map(|l| l.ok())
        .map(|l| {
            l.split("-")
                .filter_map(|s| s.parse::<u32>().ok())
                .collect::<Vec<u32>>()
        })
        .map(|l| IpRange {
            from: l[0],
            to: l[1],
        })
        .fold(IpRangeFilter { content: vec![] }, |mut acc, i| {
            acc.append(i);
            acc
        });

    let result = (0..std::u32::MAX)
        .filter(|i| filter.not_contains(*i))
        .count();

    println!("Result: {}", result);
}
