use std::collections::HashMap;
use std::io::{self, BufRead};

pub(crate) fn solve() {
    let stdin = io::stdin();
    let messages = stdin.lock().lines()
        .map(|l| l.unwrap())
        .collect::<Vec<_>>();

    let message_length = messages.iter().map(|m| m.len()).max().unwrap();

    let mut counters: Vec<HashMap<char, usize>> = Vec::with_capacity(message_length);

    for _ in 0..message_length {
        counters.push(HashMap::new())
    }

    for message in messages {
        for (i, mchar) in message.chars().enumerate() {
            let count = counters[i].entry(mchar).or_insert(0);
            *count += 1;
        }
    }

    let repetition_message = counters.iter()
        .map(|counter| counter.iter()
            .max_by_key(|&(_, count)| count)
            .map(|(mchar, _)| mchar)
            .unwrap()
        )
        .collect::<String>();

    let mod_repetition_message = counters.iter()
        .map(|counter| counter.iter()
            .min_by_key(|&(_, count)| count)
            .map(|(mchar, _)| mchar)
            .unwrap()
        )
        .collect::<String>();

    println!("Part 1: {}", repetition_message);
    println!("Part 2: {}", mod_repetition_message);
}
