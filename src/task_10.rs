use std::collections::HashMap;
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::io::{BufRead, BufReader};
use std::iter::{FromIterator, IntoIterator};
use std::str::FromStr;

#[derive(Debug, Clone)]
struct Package {
    value: usize,
    target: Target,
}

impl Package {
    fn new(value: usize, target: Target) -> Self {
        Package { value, target }
    }
}

impl FromStr for Package {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split(" ").collect::<Vec<&str>>();
        let value = parts[1].parse::<usize>().unwrap();
        let to = parts[5].parse::<usize>().unwrap();
        let to = match parts[4] {
            "bot" => Target::to_bot(to),
            "output" => Target::to_output(to),
            _ => return Err(()),
        };
        Ok(Package::new(value, to))
    }
}

#[derive(Debug)]
struct Output {
    number: usize,
}

#[derive(Debug, Clone)]
enum Target {
    ToBot(usize),
    ToOutput(usize),
}

impl Target {
    fn to_bot(number: usize) -> Self {
        Target::ToBot(number)
    }
    fn to_output(number: usize) -> Self {
        Target::ToOutput(number)
    }
}

#[derive(Debug)]
struct Bot {
    number: usize,
    lower_to: Target,
    higher_to: Target,
}

impl Bot {
    fn new(number: usize, lower_to: Target, higher_to: Target) -> Self {
        Bot {
            number,
            lower_to,
            higher_to,
        }
    }
}

impl FromStr for Bot {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split(" ").collect::<Vec<&str>>();
        let number = parts[1].parse::<usize>().unwrap();
        let l_to = parts[6].parse::<usize>().unwrap();
        let h_to = parts[11].parse::<usize>().unwrap();
        let l_to = match parts[5] {
            "bot" => Target::to_bot(l_to),
            "output" => Target::to_output(l_to),
            _ => return Err(()),
        };
        let h_to = match parts[10] {
            "bot" => Target::to_bot(h_to),
            "output" => Target::to_output(h_to),
            _ => return Err(()),
        };
        Ok(Bot::new(number, l_to, h_to))
    }
}

impl PartialEq for Bot {
    fn eq(&self, other: &Self) -> bool {
        self.number == other.number
    }
}
impl Eq for Bot {}

impl Hash for Bot {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.number.hash(state);
    }
}

#[derive(Debug)]
struct Factory {
    inputs: Vec<Package>,
    bots: Vec<Bot>,
}

impl Factory {
    fn work(&self) -> FactoryIterator {
        FactoryIterator::new(self)
    }
}

impl FromIterator<String> for Factory {
    fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
        let mut inputs = vec![];
        let mut bots = vec![];

        for l in iter {
            if l.starts_with("bot ") {
                bots.push(l.parse::<Bot>().unwrap());
            } else if l.starts_with("value ") {
                inputs.push(l.parse::<Package>().unwrap());
            }
        }

        Factory { inputs, bots }
    }
}

struct FactoryIterator<'a> {
    inputs: std::vec::IntoIter<Package>,
    factory: &'a Factory,
    state: HashMap<usize, Vec<usize>>,
    packages: Vec<(usize, Package)>,
}

impl<'a> FactoryIterator<'a> {
    fn new(factory: &'a Factory) -> Self {
        FactoryIterator {
            inputs: factory.inputs.clone().into_iter(),
            factory,
            state: HashMap::new(),
            packages: Vec::new(),
        }
    }
}
impl Iterator for FactoryIterator<'_> {
    type Item = FactoryHistoryRecord;
    fn next(&mut self) -> Option<Self::Item> {
        match self.inputs.next() {
            None => match self.packages.pop() {
                Some((from, package)) => {
                    match package.target {
                        Target::ToBot(number) => self
                            .state
                            .entry(number)
                            .or_insert(vec![])
                            .push(package.value),
                        Target::ToOutput(_) => {}
                    };
                    Some(FactoryHistoryRecord::Transmission {
                        _from_bot: from,
                        package: package,
                    })
                }
                None => {
                    let bot = self
                        .state
                        .iter()
                        .find_map(|(k, v)| if v.len() == 2 { Some(k) } else { None })
                        .cloned();
                    if bot.is_some() {
                        let bot = bot.unwrap();
                        let values = self.state.remove(&bot).unwrap();
                        let bot = self.factory.bots.iter().find(|b| b.number == bot).unwrap();
                        let min = values.iter().min().unwrap();
                        let max = values.iter().max().unwrap();
                        let min = Package::new(*min, bot.lower_to.clone());
                        let max = Package::new(*max, bot.higher_to.clone());
                        self.packages.push((bot.number, min));
                        self.packages.push((bot.number, max));
                        Some(FactoryHistoryRecord::Comparation {
                            bot: bot.number,
                            values: (values[0], values[1]),
                        })
                    } else {
                        None
                    }
                }
            },
            Some(p) => {
                match p.target {
                    Target::ToBot(b) => self.state.entry(b).or_insert(vec![]).push(p.value),
                    Target::ToOutput(_) => (),
                };
                Some(FactoryHistoryRecord::Input { _package: p })
            }
        }
    }
}

enum FactoryHistoryRecord {
    Comparation { bot: usize, values: (usize, usize) },
    Transmission { _from_bot: usize, package: Package },
    Input { _package: Package },
}

pub fn run() {
    let input = File::open("input/task_10").unwrap();
    let input = BufReader::new(input);

    let factory = input.lines().filter_map(|l| l.ok()).collect::<Factory>();

    let result = factory.work().find_map(|r| match r {
        FactoryHistoryRecord::Comparation { bot, values } => {
            let min = usize::min(values.0, values.1);
            let max = usize::max(values.0, values.1);
            if min == 17 && max == 61 {
                Some(bot)
            } else {
                None
            }
        }
        _ => None,
    });

    println!("{:?}", result);
}

pub fn run_e() {
    let input = File::open("input/task_10").unwrap();
    let input = BufReader::new(input);

    let factory = input.lines().filter_map(|l| l.ok()).collect::<Factory>();

    let result = factory
        .work()
        .filter_map(|r| match r {
            FactoryHistoryRecord::Transmission {
                _from_bot: _,
                package,
            } => match package.target {
                Target::ToOutput(number) => {
                    if number == 0 || number == 1 || number == 2 {
                        Some(package.value)
                    } else {
                        None
                    }
                }
                _ => None,
            },
            _ => None,
        })
        .product::<usize>();

    println!("{:?}", result);
}
