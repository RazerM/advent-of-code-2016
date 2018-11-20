use nom::anychar;

use super::util::stdin_as_string;
use super::parsers::int32;

#[derive(Debug)]
enum Part<'a> {
    Repeat { string: &'a str, repeat: usize },
    Normal(&'a str),
}

named!(repeat<&str, Part<'_>>,
    do_parse!(
        tag!("(") >>
        length: int32 >>
        tag!("x") >>
        repeat: int32 >>
        tag!(")") >>
        string: take!(length) >>
        (Part::Repeat { string, repeat: repeat as usize })
    )
);

named!(parse_compressed<&str, Vec<Part<'_>>>,
    many0!(
        alt_complete!(
            repeat |
            // There's probably a better way to do this.
            // If it's not a valid repeat, take until an opening bracket,
            take_until1!("(") => { |s| Part::Normal(s) } |
            // or if that fails (e.g. at EOF) any chars.
            recognize!(many1!(anychar)) => { |s| Part::Normal(s) }
        )
    )
);

fn expand(sequence: Vec<Part<'_>>) -> String {
    let mut out = String::new();
    for part in sequence {
        match part {
            Part::Normal(s) => out.push_str(s),
            Part::Repeat { string: s, repeat: r } => out.push_str(&s.repeat(r)),
        }
    }
    out
}

/// Recursively expands compressed input to its final length
fn expand_length(input: &str) -> usize {
    parse_compressed(input).to_result().unwrap().iter()
        .map(|p| match *p {
            Part::Normal(s) => s.len(),
            Part::Repeat { string: s, repeat: r } => expand_length(s) * r,
        })
        .sum()
}

pub(crate) fn solve() {
    let input = stdin_as_string();
    let seq = parse_compressed(&input).to_result().unwrap();
    println!("Part 1: {}", expand(seq).len());
    println!("Part 2: {}", expand_length(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! compressed {
        ($p:expr) => (
            parse_compressed($p).to_result().unwrap();
        )
    }

    #[test]
    fn test_expansion() {
        assert_eq!(expand(compressed!("ADVENT")), "ADVENT");
        assert_eq!(expand(compressed!("A(1x5)BC")), "ABBBBBC");
        assert_eq!(expand(compressed!("(3x3)XYZ")), "XYZXYZXYZ");
        assert_eq!(expand(compressed!("A(2x2)BCD(2x2)EFG")), "ABCBCDEFEFG");
        assert_eq!(expand(compressed!("(6x1)(1x3)A")), "(1x3)A");
        assert_eq!(expand(compressed!("X(8x2)(3x3)ABCY")), "X(3x3)ABC(3x3)ABCY");
    }
}

