use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::{BufReader, Read};
use std::ops::Deref;

#[derive(Clone)]
struct Data<A: Clone> {
    chunks: Vec<A>,
}

impl<A: Clone> Deref for Data<A> {
    type Target = Vec<A>;

    fn deref(&self) -> &Self::Target {
        &self.chunks
    }
}

impl Compressed for Data<Chunk> {
    type Into = Self;
    fn is_compressed(&self) -> bool {
        self.iter().any(|c| c.is_compressed())
    }

    fn chunks(&self) -> Self::Into {
        Data {
            chunks: self.iter().map(|c| c.chunks()).flatten().collect(),
        }
    }
}

impl<A: Read> From<A> for Data<Chunk> {
    fn from(read: A) -> Self {
        let mut bytes = read.bytes().filter_map(|b| b.ok());
        let mut counter = String::new();
        let mut chunks = vec![];
        while let Some(b) = bytes.next() {
            if b == b'(' {
                if counter.len() != 0 {
                    println!("{}", counter);
                    chunks.push(Chunk::Plain(counter));
                    counter = String::new();
                }
                let mut c = String::new();
                let mut buf = bytes.next().unwrap();
                while buf != b'x' {
                    c.push(buf as char);
                    buf = bytes.next().unwrap();
                }
                let length = c.parse::<usize>().unwrap();
                c = String::new();
                buf = bytes.next().unwrap();
                while buf != b')' {
                    c.push(buf as char);
                    buf = bytes.next().unwrap();
                }
                let times = c.parse::<usize>().unwrap();
                c = String::new();
                for _ in 0..length {
                    c.push(bytes.next().unwrap() as char);
                }
                chunks.push(Chunk::Compressed(c, times));
            } else {
                counter.push(b as char);
            }
        }
        if counter.len() != 0 {
            println!("{}", counter);
            chunks.push(Chunk::Plain(counter));
        }
        Data { chunks }
    }
}

#[derive(Clone)]
enum Chunk {
    Plain(String),
    Compressed(String, usize),
}

trait Compressed {
    type Into;
    fn is_compressed(&self) -> bool;
    fn chunks(&self) -> Self::Into;
}

impl Display for Chunk {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Chunk::Plain(s) => write!(f, "{}", s),
            Chunk::Compressed(s, l) => write!(f, "({}x{}){}", s.len(), l, s),
        }
    }
}

impl Compressed for Chunk {
    type Into = Vec<Self>;
    fn is_compressed(&self) -> bool {
        match self {
            Chunk::Plain(_) => false,
            Chunk::Compressed(_, _) => true,
        }
    }

    fn chunks(&self) -> Self::Into {
        let c = match self {
            Chunk::Plain(s) => s.clone(),
            Chunk::Compressed(s, t) => std::iter::repeat(s.clone()).take(*t).collect::<String>(),
        };
        Data::from(c.as_bytes()).to_vec()
    }
}

pub fn run() {
    let input = File::open("input/task_9").unwrap();

    let chunks = Data::from(input);

    let result = chunks.iter().map(|c| c.to_string().len()).sum::<usize>();
    println!("Result: {}", result);
}

pub fn run_e() {
    let input = File::open("input/task_9").unwrap();
    let _input = BufReader::new(input);
}
