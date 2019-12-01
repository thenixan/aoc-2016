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

impl Unit {
    fn element(&self) -> char {
        match self {
            Unit::Generator(c) => *c,
            Unit::Microchip(c) => *c,
        }
    }
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

#[derive(Clone, Debug)]
enum MovementCombination {
    Pair(Unit, Unit),
    Single(Unit),
}

#[derive(Clone)]
struct FactoryLayout {
    floors: usize,
    elevator: usize,
    units: BTreeMap<Unit, usize>,
}

impl FactoryLayout {
    fn is_finished(&self) -> bool {
        self.units.values().all(|u| u == &self.floors)
    }

    fn possible_to_move(&self) -> Vec<MovementCombination> {
        let mut result = vec![];
        let units = self
            .units
            .iter()
            .filter_map(|(k, v)| if v == &self.elevator { Some(k) } else { None })
            .collect::<Vec<&Unit>>();
        if units.len() > 0 {
            for i in 0..&units.len() - 1 {
                result.push(MovementCombination::Single(units[i].clone()));
                for j in i + 1..units.len() {
                    result.push(MovementCombination::Pair(
                        units[i].clone(),
                        units[j].clone(),
                    ));
                }
            }
            result.push(MovementCombination::Single(units[units.len() - 1].clone()));
        }
        result
    }

    fn move_up(&mut self, combination: MovementCombination) -> bool {
        if self.floors == self.elevator + 1 {
            false
        } else {
            match &combination {
                MovementCombination::Pair(f, s) => {
                    *self.units.entry(f.clone()).or_default() += 1;
                    *self.units.entry(s.clone()).or_default() += 1;
                }
                MovementCombination::Single(f) => {
                    *self.units.entry(f.clone()).or_default() += 1;
                }
            };
            self.elevator += 1;
            self.into_iter().all(|u| u.iter().check())
        }
    }

    fn move_down(&mut self, combination: MovementCombination) -> bool {
        if 0 == self.elevator {
            false
        } else {
            match combination {
                MovementCombination::Pair(f, s) => {
                    *self.units.entry(f).or_default() -= 1;
                    *self.units.entry(s).or_default() -= 1;
                }
                MovementCombination::Single(f) => {
                    *self.units.entry(f).or_default() -= 1;
                }
            };
            self.elevator -= 1;
            self.into_iter().all(|u| u.iter().check())
        }
    }
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

trait Validator {
    fn check(&mut self) -> bool;
}

impl<'a, T> Validator for T
where
    T: Iterator<Item = &'a Unit>,
{
    fn check(&mut self) -> bool {
        let mut generators = vec![];
        let mut microchips = vec![];

        while let Some(u) = self.next() {
            match u {
                Unit::Microchip(c) => microchips.push(c),
                Unit::Generator(c) => generators.push(c),
            }
        }

        microchips.iter().all(|m| generators.iter().any(|g| g == m)) || generators.is_empty()
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

#[inline]
fn evaluate(factory_layout: &FactoryLayout, found_min: Option<usize>) -> Option<usize> {
    let variants = factory_layout.possible_to_move();
    if variants.is_empty() || (found_min.is_some() && found_min == Some(0)) {
        None
    } else {
        let (pairs, singles): (Vec<MovementCombination>, Vec<MovementCombination>) =
            variants.into_iter().partition(|v| match v {
                MovementCombination::Pair(_, _) => true,
                MovementCombination::Single(_) => false,
            });
        println!("{:?}", factory_layout);
        let mut new_min = found_min.clone();
        for pair in &pairs {
            let mut f = factory_layout.clone();
            if f.move_up(pair.clone()) {
                if f.is_finished() {
                    new_min = Some(1);
                } else {
                    new_min = evaluate(&f, new_min.map(|m| m - 1))
                        .map(|m| m + 1)
                        .or_else(|| found_min.clone());
                }
            }
        }
        for single in &singles {
            let mut f = factory_layout.clone();
            if f.move_down(single.clone()) {
                if f.is_finished() {
                    new_min = Some(1);
                } else {
                    new_min = evaluate(&f, new_min.map(|m| m - 1))
                        .map(|m| m + 1)
                        .or_else(|| found_min.clone());
                }
            }
        }
        for single in &singles {
            let mut f = factory_layout.clone();
            if f.move_up(single.clone()) {
                if f.is_finished() {
                    new_min = Some(1);
                } else {
                    new_min = evaluate(&f, new_min.map(|m| m - 1))
                        .map(|m| m + 1)
                        .or_else(|| found_min.clone());
                }
            }
        }
        for pair in &pairs {
            let mut f = factory_layout.clone();
            if f.move_down(pair.clone()) {
                if f.is_finished() {
                    new_min = Some(1);
                } else {
                    new_min = evaluate(&f, new_min.map(|m| m - 1))
                        .map(|m| m + 1)
                        .or_else(|| found_min.clone());
                }
            }
        }
        new_min
    }
}

pub fn run() {
    let input = File::open("input/task_11").unwrap();
    let input = BufReader::new(input);

    let factory = input
        .lines()
        .filter_map(|l| l.ok())
        .collect::<FactoryLayout>();

    let result = evaluate(&factory, Some(100));

    println!("Result: {:?}", result);
}

pub fn run_e() {
    let input = File::open("input/task_10").unwrap();
    let _input = BufReader::new(input);
}
