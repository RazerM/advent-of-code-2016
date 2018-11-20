use std::collections::VecDeque;
use std::fmt;

use crypto::md5::Md5;
use crypto::digest::Digest;
use maplit::hashset;

use super::util::stdin_as_string;

type Pos = (i32, i32);

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        let l = match *self {
            Direction::Up => "U",
            Direction::Down => "D",
            Direction::Left => "L",
            Direction::Right => "R",
        };
        write!(f, "{}", l)
    }
}

fn get_possible_moves(size: Pos, passcode: &str, state: &State) -> Vec<Direction> {
    let open_chars = hashset!{'b', 'c', 'd', 'e', 'f'};

    let &State { ref path, pos } = state;

    let mut hasher = Md5::new();
    hasher.input_str(passcode);
    hasher.input_str(path);
    let mut output = [0; 16];
    hasher.result(&mut output);
    let hash = hasher.result_str();

    hash.chars()
        .take(4)
        .enumerate()
        .filter(|&(_, c)| open_chars.contains(&c))
        .map(|(i, _)| match i {
            0 => Direction::Up,
            1 => Direction::Down,
            2 => Direction::Left,
            3 => Direction::Right,
            _ => unreachable!(),
        })
        .filter(|d| match *d {
            Direction::Up => pos.1 < size.1 - 1,
            Direction::Down => pos.1 > 0,
            Direction::Left => pos.0 > 0,
            Direction::Right => pos.0 < size.0 - 1,
        })
        .collect::<Vec<_>>()
}

#[derive(Debug)]
struct State {
    path: String,
    pos: Pos,
}

fn find_paths(passcode: &str) -> Option<(String, usize)> {
    let size = (4, 4);

    let mut queue = VecDeque::new();

    let state = State {
        path: "".to_string(),
        pos: (0, 3)
    };

    queue.push_back(state);

    let goal = (3, 0);
    let mut shortest_path: Option<String> = None;
    let mut longest_path: Option<usize> = None;

    while let Some(state) = queue.pop_front() {
        let State { ref path, ref pos } = state;
        let possible_moves = get_possible_moves(size, passcode, &state);

        for move_ in possible_moves {
            let new_pos = match move_ {
                Direction::Up => (pos.0, pos.1 + 1),
                Direction::Down => (pos.0, pos.1 - 1),
                Direction::Left => (pos.0 - 1, pos.1),
                Direction::Right => (pos.0 + 1, pos.1),
            };

            let mut new_path = path.to_owned();
            new_path.push_str(&format!("{}", move_));

            if new_pos == goal {
                longest_path = Some(new_path.len());
                if shortest_path.is_none() {
                    shortest_path = Some(new_path);
                }
            } else {
                let new_state = State { path: new_path, pos: new_pos };
                queue.push_back(new_state);
            }
        }
    }

    if let Some(short) = shortest_path {
        Some((short, longest_path.unwrap()))
    } else {
        None
    }
}

pub(crate) fn solve() {
    let passcode = stdin_as_string();

    let (shortest_path, longest_path) = find_paths(&passcode).unwrap();

    println!("Part 1: {}", shortest_path);
    println!("Part 2: {}", longest_path);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let (shortest_path, longest_path) = find_paths("ihgpwlah").unwrap();
        assert_eq!(&shortest_path, "DDRRRD");
        assert_eq!(longest_path, 370);
    }

    #[test]
    fn example2() {
        let (shortest_path, longest_path) = find_paths("kglvqrro").unwrap();
        assert_eq!(&shortest_path, "DDUDRLRRUDRD");
        assert_eq!(longest_path, 492);
    }

    #[test]
    fn example3() {
        let (shortest_path, longest_path) = find_paths("ulqzkmiv").unwrap();
        assert_eq!(&shortest_path, "DRURDRUDDLLDLUURRDULRLDUUDDDRR");
        assert_eq!(longest_path, 830);
    }
}
