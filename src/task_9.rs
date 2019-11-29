use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::{BufReader, Bytes, Read};
use std::iter::{FromIterator, Take};
use std::ops::DerefMut;

struct Chunks {
    content: Vec<CompressionChunk>,
}

enum CompressionChunk {
    Plain { content: String },
    Compressed { body: Vec<Box<Self>>, times: usize },
}

impl CompressionChunk {
    fn plain(s: &str) -> Self {
        CompressionChunk::Plain {
            content: s.to_string(),
        }
    }
    fn compressed(c: Vec<CompressionChunk>, times: usize) -> Self {
        CompressionChunk::Compressed {
            body: c.into_iter().map(|c| Box::new(c)).collect(),
            times,
        }
    }
}

impl From<Vec<CompressionChunk>> for Chunks {
    fn from(vec: Vec<CompressionChunk>) -> Self {
        Chunks { content: vec }
    }
}

impl FromIterator<char> for Chunks {
    fn from_iter<I: IntoIterator<Item = char>>(iter: I) -> Self {
        let mut result = Vec::new();

        let iterator = iter.into_iter();
        let read = || {
            let c = iterator.next();
            match c {
                Some('(') => (),
                Some(o) => (),
                None => None
            }
        }
        let read_plain_chunk = || {
            
        };
        let read_sized_chunk = || {

        };
        result.into()
    }
}

pub fn run() {
    let input = File::open("input/task_9").unwrap();

    let s = input
        .bytes()
        .filter_map(|b| b.ok())
        .map(|b| b as char)
        .collect::<Chunks>();

    println!("Result {}", s.content.len())
}

pub fn run_e() {
    let input = File::open("input/task_9").unwrap();
    let input = BufReader::new(input);
}
