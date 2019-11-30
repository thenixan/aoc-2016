use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::FromIterator;

#[derive(Eq, PartialEq, Hash, Debug)]
enum Unit {
    Generator(char),
    Microchip(char),
}

struct FactoryLayout {
    floors: u8,
    elevator: u8,
    units: HashMap<Unit, u8>,
}

impl Display for Unit {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Unit::Generator(c) => write!(f, "{}G", c.to_uppercase()),
            Unit::Microchip(c) => write!(f, "{}M", c.to_uppercase()),
        }
    }
}

impl Debug for FactoryLayout {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let count = self.units.len();
        let result = (0..self.floors)
            .rev()
            .map(|floor| (floor, format!("F{}", floor + 1)))
            .map(|(floor, s)| {
                if self.elevator == floor {
                    (floor, format!("{} {}  ", s, "E"))
                } else {
                    (floor, format!("{} {}  ", s, "."))
                }
            })
            .map(|(floor, s)| {
                let units = self
                    .units
                    .iter()
                    .map(|(unit, unit_floor)| {
                        if &floor == unit_floor {
                            format!("{} ", unit.to_string())
                        } else {
                            ".  ".to_string()
                        }
                    })
                    .collect::<String>();
                format!("{}{}", s, units)
            })
            .map(|s| format!("{}\n", s))
            .collect::<String>();
        write!(f, "{}", result)
    }
}

impl FromIterator<String> for FactoryLayout {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        let (floors, units) = iter.into_iter().enumerate().fold(
            (0, HashMap::new()),
            |(floor_acc, mut units_acc), (floor, l)| {
                if l.ends_with(" floor contains nothing relevant.") {
                    // do nothing
                } else {
                    let parts = l
                        .split(" a ")
                        .skip(1)
                        .map(|l| l.split(" ").take(2).collect::<Vec<&str>>())
                        .map(|l| {
                            if l[1].starts_with("generator") {
                                Unit::Generator(l[0].chars().nth(0).unwrap())
                            } else {
                                Unit::Microchip(l[0].chars().nth(0).unwrap())
                            }
                        })
                        .collect::<Vec<Unit>>();
                    for p in parts {
                        units_acc.insert(p, floor as u8);
                    }
                }
                (u8::max(floor_acc, floor as u8), units_acc)
            },
        );
        FactoryLayout {
            floors: floors + 1,
            elevator: 0,
            units,
        }
    }
}

pub fn run() {
    let input = File::open("input/task_11").unwrap();
    let input = BufReader::new(input);

    let factory = input
        .lines()
        .filter_map(|l| l.ok())
        .collect::<FactoryLayout>();

    println!("{:?}", factory);
    println!("Result: {}", 0);
}

pub fn run_e() {
    let input = File::open("input/task_10").unwrap();
    let _input = BufReader::new(input);
}
