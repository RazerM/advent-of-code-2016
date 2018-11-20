use std::io::{self, BufRead};

use super::parsers::uint64;

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Range {
    // lazily use u64 because we use value + 1 on a u32
    low: u64,
    high: u64,
}

impl Range {
    fn new(low: u64, high: u64) -> Self {
        Range { low, high }
    }
}

named!(parse_range<&str, Range>,
    do_parse!(
        a: uint64 >>
        tag!("-") >>
        b: uint64 >>
        (Range::new(a, b))
    )
);

pub(crate) fn solve() {
    let stdin = io::stdin();
    let mut ranges = stdin.lock().lines()
        .map(|l| parse_range(&l.unwrap()).to_result().unwrap())
        .collect::<Vec<Range>>();
    ranges.sort();

    // Account for ranges where there isn't one up to u32::max_value()
    let max_high = ranges.iter().map(|r| r.high).max().unwrap();

    let mut allowed = u64::from(u32::max_value()) - max_high;
    // keep track of highest range seen so far
    let mut high = 0;
    let mut lowest: Option<u64> = None;
    for range in ranges {
        // This range leaves a gap after the highest range seen
        if range.low > high + 1 {
            if lowest.is_none() {
                lowest = Some(high + 1);
            }
            allowed += range.low - high - 1;
        }
        high = high.max(range.high)
    }
    println!("Part 1: {}", lowest.unwrap());
    println!("Part 2: {}",  allowed)
}
