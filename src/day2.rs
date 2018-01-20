use std::fmt::Display;
use std::io::{self, BufRead};

use itertools::join;

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn find_code<T>(
    start_pos: (usize, usize),
    instructions: &[Vec<Direction>],
    keypad: &[Vec<Option<T>>],
) -> String
where
    T: Display + Clone
{
    let mut pos: (usize, usize) = start_pos;
    let mut result = Vec::with_capacity(instructions.len());

    for directions in instructions {
        for dir in directions {
            match *dir {
                Direction::Up => {
                    decrease(&mut pos.0);
                    if keypad[pos.0][pos.1].is_none() {
                        pos.0 += 1;
                    }
                },
                Direction::Down => {
                    increase(&mut pos.0, keypad.len() - 1);
                    if keypad[pos.0][pos.1].is_none() {
                        pos.0 -= 1;
                    }
                },
                Direction::Left => {
                    decrease(&mut pos.1);
                    if keypad[pos.0][pos.1].is_none() {
                        pos.1 += 1;
                    }
                }
                Direction::Right => {
                    increase(&mut pos.1, keypad[pos.0].len() - 1);
                    if keypad[pos.0][pos.1].is_none() {
                        pos.1 -= 1;
                    }
                }
            };
        }
        result.push(keypad[pos.0][pos.1].clone().unwrap());
    }
    join(result, "")
}

pub fn solve() {
    let stdin = io::stdin();
    let instructions = stdin.lock().lines()
        .map(|l| l.unwrap()
            .chars()
            .map(|c| match c {
                'U' => Direction::Up,
                'D' => Direction::Down,
                'L' => Direction::Left,
                'R' => Direction::Right,
                _ => panic!("Expected one of U, D, L, R"),
            })
            .collect()
        )
        .collect::<Vec<_>>();

    let keypad1 = vec![
        vec![Some(1), Some(2), Some(3)],
        vec![Some(4), Some(5), Some(6)],
        vec![Some(7), Some(8), Some(9)]
    ];

    let keypad2 = vec![
        vec![     None,      None, Some('1'),      None,      None],
        vec![     None, Some('2'), Some('3'), Some('4'),      None],
        vec![Some('5'), Some('6'), Some('7'), Some('8'), Some('9')],
        vec![     None, Some('A'), Some('B'), Some('C'),      None],
        vec![     None,      None, Some('D'),      None,      None],
    ];

    let result1 = find_code((1, 1), &instructions, &keypad1);
    let result2 = find_code((2, 0), &instructions, &keypad2);

    println!("Part 1: {}", result1);
    println!("Part 2: {}", result2);
}

#[inline]
fn increase(x: &mut usize, limit: usize) {
    if *x < limit {
        *x += 1
    }
}

#[inline]
fn decrease(x: &mut usize) {
    if *x > 0 {
        *x -= 1
    }
}
