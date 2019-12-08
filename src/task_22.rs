use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::FromIterator;
use std::str::FromStr;

struct Node {
    x: usize,
    y: usize,
    used: usize,
    available: usize,
}

impl Node {
    fn total(&self) -> usize {
        self.used + self.available
    }
}

impl FromStr for Node {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let pattern =
            Regex::new(r"^/dev/grid/node-x(\d+)-y(\d+)\s+(\d+)T\s+(\d+)T\s+(\d+)T\s+\d+%$")
                .unwrap();
        let caps = pattern.captures(s).unwrap();
        let x = caps[1].parse::<usize>().unwrap();
        let y = caps[2].parse::<usize>().unwrap();
        let u = caps[4].parse::<usize>().unwrap();
        let a = caps[5].parse::<usize>().unwrap();
        Ok(Node {
            x,
            y,
            used: u,
            available: a,
        })
    }
}

pub fn run() {
    let input = File::open("input/task_22").unwrap();
    let input = BufReader::new(input);

    let nodes = input
        .lines()
        .skip(2)
        .filter_map(|l| l.ok())
        .filter_map(|l| l.parse::<Node>().ok())
        .collect::<Vec<_>>();

    let mut result = 0;
    for i in 0..nodes.len() {
        for j in 0..nodes.len() {
            if i != j && nodes[i].used != 0 && nodes[i].used <= nodes[j].available {
                result += 1;
            }
        }
    }

    println!("Result: {}", result);
}

pub fn run_e() {}
