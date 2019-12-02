use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::FromIterator;
use std::str::FromStr;

#[derive(Debug)]
struct Disk {
    positions: i32,
    offset: i32,
}

impl Disk {
    fn is_opened(&self, time: i32) -> bool {
        let result = (self.offset + time) % self.positions == 0;
        result
    }
}

impl FromStr for Disk {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s[0..s.len() - 1].split(" ").collect::<Vec<&str>>();
        let positions = parts[3].parse::<i32>().unwrap();
        let offset = parts[11].parse::<i32>().unwrap();
        Ok(Disk { positions, offset })
    }
}

#[derive(Debug)]
struct Construction {
    disks: Vec<Disk>,
}

impl Construction {
    fn can_fall(&self, time: i32) -> bool {
        self.disks
            .iter()
            .enumerate()
            .map(|(i, d)| (i as i32 + 1, d))
            .all(|(pos, disk)| disk.is_opened(time + pos))
    }
}

impl FromIterator<Disk> for Construction {
    fn from_iter<I: IntoIterator<Item = Disk>>(iter: I) -> Self {
        let mut result = Construction { disks: vec![] };
        for d in iter {
            result.disks.push(d);
        }
        result
    }
}

pub fn run() {
    let input = File::open("input/task_15").unwrap();
    let input = BufReader::new(input);

    let disks = input
        .lines()
        .filter_map(|l| l.ok())
        .filter_map(|l| l.parse::<Disk>().ok())
        .collect::<Construction>();
    let result = (0..).find(|&time| disks.can_fall(time)).unwrap();
    println!("Result: {}", result);
}

pub fn run_e() {
    let input = File::open("input/task_15").unwrap();
    let input = BufReader::new(input);

    let mut disks = input
        .lines()
        .filter_map(|l| l.ok())
        .filter_map(|l| l.parse::<Disk>().ok())
        .collect::<Construction>();
    disks.disks.push(Disk {
        offset: 0,
        positions: 11,
    });
    let result = (0..).find(|&time| disks.can_fall(time)).unwrap();
    println!("Result: {}", result);
}
