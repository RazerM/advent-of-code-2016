use std::io::{self, BufRead};
use std::ops::Rem;

use nom::digit;

use super::parsers::int_usize;

// lazy parser, input is ordered and time always == 0
named!(parse_disc<&str, Disc>,
    ws!(
        do_parse!(
            tag!("Disc #") >>
            digit >>
            tag!("has") >>
            positions: int_usize >>
            tag!("positions;") >>
            tag!("at time=0, it is at position") >>
            initial: int_usize >>
            tag!(".") >>
            (Disc { positions, initial })
        )
    )
);

#[derive(Debug)]
struct Disc {
    positions: usize,
    initial: usize,
}

fn find_button_time(discs: &[Disc]) -> Option<usize> {
    (0..).into_iter()
        .find(|t| {
            discs.iter()
                .enumerate()
                .map(|(i, disc)| {
                    (t + disc.initial + i + 1).rem(disc.positions)
                })
                .all(|pos| pos == 0)
        })
}

pub fn solve() {
    let stdin = io::stdin();
    let mut discs = stdin.lock().lines()
        .map(|l| {
            parse_disc(&l.unwrap()).to_result().unwrap()
        })
        .collect::<Vec<Disc>>();

    println!("Part 1: {}", find_button_time(&discs).unwrap());
    discs.push(Disc { positions: 11, initial: 0 });
    println!("Part 2: {}", find_button_time(&discs).unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let discs = vec![
            Disc { positions: 5, initial: 4 },
            Disc { positions: 2, initial: 1 },
        ];
        assert_eq!(find_button_time(&discs).unwrap(), 5);
    }
}
