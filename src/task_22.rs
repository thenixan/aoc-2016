use objects::Nodes;
use std::fs::File;
use std::io::{BufRead, BufReader};

mod objects {
    use regex::Regex;
    use std::collections::HashMap;
    use std::fmt::{Display, Formatter};
    use std::iter::FromIterator;

    #[derive(Hash, Eq, PartialEq)]
    pub struct NodePlacement {
        x: usize,
        y: usize,
    }

    impl NodePlacement {
        pub fn new(x: usize, y: usize) -> Self {
            NodePlacement { x, y }
        }
    }

    pub struct NodeMeta {
        used: usize,
        available: usize,
    }

    impl NodeMeta {
        pub fn new(used: usize, available: usize) -> Self {
            NodeMeta { used, available }
        }
        pub fn used(&self) -> usize {
            self.used
        }
        pub fn available(&self) -> usize {
            self.available
        }
    }
    pub struct Nodes(HashMap<NodePlacement, NodeMeta>);

    impl Nodes {
        pub fn width(&self) -> usize {
            self.0.iter().map(|(k, _)| k).map(|k| k.x).max().unwrap() + 1
        }
        pub fn height(&self) -> usize {
            self.0.iter().map(|(k, _)| k).map(|k| k.y).max().unwrap() + 1
        }

        pub fn get(&self, x: usize, y: usize) -> &NodeMeta {
            &self.0[&NodePlacement::new(x, y)]
        }

        pub fn empty(&self) -> (&NodePlacement, &NodeMeta) {
            self.0
                .iter()
                .find_map(|(k, v)| if v.used == 0 { Some((k, v)) } else { None })
                .unwrap()
        }
    }

    impl Display for Nodes {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            let (empty_pos, empty_meta) = self.empty();

            let mut s = String::new();
            for i in 0..self.height() {
                for j in 0..self.width() {
                    if self.get(j, i).used() == 0 {
                        s.push(' ');
                    } else if self.get(j, i).used() > empty_meta.available {
                        s.push('#');
                    } else {
                        s.push('.');
                    }
                }
                s.push('\n');
            }
            writeln!(f, "{}", s)
        }
    }

    impl FromIterator<String> for Nodes {
        fn from_iter<I: IntoIterator<Item = String>>(iter: I) -> Self {
            let pattern =
                Regex::new(r"^/dev/grid/node-x(\d+)-y(\d+)\s+(\d+)T\s+(\d+)T\s+(\d+)T\s+\d+%$")
                    .unwrap();
            let values = iter
                .into_iter()
                .map(|s| {
                    let caps = pattern.captures(s.as_str()).unwrap();
                    let x = caps[1].parse::<usize>().unwrap();
                    let y = caps[2].parse::<usize>().unwrap();
                    let u = caps[4].parse::<usize>().unwrap();
                    let a = caps[5].parse::<usize>().unwrap();
                    let node_placement = NodePlacement::new(x, y);
                    let node_meta = NodeMeta::new(u, a);
                    (node_placement, node_meta)
                })
                .collect();
            Nodes(values)
        }
    }
}

pub fn run() {
    let input = File::open("input/task_22").unwrap();
    let input = BufReader::new(input);

    let nodes = input
        .lines()
        .skip(2)
        .filter_map(|l| l.ok())
        .collect::<Nodes>();

    let mut result = 0;
    let width = nodes.width();
    let height = nodes.height();
    let size = width * height;
    for i in 0..size {
        for j in 0..size {
            let x_i = i % width;
            let y_i = i / width;
            let x_j = j % width;
            let y_j = j / width;
            if i != j
                && nodes.get(x_i, y_i).used() != 0
                && nodes.get(x_i, y_i).used() <= nodes.get(x_j, y_j).available()
            {
                result += 1;
            }
        }
    }

    println!("Result: {}", result);
}

pub fn run_e() {
    let input = File::open("input/task_22").unwrap();
    let input = BufReader::new(input);

    let nodes = input
        .lines()
        .skip(2)
        .filter_map(|l| l.ok())
        .collect::<Nodes>();

    println!("{}", nodes);
}
