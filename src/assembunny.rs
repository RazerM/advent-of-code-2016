#![allow(unreachable_pub)]

use std::collections::HashMap;

use super::parsers::int32;

use nom::anychar;

type Reg = char;

named!(reg<&str, Reg>,
    call!(anychar)
);

#[derive(Clone, Copy, Debug)]
pub enum Arg {
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

named!(pub instruction<&str, Instruction>,
    ws!(
        alt!(
            do_parse!(tag!("cpy") >> x: arg >> y: reg >> (Instruction::Cpy(x, y))) |
            do_parse!(tag!("inc") >> x: reg >> (Instruction::Inc(x))) |
            do_parse!(tag!("dec") >> x: reg >> (Instruction::Dec(x))) |
            do_parse!(tag!("jnz") >> x: arg >> y: arg >> (Instruction::Jnz(x, y))) |
            do_parse!(tag!("tgl") >> x: reg >> (Instruction::Tgl(x)))
        )
    )
);

#[derive(Clone, Debug)]
pub enum Instruction {
    Cpy(Arg, Reg),
    CpyInvalid(Arg, Arg),
    Inc(Reg),
    Dec(Reg),
    Jnz(Arg, Arg),
    Tgl(Reg),
    Add(Reg, Reg),
    Mul(Reg, Reg),
    Nop,
}

fn optimize_add(instructions: &mut [Instruction]) {
    for i in 2..instructions.len() {
        let inc_x = match instructions[i - 2] {
            Instruction::Inc(x) => x,
            _ => continue
        };

        let dec_x = match instructions[i - 1] {
            Instruction::Dec(x) => x,
            _ => continue
        };

        let (jnz_x, jnz_y) = match instructions[i] {
            Instruction::Jnz(Arg::Reg(x), Arg::Val(y)) => (x, y),
            _ => continue
        };

        if jnz_y != -2 || dec_x != jnz_x {
            continue;
        }

        instructions[i - 2] = Instruction::Add(dec_x, inc_x);
        instructions[i - 1] = Instruction::Cpy(Arg::Val(0), dec_x);
        instructions[i] = Instruction::Nop;
    }
}

fn optimize_mul(instructions: &mut [Instruction]) {
    for i in 5..instructions.len() {
        let (cpy_x, cpy_y) = match instructions[i - 5] {
            Instruction::Cpy(Arg::Reg(x), y) => (x, y),
            _ => continue
        };

        let inc_x = match instructions[i - 4] {
            Instruction::Inc(x) => x,
            _ => continue
        };

        let dec1_x = match instructions[i - 3] {
            Instruction::Dec(x) => x,
            _ => continue
        };

        let (jnz1_x, jnz1_y) = match instructions[i - 2] {
            Instruction::Jnz(Arg::Reg(x), Arg::Val(y)) => (x, y),
            _ => continue
        };

        let dec2_x = match instructions[i - 1] {
            Instruction::Dec(x) => x,
            _ => continue
        };

        let (jnz2_x, jnz2_y) = match instructions[i] {
            Instruction::Jnz(Arg::Reg(x), Arg::Val(y)) => (x, y),
            _ => continue
        };

        if jnz1_y != -2 || jnz2_y != -5 || jnz1_x != dec1_x || jnz2_x != dec2_x {
            continue;
        }

        instructions[i - 5] = Instruction::Mul(cpy_x, jnz2_x);
        instructions[i - 4] = Instruction::Add(jnz2_x, inc_x);
        instructions[i - 3] = Instruction::Cpy(Arg::Val(0), jnz2_x);
        instructions[i - 2] = Instruction::Cpy(Arg::Val(0), cpy_y);
        instructions[i - 1] = Instruction::Nop;
        instructions[i] = Instruction::Nop;
    }
}

pub fn run(mut instructions: Vec<Instruction>, registers: &mut HashMap<Reg, i32>) {
    let mut i = 0;

    optimize_mul(&mut instructions);
    optimize_add(&mut instructions);

    while let Some(instruction) = instructions.get(i as usize).cloned() {
        match instruction {
            Instruction::Cpy(ref x, ref y) => {
                *registers.entry(*y).or_insert(0) = x.value(registers);
            }
            Instruction::Inc(ref x) => {
                *registers.entry(*x).or_insert(0) += 1;
            }
            Instruction::Add(ref x, ref y) => {
                *registers.entry(*y).or_insert(0) += *registers.get(x).unwrap_or(&0);
            }
            Instruction::Mul(ref x, ref y) => {
                *registers.entry(*y).or_insert(0) *= *registers.get(x).unwrap_or(&0);
            }
            Instruction::Dec(ref x) => {
                *registers.entry(*x).or_insert(0) -= 1;
            }
            Instruction::Jnz(ref x, ref y) => {
                if x.value(registers) != 0 {
                    // -1 to account for normal i += 1
                    i += y.value(registers) - 1;
                }
            }
            Instruction::Tgl(ref x) => {
                let toggle_idx = (i + *registers.get(x).unwrap_or(&0)) as usize;
                if toggle_idx < instructions.len() {
                    match instructions[toggle_idx].clone() {
                        Instruction::Inc(p) => {
                            instructions[toggle_idx] = Instruction::Dec(p);
                        }
                        Instruction::Dec(p) | Instruction::Tgl(p) => {
                            instructions[toggle_idx] = Instruction::Inc(p);
                        }
                        Instruction::Jnz(p, q) => match q {
                            Arg::Reg(r) => {
                                instructions[toggle_idx] = Instruction::Cpy(p, r);
                            }
                            Arg::Val(_) => {
                                instructions[toggle_idx] = Instruction::CpyInvalid(p, q);
                            }
                        },
                        Instruction::Cpy(p, q) => {
                            instructions[toggle_idx] = Instruction::Jnz(p, Arg::Reg(q));
                        }
                        Instruction::CpyInvalid(p, q) => {
                            instructions[toggle_idx] = Instruction::Jnz(p, q);
                        }
                        Instruction::Nop
                        | Instruction::Add(_, _)
                        | Instruction::Mul(_, _) => {}
                    }
                }
            }
            Instruction::Nop | Instruction::CpyInvalid(_, _) => {}
        };
        i += 1;
    }
}
