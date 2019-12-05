use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

struct IpAddressRange {
    from: u32,
    to: u32,
}

impl IpAddressRange {
    fn is_valid(&self, value: u32) -> bool {
        value < self.from || self.to < value
    }
}

impl FromStr for IpAddressRange {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s
            .split("-")
            .filter_map(|l| l.parse::<u32>().ok())
            .collect::<Vec<u32>>();
        Ok(IpAddressRange {
            from: parts[0],
            to: parts[1],
        })
    }
}

struct AddressFilter<'a, T: Iterator<Item = u32>> {
    iter: T,
    range: Vec<&'a IpAddressRange>,
}

impl<'a, T: Iterator<Item = u32>> Iterator for AddressFilter<'a, T> {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        while let Some(n) = self.iter.next() {
            if self.is_valid(n) {
                return Some(n);
            }
        }
        None
    }
}

impl<'a, T: Iterator<Item = u32>> AddressFilter<'a, T> {
    fn new(iter: T, range: Vec<&'a IpAddressRange>) -> Self {
        AddressFilter { iter, range }
    }

    fn is_valid(&self, value: u32) -> bool {
        self.range.iter().all(|r| r.is_valid(value))
    }
}

pub trait AddressFilterIterator<T: Iterator<Item = u32>> {
    fn filter_range(self, range: Vec<&IpAddressRange>) -> AddressFilter<T>;
}

impl<T: IntoIterator<Item = u32>> AddressFilterIterator<T::IntoIter> for T {
    fn filter_range(self, range: Vec<&IpAddressRange>) -> AddressFilter<T::IntoIter> {
        AddressFilter::new(self.into_iter(), range)
    }
}

pub fn run() {
    let input = File::open("input/task_20").unwrap();
    let input = BufReader::new(input);

    let ranges = input
        .lines()
        .filter_map(|l| l.ok())
        .filter_map(|l| l.parse::<IpAddressRange>().ok())
        .collect::<Vec<IpAddressRange>>();

    let result = (0..=std::u32::MAX)
        .find(|a| ranges.iter().all(|f| f.is_valid(*a)))
        .unwrap();

    println!("Result: {}", result);
}

pub fn run_e() {
    let mut input = File::open("input/task_19").unwrap();
    let mut buffer = String::new();
}
