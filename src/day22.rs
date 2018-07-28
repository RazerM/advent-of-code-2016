use std::io::{self, BufRead};

use itertools::Itertools;
use nom::{anychar, space};

use super::parsers::int32;

#[derive(Debug)]
struct Node {
    x: i32,
    y: i32,
    size: i32,
    used: i32,
}

impl Node {
    fn available(&self) -> i32 {
        self.size - self.used
    }
}

named!(node_line<&str, Node>,
    do_parse!(
        tag!("/dev/grid/node-x") >>
        x: int32 >>
        tag!("-y") >>
        y: int32 >>
        space >>
        size: int32 >>
        tag!("T") >>
        space >>
        used: int32 >>
        anychar >>
        (Node { x, y, size, used })
    )
);

fn is_viable(a: &Node, b: &Node) -> bool {
    a.used > 0 && a.used <= b.available()
}

pub fn solve() {
    let stdin = io::stdin();
    let nodes = stdin.lock().lines()
        .skip(2)
        .map(|l| node_line(&l.unwrap()).to_result().unwrap())
        .collect::<Vec<_>>();
    let viable = nodes.iter()
        .tuple_combinations::<(_, _)>()
        .filter(|&(a, b)| is_viable(a, b) || is_viable(b, a))
        .collect::<Vec<_>>();
    println!("Part 1: {}", viable.len());
}
