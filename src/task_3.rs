use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

struct Triangle {
    _s1: u16,
    _s2: u16,
    _l: u16,
}

impl Triangle {
    fn new_from_num(i1: u16, i2: u16, i3: u16) -> Result<Self, ()> {
        let mut v = vec![i1, i2, i3];
        v.sort();

        if v[2] >= v[0] + v[1] {
            Err(())
        } else {
            Ok(Triangle {
                _s1: v[0],
                _s2: v[1],
                _l: v[2],
            })
        }
    }
}

impl FromStr for Triangle {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s1 = s[0..5].trim().parse::<u16>();
        let s2 = s[5..10].trim().parse::<u16>();
        let s3 = s[10..].trim().parse::<u16>();

        if s1.is_err() || s2.is_err() || s3.is_err() {
            Err(())
        } else {
            let s1 = s1.unwrap();
            let s2 = s2.unwrap();
            let s3 = s3.unwrap();

            Triangle::new_from_num(s1, s2, s3)
        }
    }
}

pub fn run() {
    let input = File::open("input/task_3").unwrap();
    let input = BufReader::new(input);

    let commands: Vec<Triangle> = input
        .lines()
        .filter_map(|l| l.ok())
        .filter_map(|s| s.parse::<Triangle>().ok())
        .collect();

    println!("Result: {}", commands.len());
}

pub fn run_e() {
    let input = File::open("input/task_3").unwrap();
    let input = BufReader::new(input);

    let commands: Vec<Vec<u16>> = input
        .lines()
        .filter_map(|l| l.ok())
        .map(|s| {
            vec![
                s[0..5].trim().parse::<u16>().unwrap(),
                s[5..10].trim().parse::<u16>().unwrap(),
                s[10..15].trim().parse::<u16>().unwrap(),
            ]
        })
        .collect();

    let mut result = vec![];

    for i in 0..commands.len() / 3 {
        let p = i * 3;
        let l1 = &commands[p];
        let l2 = &commands[p + 1];
        let l3 = &commands[p + 2];

        if let Ok(t) = Triangle::new_from_num(l1[0], l2[0], l3[0]) {
            result.push(t);
        }

        if let Ok(t) = Triangle::new_from_num(l1[1], l2[1], l3[1]) {
            result.push(t);
        }

        if let Ok(t) = Triangle::new_from_num(l1[2], l2[2], l3[2]) {
            result.push(t);
        }
    }

    println!("Result: {}", result.len());
}
