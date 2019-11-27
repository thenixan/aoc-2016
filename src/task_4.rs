use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

struct Room {
    name: String,
    number: u16,
}

impl Room {
    fn new(name: String, number: u16) -> Self {
        Room { name, number }
    }

    fn checksum(&self) -> String {
        let mut counts = self
            .name
            .chars()
            .filter(|c| *c != '-')
            .fold(HashMap::new(), |mut s, i| {
                *s.entry(i).or_insert(0u8) += 1;
                s
            })
            .into_iter()
            .collect::<Vec<(char, u8)>>();
        counts.sort_by(|a, b| match a.1.cmp(&b.1) {
            Ordering::Equal => a.0.cmp(&b.0).reverse(),
            o => o,
        });
        counts.reverse();
        counts.into_iter().take(5).map(|(c, _)| c).collect()
    }

    fn decrypt(&self) -> String {
        let alphabet_lower: &str = "abcdefghijklmnopqrstuvwxyz";
        let mut result: String = String::new();

        let shift = self.number % 26;

        for c in self.name.chars() {
            if c == '-' {
                result.push(' ');
                continue;
            }

            if shift >= 26 {
                panic!("Please specify a smaller shift.");
            }

            match alphabet_lower.chars().position(|b| c == b) {
                Some(x) => {
                    let idx: usize = shift as usize + x;

                    let new_index = if (idx as u32) >= 26u32 {
                        idx - 26usize
                    } else {
                        idx
                    };

                    match alphabet_lower.chars().nth(new_index) {
                        Some(x) => {
                            result.push(x);
                        }
                        None => {
                            panic!("No element could be found at index {}", new_index);
                        }
                    };
                }
                None => {
                    panic!("'{}' is not a valid element in the ASCII alphabet", c);
                }
            };
        }
        return result;
    }
}

pub fn run() {
    let input = File::open("input/task_4").unwrap();
    let input = BufReader::new(input);

    let rooms: Vec<Room> = input
        .lines()
        .filter_map(|l| l.ok())
        .filter_map(|s| {
            let s: Vec<&str> = s.split("[").collect();
            let cs_l = s[1].len();
            let checksum = &s[1][..cs_l - 1];
            let mut name_and_number: Vec<&str> = s[0].split("-").collect();
            let number = name_and_number.pop().unwrap().parse::<u16>().unwrap();
            let name = name_and_number.join("-");

            let room = Room::new(name, number);
            if room.checksum() == checksum {
                Some(room)
            } else {
                None
            }
        })
        .collect();

    let result = rooms.iter().map(|r| r.number as u32).sum::<u32>();

    println!("Result: {}", result);
}

pub fn run_e() {
    let input = File::open("input/task_4").unwrap();
    let input = BufReader::new(input);

    let rooms: Vec<Room> = input
        .lines()
        .filter_map(|l| l.ok())
        .filter_map(|s| {
            let s: Vec<&str> = s.split("[").collect();
            let cs_l = s[1].len();
            let checksum = &s[1][..cs_l - 1];
            let mut name_and_number: Vec<&str> = s[0].split("-").collect();
            let number = name_and_number.pop().unwrap().parse::<u16>().unwrap();
            let name = name_and_number.join("-");

            let room = Room::new(name, number);
            if room.checksum() == checksum {
                Some(room)
            } else {
                None
            }
        })
        .collect();

    let result = rooms
        .iter()
        .filter_map(|r| {
            if r.decrypt() == "northpole object storage" {
                Some(r.number)
            } else {
                None
            }
        })
        .next()
        .unwrap();

    println!("Result: {}", result);
}
