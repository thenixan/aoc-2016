use std::fs::File;
use std::io::{BufRead, BufReader};

enum Destination {
    North,
    South,
    East,
    West,
}

impl Destination {
    fn clockwise(&self) -> Self {
        match self {
            Destination::North => Destination::East,
            Destination::East => Destination::South,
            Destination::South => Destination::West,
            Destination::West => Destination::North,
        }
    }
    fn counter_clockwise(&self) -> Self {
        match self {
            Destination::North => Destination::West,
            Destination::West => Destination::South,
            Destination::South => Destination::East,
            Destination::East => Destination::North,
        }
    }
}

#[derive(Debug)]
enum RoutePart {
    Clockwise(u8),
    CounterClockwise(u8),
}

#[derive(Eq, PartialEq, Clone)]
struct Coords {
    x: i32,
    y: i32,
}

impl Coords {
    fn move_by(&mut self, r: u8, destination: &Destination) -> Vec<Coords> {
        match destination {
            Destination::North => {
                self.y += r as i32;
                (0..r)
                    .map(|i| Coords {
                        x: self.x,
                        y: self.y + i as i32 - r as i32,
                    })
                    .collect()
            }
            Destination::East => {
                self.x += r as i32;
                (1..r)
                    .map(|i| Coords {
                        x: self.x + i as i32 - r as i32,
                        y: self.y,
                    })
                    .collect()
            }
            Destination::South => {
                self.y -= r as i32;
                (1..r)
                    .map(|i| Coords {
                        x: self.x,
                        y: self.y - i as i32 + r as i32,
                    })
                    .collect()
            }
            Destination::West => {
                self.x -= r as i32;
                (1..r)
                    .map(|i| Coords {
                        x: self.x - i as i32 + r as i32,
                        y: self.y,
                    })
                    .collect()
            }
        }
    }
}

pub fn run() {
    let input = File::open("input/task_1").unwrap();
    let mut input = BufReader::new(input);

    let mut s = String::new();

    input.read_line(&mut s).unwrap();
    let s = s
        .split(", ")
        .map(|s| {
            if s.starts_with("L") {
                RoutePart::CounterClockwise(s[1..].to_string().parse::<u8>().unwrap())
            } else {
                RoutePart::Clockwise(s[1..].parse::<u8>().unwrap())
            }
        })
        .collect::<Vec<RoutePart>>();

    let result = evaluate(
        &mut s.iter(),
        &Destination::North,
        &mut Coords { x: 0, y: 0 },
    );
    println!("Result: {:?}", result);
}

pub fn run_e() {
    let input = File::open("input/task_1").unwrap();
    let mut input = BufReader::new(input);

    let mut s = String::new();

    input.read_line(&mut s).unwrap();
    let s = s
        .split(", ")
        .map(|s| {
            if s.starts_with("L") {
                RoutePart::CounterClockwise(s[1..].to_string().parse::<u8>().unwrap())
            } else {
                RoutePart::Clockwise(s[1..].parse::<u8>().unwrap())
            }
        })
        .collect::<Vec<RoutePart>>();

    let result = evaluate_second_visit(
        &mut s.iter(),
        &Destination::North,
        &mut vec![],
        &mut Coords { x: 0, y: 0 },
    );
    println!("Result: {:?}", result);
}

fn evaluate(
    route: &mut dyn Iterator<Item = &RoutePart>,
    destination: &Destination,
    coords: &mut Coords,
) -> i32 {
    let route_part = route.next();
    match route_part {
        Some(RoutePart::Clockwise(r)) => {
            let new_destination = destination.clockwise();
            coords.move_by(*r, &new_destination);
            evaluate(route, &new_destination, coords)
        }
        Some(RoutePart::CounterClockwise(r)) => {
            let new_destination = destination.counter_clockwise();
            coords.move_by(*r, &new_destination);
            evaluate(route, &new_destination, coords)
        }
        None => coords.x.abs() + coords.y.abs(),
    }
}

fn evaluate_second_visit(
    route: &mut dyn Iterator<Item = &RoutePart>,
    destination: &Destination,
    path: &mut Vec<Coords>,
    current: &mut Coords,
) -> i32 {
    let route_part = route.next();
    match route_part {
        Some(RoutePart::Clockwise(r)) => {
            let new_destination = destination.clockwise();
            for p in current.move_by(*r, &new_destination) {
                if path.contains(&p) {
                    return p.x.abs() + p.y.abs();
                } else {
                    path.push(p);
                }
            }
            evaluate_second_visit(route, &new_destination, path, current)
        }
        Some(RoutePart::CounterClockwise(r)) => {
            let new_destination = destination.counter_clockwise();
            for p in current.move_by(*r, &new_destination) {
                if path.contains(&p) {
                    return p.x.abs() + p.y.abs();
                } else {
                    path.push(p);
                }
            }
            evaluate_second_visit(route, &new_destination, path, current)
        }
        None => 0,
    }
}
