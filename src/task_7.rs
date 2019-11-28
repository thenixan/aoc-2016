use std::fmt::{Debug, Display, Formatter};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

trait AbbaValid {
    fn is_valid(&self) -> bool;
}

struct ByteAllocationBlock {
    s1: char,
    s2: char,
}

trait ByteAllocationBlockGenerator {
    fn get_byte_allocation_blocks(&self) -> Vec<ByteAllocationBlock>;
}

trait ByteAllocationConsumer {
    fn validate_byte_allocation_block(&self, blocks: &Vec<ByteAllocationBlock>) -> bool;
}

#[derive(Debug)]
struct AddressPart {
    value: String,
}

impl Display for AddressPart {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl ByteAllocationBlockGenerator for AddressPart {
    fn get_byte_allocation_blocks(&self) -> Vec<ByteAllocationBlock> {
        let mut result = vec![];
        if self.value.len() >= 3 {
            let s = self.value.as_str().as_bytes();
            for i in 1..s.len() - 1 {
                if s[i - 1] == s[i + 1] && s[i] != s[i - 1] {
                    result.push(ByteAllocationBlock {
                        s1: s[i - 1] as char,
                        s2: s[i] as char,
                    })
                }
            }
        }
        result
    }
}

#[derive(Debug)]
struct HypernetSequence {
    value: String,
}

impl ByteAllocationConsumer for HypernetSequence {
    fn validate_byte_allocation_block(&self, blocks: &Vec<ByteAllocationBlock>) -> bool {
        let mut result = false;
        if self.value.len() >= 3 {
            let s = self.value.as_str().as_bytes();
            for i in 1..s.len() - 1 {
                if s[i - 1] == s[i + 1]
                    && s[i] != s[i - 1]
                    && blocks
                        .iter()
                        .any(|b| b.s1 == s[i] as char && b.s2 == s[i - 1] as char)
                {
                    result = true;
                }
            }
        }
        result
    }
}

impl Display for HypernetSequence {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl FromStr for AddressPart {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(AddressPart {
            value: s.to_string(),
        })
    }
}

trait BabValid {
    fn is_valid(&self) -> bool;
}

impl AbbaValid for AddressPart {
    fn is_valid(&self) -> bool {
        if self.value.len() < 4 {
            false
        } else {
            let mut result = false;
            let s = self.value.as_str().as_bytes();
            for i in 1..s.len() - 2 {
                if s[i - 1] == s[i + 2] && s[i] == s[i + 1] && s[i] != s[i - 1] {
                    result = true;
                }
            }
            result
        }
    }
}

impl FromStr for HypernetSequence {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(HypernetSequence {
            value: s.to_string(),
        })
    }
}

impl AbbaValid for HypernetSequence {
    fn is_valid(&self) -> bool {
        if self.value.len() < 4 {
            true
        } else {
            let mut result = true;
            let s = self.value.as_str().as_bytes();
            for i in 1..s.len() - 2 {
                if s[i - 1] == s[i + 2] && s[i] == s[i + 1] {
                    result = false;
                }
            }
            result
        }
    }
}

struct Address<A, B> {
    address: Vec<A>,
    hypernet_seq: Vec<B>,
}

impl<A: AbbaValid, B: AbbaValid> AbbaValid for Address<A, B> {
    fn is_valid(&self) -> bool {
        self.address.iter().any(|a| a.is_valid()) && self.hypernet_seq.iter().all(|s| s.is_valid())
    }
}

impl<A: ByteAllocationBlockGenerator, B: ByteAllocationConsumer> BabValid for Address<A, B> {
    fn is_valid(&self) -> bool {
        let byte_allocation_blocks = self
            .address
            .iter()
            .flat_map(|a| a.get_byte_allocation_blocks())
            .collect::<Vec<_>>();
        if byte_allocation_blocks.is_empty() {
            false
        } else {
            self.hypernet_seq
                .iter()
                .any(|s| s.validate_byte_allocation_block(&byte_allocation_blocks))
        }
    }
}

impl<A: FromStr, B: FromStr> FromStr for Address<A, B> {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (parts, hypernets) =
            s.split(|c| c == '[' || c == ']')
                .fold((vec![], vec![]), |mut acc, p| {
                    if acc.0.len() == acc.1.len() {
                        acc.0.push(p);
                    } else {
                        acc.1.push(p);
                    }
                    acc
                });
        let parts = parts.iter().filter_map(|p| p.parse::<A>().ok()).collect();
        let hypernets = hypernets
            .iter()
            .filter_map(|p| p.parse::<B>().ok())
            .collect();
        Ok(Address {
            address: parts,
            hypernet_seq: hypernets,
        })
    }
}

pub fn run() {
    let input = File::open("input/task_7").unwrap();
    let input = BufReader::new(input);

    let result = input
        .lines()
        .filter_map(|l| l.ok())
        .filter_map(|l| l.parse::<Address<AddressPart, HypernetSequence>>().ok())
        .filter(|a| AbbaValid::is_valid(a))
        .collect::<Vec<_>>();

    println!("Result: {}", result.len());
}

pub fn run_e() {
    let input = File::open("input/task_7").unwrap();
    let input = BufReader::new(input);

    let result = input
        .lines()
        .filter_map(|l| l.ok())
        .filter_map(|l| l.parse::<Address<AddressPart, HypernetSequence>>().ok())
        .filter(|a| BabValid::is_valid(a))
        .collect::<Vec<_>>();

    println!("Result: {}", result.len());
}
