use std::fmt::Display;
use std::fs::File;
use std::io::Read;

struct Key {
    password: String,
    width: i32,
    height: i32,
}

impl Key {
    fn new(password: String, width: i32, height: i32) -> Self {
        Key {
            password,
            width,
            height,
        }
    }

    fn next_steps(&self, path: &Path) -> Vec<Step> {
        let mut result = vec![];
        let (x, y) = path.position();
        let password = format!("{}{}", self.password, path.to_string());
        let digest = format!("{:x}", md5::compute(password));
        if ('b'..='f').contains(&digest.chars().nth(0).unwrap())
            && ((y != self.height && self.height > 0) || (y != 0 && self.height < 0))
        {
            result.push(Step::Up);
        }
        if ('b'..='f').contains(&digest.chars().nth(1).unwrap())
            && ((y != 0 && self.height > 0) || (y != self.height && self.height < 0))
        {
            result.push(Step::Down);
        }
        if ('b'..='f').contains(&digest.chars().nth(2).unwrap())
            && ((x != 0 && self.width > 0) || (x != self.width && self.width < 0))
        {
            result.push(Step::Left);
        }
        if ('b'..='f').contains(&digest.chars().nth(3).unwrap())
            && ((x != self.width && self.width > 0) || (x != 0 && self.width < 0))
        {
            result.push(Step::Right);
        }
        result
    }

    fn is_finished(&self, path: &Path) -> bool {
        let (x, y) = path.position();
        x == self.width && y == self.height
    }
}

#[derive(Clone)]
enum Step {
    Up,
    Down,
    Left,
    Right,
}

impl Step {
    fn apply(&self, x: &mut i32, y: &mut i32) {
        match self {
            Step::Up => *y += 1,
            Step::Down => *y -= 1,
            Step::Right => *x += 1,
            Step::Left => *x -= 1,
        }
    }
}

#[derive(Clone)]
struct Path(Vec<Step>);

impl Path {
    fn new() -> Self {
        Path(vec![])
    }

    fn position(&self) -> (i32, i32) {
        self.0
            .iter()
            .fold((0, 0), |(mut x, mut y): (i32, i32), step: &Step| {
                step.apply(&mut x, &mut y);
                (x, y)
            })
    }

    fn move_to(&mut self, step: Step) {
        self.0.push(step);
    }
}

impl Display for Path {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let result = self
            .0
            .iter()
            .map(|step| match step {
                Step::Up => 'U',
                Step::Down => 'D',
                Step::Left => 'L',
                Step::Right => 'R',
            })
            .collect::<String>();
        write!(f, "{}", result)
    }
}

pub fn run() {
    let mut input = File::open("input/task_17").unwrap();
    let mut password = String::new();

    input.read_to_string(&mut password).unwrap();
    let key = Key::new(password, 3, -3);

    let mut steps = vec![Path::new()];

    loop {
        let result = steps.iter().find(|path: &&Path| key.is_finished(path));
        match result {
            Some(result) => {
                println!("Result: {}", result);
                break;
            }
            None => (),
        };
        steps = steps
            .into_iter()
            .flat_map(|path| {
                let next_steps = key.next_steps(&path);
                let mut result = vec![];
                for step in next_steps {
                    let mut p = path.clone();
                    p.move_to(step);
                    result.push(p);
                }
                result
            })
            .collect();
    }
}

pub fn run_e() {
    let mut input = File::open("input/task_17").unwrap();
    let mut password = String::new();

    input.read_to_string(&mut password).unwrap();
    let key = Key::new(password, 3, -3);

    let mut steps = vec![Path::new()];

    let mut i = 0;

    let mut latest = 0;

    loop {
        if steps.is_empty() {
            println!("Result: {}", latest);
            break;
        }
        steps = steps
            .into_iter()
            .inspect(|step| {
                if key.is_finished(&step) {
                    latest = i;
                }
            })
            .filter(|step| !key.is_finished(&step))
            .collect();
        steps = steps
            .into_iter()
            .flat_map(|path| {
                let next_steps = key.next_steps(&path);
                let mut result = vec![];
                for step in next_steps {
                    let mut p = path.clone();
                    p.move_to(step);
                    result.push(p);
                }
                result
            })
            .collect();
        i += 1;
    }
}
