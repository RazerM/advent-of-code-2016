use std::io::{self, BufRead};

use nom::alpha;

struct SliceIter<'a> {
    s: &'a str,
    i: usize,
    len: usize,
}

impl<'a> SliceIter<'a> {
    fn new(s: &'a str, len: usize) -> Self {
        SliceIter { s, i: 0, len }
    }
}

impl<'a> Iterator for SliceIter<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        let lower = self.i;
        let upper = lower + self.len;

        if upper > self.s.len() {
            None
        } else {
            self.i += 1;
            Some(&self.s[lower..upper])
        }
    }
}

#[derive(Debug)]
struct Ip7 {
    parts: Vec<Ip7Part>,
}

#[derive(Debug)]
enum Ip7Part {
    Supernet(String),
    Hypernet(String),
}

fn str_is_abba(s: &str) -> bool {
    let a = s.chars().nth(0).unwrap();
    let b = s.chars().nth(1).unwrap();
    let ab = s.chars().take(2).collect::<String>();
    let ba = s.chars().rev().take(2).collect::<String>();

    a != b && ab == ba
}

fn str_is_aba(s: &str) -> bool {
    let a = s.chars().nth(0).unwrap();
    let b = s.chars().nth(1).unwrap();
    let c = s.chars().nth(2).unwrap();

    a != b && a == c
}

impl Ip7 {
    fn supernet_parts(&self) -> Vec<&str> {
        self.parts.iter()
            .filter_map(|p| match *p {
                Ip7Part::Supernet(ref s) => Some(s.as_str()),
                _ => None,
            })
            .collect()
    }

    fn hypernet_parts(&self) -> Vec<&str> {
        self.parts.iter()
            .filter_map(|p| match *p {
                Ip7Part::Hypernet(ref s) => Some(s.as_str()),
                _ => None,
            })
            .collect()
    }

    fn supports_tls(&self) -> bool {
        let supernet_has_abba = self.supernet_parts().iter()
            .any(|p| {
                SliceIter::new(p, 4).any(str_is_abba)
            });

        let hypernet_has_abba = self.hypernet_parts().iter()
            .any(|p| {
                SliceIter::new(p, 4).any(str_is_abba)
            });

        supernet_has_abba && !hypernet_has_abba
    }

    fn supports_ssl(&self) -> bool {
        self.supernet_parts().iter()
            .any(|p| SliceIter::new(p, 3)
                .filter(|aba| str_is_aba(*aba))
                .any(|aba| self.hypernet_parts().iter()
                    .any(|p| SliceIter::new(p, 3)
                        .filter(|s| str_is_aba(*s))
                        .map(|aba| {
                            let a = aba.chars().nth(0).unwrap();
                            let b = aba.chars().nth(1).unwrap();
                            format!("{b}{a}{b}", a = a, b = b)
                        })
                        .any(|bab| bab == aba)
                    )
                )
            )
    }
}

named!(parse_ip7<&str, Ip7>,
    map!(
        many1!(
            alt!(
                delimited!(tag!("["), alpha, tag!("]")) => { |s: &str|
                    Ip7Part::Hypernet(s.to_string())
                } |
                alpha => { |s: &str| Ip7Part::Supernet(s.to_string()) }
            )
        ),
        |v| Ip7 { parts: v }
    )
);

pub fn solve() {
    let stdin = io::stdin();
    let ip7s = stdin.lock().lines()
        .map(|l| parse_ip7(&l.unwrap()).to_result().unwrap())
        .collect::<Vec<Ip7>>();

    let num_tls = ip7s.iter()
        .filter(|ip7| ip7.supports_tls())
        .count();
    let num_ssl = ip7s.iter()
        .filter(|ip7| ip7.supports_ssl())
        .count();

    println!("Part 1: {}", num_tls);
    println!("Part 2: {}", num_ssl);
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! ip7 {
        ($p:expr) => (
            parse_ip7($p).to_result().unwrap();
        )
    }

    #[test]
    fn ip7_supports_tls() {
        assert_eq!(ip7!("abba[mnop]qrst").supports_tls(), true);
        assert_eq!(ip7!("abcd[bddb]xyyx").supports_tls(), false);
        assert_eq!(ip7!("aaaa[qwer]tyui").supports_tls(), false);
        assert_eq!(ip7!("ioxxoj[asdfgh]zxcvbn").supports_tls(), true);
    }

    #[test]
    fn ip7_supports_ssl() {
        assert_eq!(ip7!("aba[bab]xyz").supports_ssl(), true);
        assert_eq!(ip7!("xyx[xyx]xyx").supports_ssl(), false);
        assert_eq!(ip7!("aaa[kek]eke").supports_ssl(), true);
        assert_eq!(ip7!("zazbz[bzb]cdb").supports_ssl(), true);
    }
}
