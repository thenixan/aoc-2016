use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct NumPad {
    current: u8,
}

struct CommandSequence {
    seq: Vec<Direction>,
}

impl CommandSequence {
    fn apply<F: Movable>(&self, num_pad: &mut F) {
        for d in &self.seq {
            num_pad.move_to(&d);
        }
    }
}

impl FromStr for CommandSequence {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let directions: Vec<Direction> = s
            .chars()
            .filter_map(|c| match c as u8 {
                b'R' => Some(Direction::Right),
                b'L' => Some(Direction::Left),
                b'D' => Some(Direction::Down),
                b'U' => Some(Direction::Up),
                _ => None,
            })
            .collect();
        Ok(CommandSequence { seq: directions })
    }
}

trait Movable {
    fn move_to(&mut self, d: &Direction);
}

struct CoolerNumPad {
    current: char,
}

impl CoolerNumPad {
    fn new() -> Self {
        CoolerNumPad { current: '5' }
    }

    fn move_up(&mut self) {
        self.current = match self.current {
            '6' => '2',
            'A' => '6',
            '3' => '1',
            '7' => '3',
            'B' => '7',
            'D' => 'B',
            '8' => '4',
            'C' => '8',
            c => c,
        };
    }

    fn move_down(&mut self) {
        self.current = match self.current {
            '2' => '6',
            '6' => 'A',
            '1' => '3',
            '3' => '7',
            '7' => 'B',
            'B' => 'D',
            '4' => '8',
            '8' => 'C',
            c => c,
        };
    }

    fn move_left(&mut self) {
        self.current = match self.current {
            '3' => '2',
            '4' => '3',
            '6' => '5',
            '7' => '6',
            '8' => '7',
            '9' => '8',
            'B' => 'A',
            'C' => 'B',
            c => c,
        };
    }

    fn move_right(&mut self) {
        self.current = match self.current {
            '2' => '3',
            '3' => '4',
            '5' => '6',
            '6' => '7',
            '7' => '8',
            '8' => '9',
            'A' => 'B',
            'B' => 'C',
            c => c,
        };
    }
}

impl Movable for CoolerNumPad {
    fn move_to(&mut self, d: &Direction) {
        match d {
            Direction::Up => self.move_up(),
            Direction::Down => self.move_down(),
            Direction::Left => self.move_left(),
            Direction::Right => self.move_right(),
        }
    }
}

impl NumPad {
    fn new() -> Self {
        NumPad { current: 5 }
    }

    fn move_up(&mut self) {
        self.current = match self.current {
            4 => 1,
            5 => 2,
            6 => 3,
            7 => 4,
            8 => 5,
            9 => 6,
            i => i,
        };
    }

    fn move_down(&mut self) {
        self.current = match self.current {
            1 => 4,
            2 => 5,
            3 => 6,
            4 => 7,
            5 => 8,
            6 => 9,
            i => i,
        };
    }

    fn move_left(&mut self) {
        self.current = match self.current {
            2 => 1,
            3 => 2,
            5 => 4,
            6 => 5,
            8 => 7,
            9 => 8,
            i => i,
        };
    }

    fn move_right(&mut self) {
        self.current = match self.current {
            1 => 2,
            2 => 3,
            4 => 5,
            5 => 6,
            7 => 8,
            8 => 9,
            i => i,
        };
    }
}

impl Movable for NumPad {
    fn move_to(&mut self, d: &Direction) {
        match d {
            Direction::Up => self.move_up(),
            Direction::Down => self.move_down(),
            Direction::Left => self.move_left(),
            Direction::Right => self.move_right(),
        }
    }
}

pub fn run() {
    let input = File::open("input/task_2").unwrap();
    let input = BufReader::new(input);

    let commands: Vec<CommandSequence> = input
        .lines()
        .filter_map(|l| l.ok())
        .filter_map(|s| s.parse::<CommandSequence>().ok())
        .collect();

    let mut num_pad = NumPad::new();

    let mut result = 0_u32;

    for c in commands {
        c.apply(&mut num_pad);
        result = result * 10 + num_pad.current as u32;
    }

    println!("Result: {}", result);
}

pub fn run_e() {
    let input = File::open("input/task_2").unwrap();
    let input = BufReader::new(input);

    let commands: Vec<CommandSequence> = input
        .lines()
        .filter_map(|l| l.ok())
        .filter_map(|s| s.parse::<CommandSequence>().ok())
        .collect();

    let mut num_pad = CoolerNumPad::new();

    let mut result = String::new();

    for c in commands {
        c.apply(&mut num_pad);
        result.push(num_pad.current);
    }

    println!("Result: {}", result);
}
