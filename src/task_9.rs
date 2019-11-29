use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::{BufReader, Read};
use std::ops::Deref;

#[derive(Clone, Debug)]
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
    fn decompressable(&self) -> bool {
        self.iter().any(|c| c.is_compressed())
    }
    fn decompress_top(&self) -> Self::Into {
        Data {
            chunks: self.iter().map(|c| c.decompress_top()).flatten().collect(),
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
                    chunks.push(Chunk::plain(&counter));
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
                chunks.push(Chunk::compressed(&c, times));
            } else {
                counter.push(b as char);
            }
        }
        if counter.len() != 0 {
            println!("{}", counter);
            chunks.push(Chunk::plain(&counter));
        }
        Data { chunks }
    }
}

#[derive(Clone, Debug)]
enum Chunk {
    Plain { content: String },
    Compressed { repeats: usize, content: Vec<Chunk> },
}

impl Chunk {
    fn plain(s: &str) -> Self {
        Chunk::Plain {
            content: s.to_string(),
        }
    }
    fn compressed(s: &str, l: usize) -> Self {
        Chunk::Compressed {
            content: Data::from(s.as_bytes()).to_vec(),
            repeats: l,
        }
    }

    fn is_compressed(&self) -> bool {
        match self {
            Chunk::Plain { content: _ } => false,
            Chunk::Compressed {
                content: _,
                repeats: _,
            } => true,
        }
    }
}

trait Compressed {
    type Into;
    fn decompressable(&self) -> bool;
    fn decompress_top(&self) -> Self::Into;
}

impl Display for Chunk {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Chunk::Plain { content } => write!(f, "{}", content),
            Chunk::Compressed { content, repeats } => {
                let content: String = content.iter().map(|c: &Chunk| c.to_string()).collect();
                write!(f, "({}x{}){}", content.len(), repeats, content)
            }
        }
    }
}

impl Compressed for Chunk {
    type Into = Vec<Self>;
    fn decompressable(&self) -> bool {
        self.is_compressed()
    }
    fn decompress_top(&self) -> Vec<Self> {
        match self {
            Chunk::Plain { content } => vec![self.clone()],
            Chunk::Compressed { content, repeats } => std::iter::repeat(content.clone())
                .take(*repeats)
                .flatten()
                .collect(),
        }
    }
}

pub fn run() {
    let input = File::open("input/task_9").unwrap();

    let chunks = Data::from(input).decompress_top();

    for c in chunks.iter() {
        println!("Chunks: {}", c);
    }

    let result = chunks.iter().map(|c| c.to_string().len()).sum::<usize>();
    println!("Result: {}", result);
}

pub fn run_e() {
    let input = File::open("input/task_9").unwrap();
    let _input = BufReader::new(input);
}
