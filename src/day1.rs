use std::collections::HashSet;

use super::util::stdin_as_string;
use super::parsers::int32;

#[derive(Debug)]
enum Turn {
    Left,
    Right,
}

named!(turn<&str, Turn>,
    map!(one_of!("RL"), |c| match c {
        'R' => Turn::Left,
        'L' => Turn::Right,
        _ => unreachable!(),
    })
);

named!(parse_instructions<&str, Vec<(Turn, i32)>>,
    separated_nonempty_list_complete!(tag!(", "), tuple!(turn, int32))
);

pub(crate) fn solve() {
    let buffer = stdin_as_string();
    let instructions: Vec<(Turn, i32)> = parse_instructions(&buffer).to_result().unwrap();

    type Pos = (i32, i32);

    let mut direction = (0, 1);
    let mut position: Pos = (0, 0);

    let mut visited: HashSet<Pos> = HashSet::new();
    let mut found: Option<i32> = None;

    for (turn, distance) in instructions {
        direction = match turn {
            Turn::Left => (direction.1, -direction.0),
            Turn::Right => (-direction.1, direction.0),
        };

        for _ in 0..distance {
            position.0 += direction.0;
            position.1 += direction.1;

            if found.is_none() && !visited.insert(position) {
                found = Some(position.0.abs() + position.1.abs());
            }
        }
    }

    println!("Part 1: {}", position.0.abs() + position.1.abs());
    println!("Part 2: {:?}", found.expect("did not revisit any position"));
}
