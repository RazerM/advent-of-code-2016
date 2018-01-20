use std::io::{self, BufRead};

use ndarray::Array;

use super::parsers::int32;

#[derive(Debug)]
enum Instruction {
    Rect { width: i32, height: i32 },
    RotateRow { row: i32, shift: usize },
    RotateCol { col: i32, shift: usize },
}

named!(rect<&str, Instruction>,
    ws!(
        do_parse!(
            tag!("rect") >>
            width: int32 >>
            tag!("x") >>
            height: int32 >>
            (Instruction::Rect { width, height })
        )
    )
);

named!(rotate<&str, Instruction>,
    ws!(
        do_parse!(
            tag!("rotate") >>
            type_: alt!(tag!("row") | tag!("column")) >>
            one_of!("xy") >>
            tag!("=") >>
            row_or_col: int32 >>
            tag!("by") >>
            shift: int32 >>
            (match type_ {
                "row" => Instruction::RotateRow { row: row_or_col, shift: shift as usize },
                "column" => Instruction::RotateCol { col: row_or_col, shift: shift as usize },
                _ => unreachable!(),
            })
        )
    )
);

named!(parse_instruction<&str, Instruction>,
    alt!(rect | rotate)
);

pub fn solve() {
    let width = 50;
    let height = 6;
    let mut x = Array::<u8, _>::zeros((height, width));

    let stdin = io::stdin();
    let instructions = stdin.lock().lines()
        .map(|l| parse_instruction(&l.unwrap()).to_result().unwrap())
        .collect::<Vec<_>>();

    for instruction in instructions {
        match instruction {
            Instruction::Rect { width, height } => {
                x.slice_mut(s![..height, ..width]).fill(1);
            },
            Instruction::RotateRow { row, shift } => {
                x.slice_mut(s![row, ..]).as_slice_mut().unwrap().rotate_left(width - shift);
            },
            Instruction::RotateCol { col, shift } => {
                let mut sl = x.slice(s![.., col]).iter().cloned().collect::<Vec<_>>();
                sl.rotate_left(height - shift);
                x.slice_mut(s![.., col]).assign(&Array::from_vec(sl));
            },
        };
    }

    println!("Part 1: {}", x.iter().sum::<u8>());
    println!("Part 2:");
    x.genrows().into_iter()
        .map(|row| row.iter()
            .map(|i| if *i == 0 { ' ' } else { '#' })
            .collect::<String>()
        )
        .for_each(|s| println!("{}", s));
}
