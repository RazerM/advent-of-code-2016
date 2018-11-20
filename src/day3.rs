use std::io::{self, BufRead};
use itertools::Itertools;

fn valid_triangle(x: &[i32; 3]) -> bool {
    x[0] + x[1] > x[2] &&
    x[0] + x[2] > x[1] &&
    x[1] + x[2] > x[0]
}

pub(crate) fn solve() {
    let stdin = io::stdin();
    let rows = stdin.lock().lines()
        .map(|l| {
            let x: Vec<i32> = l.unwrap()
                .split_whitespace()
                .map(|s| s.parse::<i32>().unwrap())
                .collect();

            assert_eq!(x.len(), 3, "Expected 3 numbers");

            let mut a = [0; 3];
            a.clone_from_slice(&x);
            a
        })
        .collect::<Vec<[i32; 3]>>();

    let possible1= rows.iter()
        .map(valid_triangle)
        .filter(|b| *b)
        .count();

    let possible2 = rows.iter()
        .chunks(3)
        .into_iter()
        .flat_map(|c| {
            let rows = c.collect::<Vec<&[i32; 3]>>();

            vec![
                [rows[0][0], rows[1][0], rows[2][0]],
                [rows[0][1], rows[1][1], rows[2][1]],
                [rows[0][2], rows[1][2], rows[2][2]],
            ]
        })
        .map(|t| valid_triangle(&t))
        .filter(|b| *b)
        .count();

    println!("Part 1: {}", possible1);
    println!("Part 2: {}", possible2);
}
