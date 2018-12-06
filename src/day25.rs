use std::io::{self, BufRead};

use maplit::hashmap;

use crate::assembunny;

pub(crate) fn solve() {
    let stdin = io::stdin();
    let instructions = stdin
        .lock()
        .lines()
        .map(|l| l.unwrap())
        .map(|l| assembunny::instruction(&l).to_result().unwrap())
        .collect::<Vec<_>>();

    for a in 1.. {
        let mut registers = hashmap!{'a' => a};
        if assembunny::run(instructions.clone(), &mut registers, Some(100)) {
            println!("Part 1: {}", a);
            break;
        }
    }
}
