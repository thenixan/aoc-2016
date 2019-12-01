use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::fmt::{Debug, Display, Formatter};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::{FromIterator, IntoIterator};

#[derive(Eq, PartialEq, Hash, Debug, Clone)]
enum Unit {
    Generator(char),
    Microchip(char),
}

impl Ord for Unit {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Unit::Generator(_), Unit::Microchip(_)) => Ordering::Less,
            (Unit::Microchip(_), Unit::Generator(_)) => Ordering::Greater,
            (Unit::Generator(l), Unit::Generator(r)) => l.cmp(r),
            (Unit::Microchip(l), Unit::Microchip(r)) => l.cmp(r),
        }
    }
}

impl PartialOrd for Unit {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

struct FactoryLayout {
    floors: usize,
    elevator: usize,
    units: BTreeMap<Unit, usize>,
}

struct FactoryFloorIterator<'a> {
    factory: &'a FactoryLayout,
    floor: usize,
}

impl<'a> IntoIterator for &'a FactoryLayout {
    type Item = Vec<Unit>;
    type IntoIter = FactoryFloorIterator<'a>;
    fn into_iter(self) -> Self::IntoIter {
        FactoryFloorIterator {
            factory: self,
            floor: 0,
        }
    }
}

impl<'a> Iterator for FactoryFloorIterator<'a> {
    type Item = Vec<Unit>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.floor < self.factory.floors {
            let result: Self::Item = self
                .factory
                .units
                .iter()
                .filter_map(|(u, f)| {
                    if f == &self.floor {
                        Some(u.clone())
                    } else {
                        None
                    }
                })
                .collect::<Vec<Unit>>();
            self.floor += 1;
            Some(result)
        } else {
            None
        }
    }
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
        write!(f, "{}\n", result)
    }
}

impl FromIterator<String> for FactoryLayout {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        let (floors, units) = iter.into_iter().enumerate().fold(
            (0, BTreeMap::new()),
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
                        units_acc.insert(p, floor);
                    }
                }
                (usize::max(floor_acc, floor), units_acc)
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

    let result = (0..factory.floors - 1)
        .map(|i| {
            2 * factory
                .into_iter()
                .take(i + 1)
                .map(|v| v.len())
                .sum::<usize>()
                - 3
        })
        .sum::<usize>();

    println!("Result: {}", result);
}

pub fn run_e() {
    let input = File::open("input/task_11").unwrap();
    let input = BufReader::new(input);

    let mut factory = input
        .lines()
        .filter_map(|l| l.ok())
        .collect::<FactoryLayout>();
    factory.units.insert(Unit::Generator('e'), 0);
    factory.units.insert(Unit::Microchip('e'), 0);
    factory.units.insert(Unit::Generator('d'), 0);
    factory.units.insert(Unit::Microchip('d'), 0);

    println!("{:?}", factory);

    let result = (0..factory.floors - 1)
        .map(|i| {
            2 * factory
                .into_iter()
                .take(i + 1)
                .map(|v| v.len())
                .sum::<usize>()
                - 3
        })
        .sum::<usize>();

    println!("Result: {}", result);
}
