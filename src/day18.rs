use std::collections::VecDeque;

use super::util::stdin_as_string;

#[derive(Debug, PartialEq)]
enum Tile {
    Safe,
    Trap,
}

fn next_tile(left: &Tile, right: &Tile) -> Tile {
    let a = *left == Tile::Trap;
    let c: bool = *right == Tile::Trap;
    if !a && c || a && !c {
        Tile::Trap
    } else {
        Tile::Safe
    }
}

fn count_safe(first_row: &str, rows: i32) -> usize {
    let row = first_row.chars()
        .map(|c| match c {
            '.' => Tile::Safe,
            '^' => Tile::Trap,
            _ => panic!("Expected '^' or '.'")
        })
        .collect::<Vec<_>>();

    let mut safe = 0;
    let mut num_rows = 1;
    let mut tile_rows = VecDeque::new();
    tile_rows.push_front(row);

    while let Some(prev_row) = tile_rows.pop_back() {
        safe += prev_row.iter().filter(|&t| *t == Tile::Safe).count();

        if num_rows == rows { break; }

        let next_row = (0..prev_row.len())
            .map(|i| {
                let left = match i {
                    0 => &Tile::Safe,
                    _ => &prev_row[i - 1],
                };

                let right = match i {
                    x if x == prev_row.len() - 1 => &Tile::Safe,
                    _ => &prev_row[i + 1],
                };

                next_tile(left, right)
            })
            .collect::<Vec<_>>();
        num_rows += 1;
        tile_rows.push_front(next_row);
    }
    safe
}

pub fn solve() {
    let first_row = stdin_as_string();
    println!("Part 1: {}", count_safe(&first_row, 40));
    println!("Part 2: {}", count_safe(&first_row, 400_000));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let first_row = ".^^.^.^^^^";
        let safe = count_safe(first_row, 10);
        assert_eq!(safe, 38);
    }
}
