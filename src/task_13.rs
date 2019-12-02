use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use std::fs::File;
use std::io::Read;

struct Room {
    seed: usize,
}

struct Path<'a> {
    room: &'a Room,
    visited: HashMap<(usize, usize), usize>,
    position: (usize, usize),
}

impl<'a> Path<'a> {
    fn new(start_x: usize, start_y: usize, room: &'a Room) -> Self {
        Path {
            room,
            visited: HashMap::new(),
            position: (start_x, start_y),
        }
    }

    fn evaluate(&mut self, radius: usize) {
        self.evaluate_iterative(radius, self.position, 0);
    }

    fn evaluate_iterative(&mut self, radius: usize, position: (usize, usize), step: usize) {
        if !self.room.wall_at(position.0, position.1) {
            if self
                .visited
                .entry(position)
                .or_insert(std::usize::MAX)
                .clone()
                > step
            {
                self.visited.insert(position, step);
                if 0 < position.0 {
                    self.evaluate_iterative(radius, (position.0 - 1, position.1), step + 1);
                }
                if position.0 < radius {
                    self.evaluate_iterative(radius, (position.0 + 1, position.1), step + 1);
                }
                if 0 < position.1 {
                    self.evaluate_iterative(radius, (position.0, position.1 - 1), step + 1);
                }
                if position.1 < radius {
                    self.evaluate_iterative(radius, (position.0, position.1 + 1), step + 1);
                }
            }
        }
    }
}

impl Room {
    fn new(seed: usize) -> Self {
        Room { seed }
    }

    fn wall_at(&self, x: usize, y: usize) -> bool {
        let sum = x * x + 3 * x + 2 * x * y + y + y * y + self.seed;
        let sum = format!("{:b}", sum).replace("0", "").len();
        sum % 2 == 1
    }

    fn print_radius(&self, radius: usize) -> RoomPrinter {
        RoomPrinter::new(self, radius)
    }
}

struct RoomPrinter<'a> {
    room: &'a Room,
    radius: usize,
}

impl<'a> RoomPrinter<'a> {
    fn new(room: &'a Room, radius: usize) -> Self {
        RoomPrinter { room, radius }
    }
}

impl<'a> Debug for RoomPrinter<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut result = String::new();
        for y in 0..self.radius {
            if y == 0 {
                result.push_str("  ");
                for x in 0..self.radius {
                    result.push_str(format!(" {:2}", x).as_str());
                }
                result.push('\n');
            }
            for x in 0..self.radius {
                if x == 0 {
                    result.push_str(format!("{:2}", y).as_str());
                }
                result.push(' ');
                if self.room.wall_at(x, y) {
                    result.push('#');
                    result.push('#');
                } else {
                    result.push(' ');
                    result.push(' ');
                }
            }
            result.push('\n');
        }
        write!(f, "{}", result)
    }
}

pub fn run() {
    let mut input = File::open("input/task_13").unwrap();
    let mut buffer = String::new();
    input.read_to_string(&mut buffer).unwrap();
    let input = buffer.parse::<usize>().unwrap();

    let room = Room::new(input);

    let mut path = Path::new(1, 1, &room);

    let target = (31, 39);

    let radius = usize::max(target.0, target.1) * 2;

    path.evaluate(radius);

    println!("Result: {}", path.visited[&target])
}

pub fn run_e() {
    let mut input = File::open("input/task_13").unwrap();
    let mut buffer = String::new();
    input.read_to_string(&mut buffer).unwrap();
    let input = buffer.parse::<usize>().unwrap();

    let room = Room::new(input);

    let mut path = Path::new(1, 1, &room);

    let target = (25, 25);

    let radius = usize::max(target.0, target.1) * 2;

    path.evaluate(radius);
    let result = path.visited.iter().filter(|(_, v)| v <= &&50).count();

    println!("Result: {}", result)
}
