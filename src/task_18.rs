use std::fs::File;
use std::io::Read;
use std::str::FromStr;

enum Tile {
    Safe,
    Trap,
}

impl Tile {
    fn is_trap(&self) -> bool {
        match self {
            Tile::Safe => false,
            Tile::Trap => true,
        }
    }
}

struct TrapRow(Vec<Tile>);

impl TrapRow {
    fn prev_row_weights<'a>(&'a self, position: usize) -> [&'a Tile; 3] {
        let mut result = [&Tile::Safe; 3];
        if position == 0 {
            result[0] = &Tile::Safe;
        } else {
            result[0] = &self.0[position - 1];
        }
        if position == self.0.len() - 1 {
            result[2] = &Tile::Safe;
        } else {
            result[2] = &self.0[position + 1];
        }
        result[1] = &self.0[position];
        result
    }

    fn next_row(&self) -> Self {
        let mut result = vec![];
        for i in 0..self.0.len() {
            let prev = self.prev_row_weights(i);
            let next = if prev[0].is_trap() && !prev[2].is_trap() {
                Tile::Trap
            } else if !prev[0].is_trap() && prev[2].is_trap() {
                Tile::Trap
            } else {
                Tile::Safe
            };
            result.push(next);
        }
        TrapRow(result)
    }

    fn count_safe(&self) -> usize {
        self.0.iter().filter(|t| !t.is_trap()).count()
    }
}

impl FromStr for TrapRow {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut result = vec![];
        for c in s.chars() {
            match c {
                '.' => result.push(Tile::Safe),
                '^' => result.push(Tile::Trap),
                _ => (),
            }
        }
        Ok(TrapRow(result))
    }
}

pub fn run() {
    let mut input = File::open("input/task_18").unwrap();
    let mut buffer = String::new();

    input.read_to_string(&mut buffer).unwrap();

    let row = buffer.parse::<TrapRow>().unwrap();

    let mut plane = vec![row];
    for _ in 1..40 {
        let next = plane.last().unwrap().next_row();
        plane.push(next);
    }

    let result = plane.iter().map(|r| r.count_safe()).sum::<usize>();
    println!("Result: {}", result);
}

pub fn run_e() {
    let mut input = File::open("input/task_18").unwrap();
    let mut buffer = String::new();

    input.read_to_string(&mut buffer).unwrap();

    let row = buffer.parse::<TrapRow>().unwrap();

    let mut plane = vec![row];
    for _ in 1..400000 {
        let next = plane.last().unwrap().next_row();
        plane.push(next);
    }

    let result = plane.iter().map(|r| r.count_safe()).sum::<usize>();
    println!("Result: {}", result);
}
