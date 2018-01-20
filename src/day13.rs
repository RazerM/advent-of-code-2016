use std::collections::{HashSet, VecDeque};

use super::util::stdin_as_string;

type Point = (i32, i32);


struct Grid {
    fav: i32,
}

impl Grid {
    fn in_bounds(&self, pos: &Point) -> bool {
        let &(x, y) = pos;
        x >= 0 && y >= 0
    }

    fn is_wall(&self, pos: &Point) -> bool {
        let &(x, y) = pos;
        let n = x * x + 3 * x + 2 * x * y + y + y * y + self.fav;
        n.count_ones() % 2 != 0
    }

    fn neighbours(&self, pos: &Point) -> Vec<Point> {
        let &(x, y) = pos;
        let neighbours = [
            (x + 1, y),
            (x, y - 1),
            (x - 1, y),
            (x, y + 1),
        ];

        neighbours.iter()
            .cloned()
            .filter(|p| self.in_bounds(p))
            .filter(|p| !self.is_wall(p))
            .collect::<Vec<Point>>()
    }

    #[allow(dead_code)]
    fn draw(&self, to: Point) {
        let (width, height) = to;
        for y in 0..height {
            for x in 0..width {
                if self.is_wall(&(x, y)) {
                    print!("#")
                } else {
                    print!(".")
                }
            }
            println!();
        }
    }
}

fn find_goal(grid: &Grid, start: Point, goal: Point) -> (i32, i32) {
    let mut visited: HashSet<Point> = HashSet::new();
    let mut new = VecDeque::new();
    visited.insert(start);
    new.push_back(start);

    let mut steps = 0;
    let mut part1: Option<i32> = None;
    let mut part2: Option<i32> = None;

    while part1.is_none() || part2.is_none() {
        let mut open = new.clone();
        new = VecDeque::new();
        while let Some(current) = open.pop_front() {
            if current == goal {
                part1 = Some(steps)
            }
            for neighbour in grid.neighbours(&current) {
                if visited.insert(neighbour) {
                    new.push_back(neighbour);
                }
            }
        }
        steps += 1;
        if steps == 50 {
            part2 = Some(visited.len() as i32)
        }
    }
    (part1.unwrap(), part2.unwrap())
}

pub fn solve() {
    let fav = stdin_as_string().parse::<i32>().expect("an integer");
    let grid = Grid { fav };
//    grid.draw((32, 40));
    let (part1, part2) = find_goal(&grid, (1, 1), (31, 39));
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let grid = Grid { fav: 10 };
        let (fewest, _) = find_goal(&grid, (1, 1), (7, 4));
        assert_eq!(fewest, 11);
    }
}
