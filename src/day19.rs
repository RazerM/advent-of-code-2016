use std::collections::VecDeque;
use std::ops::Rem;

use super::util::stdin_as_string;

fn is_power_of_3(mut n: i32) -> bool {
    while n.rem(3) == 0 {
        n /= 3;
    }
    n == 1
}

pub(crate) fn solve() {
    let num_elfs: i32 = stdin_as_string().parse().unwrap();

    // I spent far too long before drawing this on paper :-(
    let mut elfs = (1..=num_elfs).collect::<VecDeque<_>>();
    while elfs.len() > 1 {
        let cur = elfs.pop_front().unwrap();
        elfs.pop_front();
        elfs.push_back(cur);
    }
    println!("Part 1: {}", elfs[0]);

    // For part 2 I looked for the pattern on paper
    let mut gifted_elf = 0;
    let mut prev_power = 0;

    for i in 1..=num_elfs {
        if is_power_of_3(i) {
            gifted_elf = i;
            prev_power = i;
        } else if i <= prev_power * 2 {
            gifted_elf = i - prev_power;
        } else if i > prev_power * 2 {
            gifted_elf = 2 * i - 3 * prev_power;
        }
    }
    println!("Part 2: {}", gifted_elf);
}
