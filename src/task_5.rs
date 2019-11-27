use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

struct DoorId {
    value: String,
}

impl DoorId {
    fn new(s: &str) -> Self {
        DoorId {
            value: s.to_string(),
        }
    }

    fn iter(&self) -> DoorIdIterator {
        DoorIdIterator {
            door_id: self,
            step: 0,
        }
    }
}

impl FromStr for DoorId {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(DoorId {
            value: s.to_string(),
        })
    }
}

struct DoorIdIterator<'a> {
    door_id: &'a DoorId,
    step: usize,
}

impl<'a> Iterator for DoorIdIterator<'a> {
    type Item = String;
    fn next(&mut self) -> Option<Self::Item> {
        let digest = format!("{}{}", self.door_id.value, self.step);
        let digest = md5::compute(digest);
        self.step += 1;

        Some(format!("{:x}", digest))
    }
}

pub fn run() {
    let input = File::open("input/task_5").unwrap();
    let mut input = BufReader::new(input);
    let mut door_id = String::new();
    input.read_line(&mut door_id).unwrap();

    let door = door_id.parse::<DoorId>().unwrap();
    let result = door
        .iter()
        .filter(|hash| hash.starts_with("00000"))
        .map(|s| s.chars().nth(5).unwrap())
        .take(8)
        .collect::<String>();

    println!("Result: {}", result);
}

pub fn run_e() {
    let input = File::open("input/task_5").unwrap();
    let mut input = BufReader::new(input);
    let mut door_id = String::new();
    input.read_line(&mut door_id).unwrap();

    let door = door_id.parse::<DoorId>().unwrap();
    let mut password = HashMap::new();
    let _result = door
        .iter()
        .filter(|hash| hash.starts_with("00000"))
        .filter_map(|hash| {
            let index = hash.chars().nth(5).unwrap().to_digit(10);
            if let Some(index) = index {
                if 8 > index && !password.contains_key(&index) {
                    password.insert(index, hash.chars().nth(6).unwrap());
                    Some(index)
                } else {
                    None
                }
            } else {
                None
            }
        })
        .take(8)
        .collect::<Vec<u32>>();

    let mut password = password.into_iter().collect::<Vec<(u32, char)>>();
    password.sort_by(|a, b| a.0.cmp(&b.0));
    let password = password.iter().map(|(_, v)| v).collect::<String>();

    println!("Result: {}", password);
}
