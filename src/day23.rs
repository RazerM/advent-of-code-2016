use std::io::{self, BufRead};

use maplit::hashmap;

use assembunny;

pub(crate) fn solve() {
    let stdin = io::stdin();
    let instructions = stdin.lock().lines()
        .map(|l| l.unwrap())
        .map(|l| assembunny::instruction(&l).to_result().unwrap())
        .collect::<Vec<_>>();

    let mut registers1 = hashmap!{'a' => 7};
    let mut registers2 = hashmap!{'a' => 12};

    assembunny::run(instructions.clone(), &mut registers1, None);
    println!("Part 1: {}", &registers1[&'a']);

    assembunny::run(instructions, &mut registers2, None);
    println!("Part 2: {}", &registers2[&'a']);
}
