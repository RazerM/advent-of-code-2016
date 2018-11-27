use std::collections::HashMap;
use std::io::{self, BufRead};

use assembunny;

use maplit::hashmap;

pub(crate) fn solve() {
    let stdin = io::stdin();
    let instructions = stdin.lock().lines()
        .map(|l| l.unwrap())
        .map(|l| assembunny::instruction(&l).to_result().unwrap())
        .collect::<Vec<_>>();

    let mut registers1 = HashMap::new();
    let mut registers2 = hashmap!{'c' => 1};

    assembunny::run(instructions.clone(), &mut registers1, None);
    println!("Part 1: {}", &registers1[&'a']);

    assembunny::run(instructions, &mut registers2, None);
    println!("Part 2: {}", &registers2[&'a']);
}
