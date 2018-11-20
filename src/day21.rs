use std::io::{self, BufRead};

use nom::{anychar, space};
use permutohedron::Heap;

use super::parsers::int_usize;

#[derive(Debug)]
enum Instruction {
    SwapPos(usize, usize),
    SwapLetter(char, char),
    RotateLeft(usize),
    RotateRight(usize),
    RotateLetter(char),
    Reverse(usize, usize),
    Move(usize, usize),
}

named!(swap_pos<&str, Instruction>,
    do_parse!(
        tag!("swap position ") >>
        x: int_usize >>
        tag!(" with position ") >>
        y: int_usize >>
        (Instruction::SwapPos(x, y))
    )
);

named!(swap_letter<&str, Instruction>,
    do_parse!(
        tag!("swap letter ") >>
        x: anychar >>
        tag!(" with letter ") >>
        y: anychar >>
        (Instruction::SwapLetter(x, y))
    )
);

named!(rotate<&str, Instruction>,
    do_parse!(
        tag!("rotate ") >>
        dir: alt!(tag!("left") | tag!("right")) >>
        space >>
        x: int_usize >>
        alt!(tag!(" step") | tag!(" steps")) >>
        (
            match dir {
                "left" => Instruction::RotateLeft(x),
                "right" => Instruction::RotateRight(x),
                _ => unreachable!("expected left or right"),
            }
        )
    )
);

named!(rotate_letter<&str, Instruction>,
    do_parse!(
        tag!("rotate based on position of letter ") >>
        x: anychar >>
        (Instruction::RotateLetter(x))
    )
);

named!(reverse<&str, Instruction>,
    do_parse!(
        tag!("reverse positions ") >>
        x: int_usize >>
        tag!(" through ") >>
        y: int_usize >>
        (Instruction::Reverse(x, y))
    )
);

named!(move_pos<&str, Instruction>,
    do_parse!(
        tag!("move position ") >>
        x: int_usize >>
        tag!(" to position ") >>
        y: int_usize >>
        (Instruction::Move(x, y))
    )
);

named!(parse_instruction<&str, Instruction>,
    alt!(
        swap_pos |
        swap_letter |
        rotate |
        rotate_letter |
        reverse |
        move_pos
    )
);

fn scramble(mut pchars: Vec<char>, instructions: &[Instruction]) -> String {
    for instruction in instructions {
        match *instruction {
            Instruction::SwapPos(x, y) => pchars.swap(x, y),
            Instruction::SwapLetter(x, y) => {
                let x_pos = pchars.iter().position(|&c| c == x).unwrap();
                let y_pos = pchars.iter().position(|&c| c == y).unwrap();
                pchars.swap(x_pos, y_pos);
            },
            Instruction::RotateLeft(x) => pchars.rotate_left(x),
            Instruction::RotateRight(x) => pchars.rotate_right(x),
            Instruction::RotateLetter(x) => {
                let x_pos = pchars.iter().position(|&c| c == x).unwrap();
                pchars.rotate_right(1);
                pchars.rotate_right(x_pos);
                if x_pos >= 4 {
                    pchars.rotate_right(1);
                }
            },
            Instruction::Reverse(x, y) => {
                let revslice = pchars[x..=y].iter().rev().cloned().collect::<Vec<_>>();
                pchars.splice(x..=y, revslice);
            },
            Instruction::Move(x, y) => {
                let x_char = pchars.remove(x);
                pchars.insert(y, x_char);
            }
        }
    }
    pchars.iter().collect()
}

pub(crate) fn solve() {
    let stdin = io::stdin();
    let instructions = stdin.lock().lines()
        .map(|l| parse_instruction(&l.unwrap()).to_result().unwrap())
        .collect::<Vec<_>>();

    let password = "abcdefgh";
    println!("Part 1: {}", scramble(password.chars().collect::<Vec<_>>(), &instructions));

    let mut pchars = password.chars().collect::<Vec<_>>();
    let heap = Heap::new(&mut pchars);
    for data in heap {
        let result = scramble(data.clone(), &instructions);
        if result == "fbgdceah" {
            println!("Part 2: {}", data.iter().collect::<String>());
            break;
        }
    }
}
