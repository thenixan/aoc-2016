use std::fmt::Debug;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::{FromIterator, IntoIterator};
use std::str::FromStr;

#[derive(Debug)]
struct Programm {
    commands: Vec<Command>,
}

impl Programm {
    fn run(&self) -> ProgramExecution {
        Programm::run_with_values(self, 0, 0, 0, 0)
    }
    fn run_with_values(&self, a: i32, b: i32, c: i32, d: i32) -> ProgramExecution {
        ProgramExecution::new(self, a, b, c, d)
    }
}

#[derive(Clone, Debug)]
struct Register {
    a: i32,
    b: i32,
    c: i32,
    d: i32,
}

impl Register {
    fn new(a: i32, b: i32, c: i32, d: i32) -> Self {
        Register { a, b, c, d }
    }
}

struct ProgramExecution<'a> {
    programm: &'a Programm,
    registers: Register,
    position: usize,
}

impl<'a> ProgramExecution<'a> {
    fn new(programm: &'a Programm, a: i32, b: i32, c: i32, d: i32) -> Self {
        ProgramExecution {
            programm,
            registers: Register::new(a, b, c, d),
            position: 0,
        }
    }
}

impl<'a> Iterator for ProgramExecution<'a> {
    type Item = Register;

    fn next(&mut self) -> Option<Self::Item> {
        let current_command = self.programm.commands.get(self.position);
        match current_command {
            Some(Command::Inc(target)) => {
                match target {
                    'a' => self.registers.a += 1,
                    'b' => self.registers.b += 1,
                    'c' => self.registers.c += 1,
                    'd' => self.registers.d += 1,
                    _ => (),
                };
                self.position += 1;
                Some(self.registers.clone())
            }
            Some(Command::Dec(target)) => {
                match target {
                    'a' => self.registers.a -= 1,
                    'b' => self.registers.b -= 1,
                    'c' => self.registers.c -= 1,
                    'd' => self.registers.d -= 1,
                    _ => (),
                };
                self.position += 1;
                Some(self.registers.clone())
            }
            Some(Command::JumpNonZero(target, value)) => {
                if match target {
                    Source::Value(v) => v != &0,
                    Source::Register('a') => self.registers.a != 0,
                    Source::Register('b') => self.registers.b != 0,
                    Source::Register('c') => self.registers.c != 0,
                    Source::Register('d') => self.registers.d != 0,
                    _ => false,
                } {
                    if value > &0 {
                        self.position += value.abs() as usize;
                    } else {
                        self.position -= value.abs() as usize;
                    }
                } else {
                    self.position += 1;
                }
                Some(self.registers.clone())
            }
            Some(Command::Copy(from, to)) => {
                let v = match from {
                    Source::Value(x) => x.clone(),
                    Source::Register('a') => self.registers.a,
                    Source::Register('b') => self.registers.b,
                    Source::Register('c') => self.registers.c,
                    Source::Register('d') => self.registers.d,
                    _ => 0,
                };
                match to.clone() as u8 {
                    b'a' => self.registers.a = v,
                    b'b' => self.registers.b = v,
                    b'c' => self.registers.c = v,
                    b'd' => self.registers.d = v,
                    _ => (),
                }
                self.position += 1;
                Some(self.registers.clone())
            }
            _ => None,
        }
    }
}

impl FromIterator<Command> for Programm {
    fn from_iter<I: IntoIterator<Item = Command>>(iter: I) -> Self {
        Programm {
            commands: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Clone)]
enum Source {
    Value(i32),
    Register(char),
}

#[derive(Debug, Clone)]
enum Command {
    Copy(Source, char),
    Inc(char),
    Dec(char),
    JumpNonZero(Source, i32),
}

impl FromStr for Command {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split(" ").collect::<Vec<&str>>();
        match parts[0] {
            "cpy" => {
                let source = parts[1].parse::<i32>();
                let source = if source.is_ok() {
                    Source::Value(source.unwrap())
                } else {
                    Source::Register(parts[1].as_bytes()[0] as char)
                };
                Ok(Command::Copy(source, parts[2].as_bytes()[0] as char))
            }
            "inc" => Ok(Command::Inc(parts[1].as_bytes()[0] as char)),
            "dec" => Ok(Command::Dec(parts[1].as_bytes()[0] as char)),
            "jnz" => {
                let source = parts[1].parse::<i32>();
                let source = if source.is_ok() {
                    Source::Value(source.unwrap())
                } else {
                    Source::Register(parts[1].as_bytes()[0] as char)
                };
                Ok(Command::JumpNonZero(
                    source,
                    parts[2].parse::<i32>().unwrap(),
                ))
            }
            _ => Err(()),
        }
    }
}

pub fn run() {
    let input = File::open("input/task_12").unwrap();
    let input = BufReader::new(input);

    let commands = input
        .lines()
        .filter_map(|l| l.ok())
        .filter_map(|l| l.parse::<Command>().ok())
        .collect::<Programm>();

    let result = commands.run().last().unwrap();

    println!("{}", result.a)
}

pub fn run_e() {
    let input = File::open("input/task_12").unwrap();
    let input = BufReader::new(input);

    let commands = input
        .lines()
        .filter_map(|l| l.ok())
        .filter_map(|l| l.parse::<Command>().ok())
        .collect::<Programm>();

    let result = commands.run_with_values(0, 0, 1, 0).last().unwrap();

    println!("{}", result.a)
}
