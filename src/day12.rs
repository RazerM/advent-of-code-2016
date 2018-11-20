use std::collections::HashMap;
use std::io::{self, BufRead};

use super::parsers::int32;

use maplit::hashmap;
use nom::anychar;

type Reg = char;

named!(reg<&str, Reg>,
    call!(anychar)
);

#[derive(Clone, Copy, Debug)]
enum Arg {
    Reg(Reg),
    Val(i32),
}

impl Arg {
    fn value(self, regs: &HashMap<Reg, i32>) -> i32 {
        match self {
            Arg::Reg(ref r) => *regs.get(r).unwrap_or(&0),
            Arg::Val(v) => v,
        }
    }
}

named!(arg<&str, Arg>,
    alt!(
        int32 => { |n| Arg::Val(n) } |
        reg => { |c| Arg::Reg(c) }
    )
);

named!(instruction<&str, Instruction>,
    ws!(
        alt!(
            do_parse!(tag!("cpy") >> x: arg >> y: reg >> (Instruction::Cpy(x, y))) |
            do_parse!(tag!("inc") >> x: reg >> (Instruction::Inc(x))) |
            do_parse!(tag!("dec") >> x: reg >> (Instruction::Dec(x))) |
            do_parse!(tag!("jnz") >> x: arg >> y: arg >> (Instruction::Jnz(x, y)))
        )
    )
);

#[derive(Debug)]
enum Instruction {
    Cpy(Arg, Reg),
    Inc(Reg),
    Dec(Reg),
    Jnz(Arg, Arg),
}

fn run(instructions: &[Instruction], registers: &mut HashMap<Reg, i32>) {
    let mut i = 0;

    while let Some(instruction) = instructions.get(i as usize) {
        match *instruction {
            Instruction::Cpy(ref x, ref y) => {
                *registers.entry(*y).or_insert(0) = x.value(registers);
            },
            Instruction::Inc(ref x) => {
                *registers.entry(*x).or_insert(0) += 1;
            },
            Instruction::Dec(ref x) => {
                *registers.entry(*x).or_insert(0) -= 1;
            },
            Instruction::Jnz(ref x, ref y) => {
                if x.value(registers) != 0 {
                    i += y.value(registers) - 1;
                }
            },
        };
        i += 1;
    }
}

pub fn solve() {
    let stdin = io::stdin();
    let instructions = stdin.lock().lines()
        .map(|l| l.unwrap())
        .map(|l| instruction(&l).to_result().unwrap())
        .collect::<Vec<_>>();


    let mut registers1 = HashMap::new();
    let mut registers2 = hashmap!{'c' => 1};

    run(&instructions, &mut registers1);
    println!("Part 1: {}", &registers1[&'a']);

    run(&instructions, &mut registers2);
    println!("Part 2: {}", &registers2[&'a']);
}
