use std::collections::HashMap;
use std::io::{self, BufRead};

use super::parsers::int32;

#[derive(Clone, Debug)]
enum Destination {
    Bot(i32),
    Output(i32),
}

#[derive(Clone, Debug)]
enum Instruction {
    Initial { value: i32, bot: i32 },
    Give { bot: i32, low: Destination, high: Destination }
}

named!(dest<&str, Destination>,
    ws!(
        do_parse!(
            type_: alt!(tag!("output") | tag!("bot")) >>
            num: int32 >>
            (
                match type_ {
                    "bot" => Destination::Bot(num),
                    "output" => Destination::Output(num),
                    _ => unreachable!(),
                }
            )
        )
    )
);

named!(parse_instruction<&str, Instruction>,
    ws!(
        alt!(
            do_parse!(
                tag!("value") >>
                value: int32 >>
                tag!("goes to bot") >>
                bot: int32 >>
                (Instruction::Initial { value, bot })
            ) |
            do_parse!(
                tag!("bot") >>
                bot: int32 >>
                tag!("gives low to") >>
                low: dest >>
                tag!("and high to") >>
                high: dest >>
                (Instruction::Give { bot, low, high })
            )
        )
    )
);

pub(crate) fn solve() {
    let stdin = io::stdin();
    let instructions = stdin.lock().lines()
        .map(|l| parse_instruction(&l.unwrap()).to_result().unwrap())
        .collect::<Vec<_>>();

    let mut bots: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut outputs: HashMap<i32, i32> = HashMap::new();
    let mut gives: HashMap<i32, Instruction> = HashMap::new();

    for instruction in instructions {
        match instruction {
            Instruction::Initial { value, bot } => {
                bots.entry(bot).or_insert_with(Vec::new).push(value);
            },
            Instruction::Give { ref bot, .. } => {
                gives.insert(*bot, instruction.clone());
            },
        }
    }


    let mut part1 = None;

    loop {
        {
            let res = bots.iter().find(|&(_, v)| {
                *v == vec![17, 61] || *v == vec![61, 17]
            });

            if let Some((bot, _)) = res {
                part1 = Some(bot.to_owned());
            }
        }

        let bot;
        let values;
        {
            let res = bots.iter_mut().find(|&(_, ref v)| v.len() == 2);
            if let Some((b, v)) = res {
                bot = *b;
                values = v.clone();
                v.clear();

            } else {
                break
            };
        }
        if let Instruction::Give { ref low, ref high, .. } = gives[&bot] {
            let min = values.iter().min().unwrap();
            let max = values.iter().max().unwrap();

            match *low {
                Destination::Bot(n) => {
                    bots.entry(n).or_insert_with(Vec::new).push(*min);;
                },
                Destination::Output(n) => {outputs.insert(n, *min);},
            }

            match *high {
                Destination::Bot(n) => {
                    bots.entry(n).or_insert_with(Vec::new).push(*max);
                },
                Destination::Output(n) => {outputs.insert(n, *max);},
            }
        }
    }

    println!("Part 1: {}", part1.unwrap());
    println!("Part 2: {}", outputs[&0] * outputs[&1] * outputs[&2]);
}
