use std::collections::HashMap;
use std::io::{self, BufRead};

use itertools::{Itertools, join};
use nom::alpha;

use super::parsers::int32;

const LETTERS: &str = "abcdefghijklmnopqrstuvwxyz";

named!(parse_room<&str, Room>,
    do_parse!(
        name: separated_nonempty_list_complete!(tag!("-"), alpha) >>
        tag!("-") >>
        sector_id: int32 >>
        checksum: delimited!(tag!("["), alpha, tag!("]")) >>
        (
            Room {
                name: name.into_iter().map(String::from).collect(),
                sector_id,
                checksum: checksum.to_string(),
            }
        )
    )
);

#[derive(Debug)]
struct Room {
    name: Vec<String>,
    sector_id: i32,
    checksum: String,
}

impl Room {
    fn calculate_checksum(&self) -> String {
        let mut counter: HashMap<char, usize> = HashMap::new();

        for part in &self.name {
            for c in part.chars() {
                let counter = counter.entry(c).or_insert(0);
                *counter += 1;
            }
        }

        let mut items = counter.iter().collect::<Vec<(&char, &usize)>>();
        items.sort_by_key(|x| x.1);
        items.reverse();

        let mut checksum = String::new();

        'outer: for (_, group) in &items.iter().group_by(|x| x.1) {
            let mut chars = group.map(|x| x.0).collect::<Vec<_>>();
            chars.sort();
            for c in chars {
                checksum.push(*c);
                if checksum.len() == 5 {
                    break 'outer
                }
            }
        }
        checksum
    }

    fn decrypt(&self) -> Vec<String> {
        self.name.iter()
            .map(|part| {
                part.chars()
                    .map(|c| {
                        let i = LETTERS.find(c).expect("char to be in letters");
                        LETTERS.chars().nth((i + self.sector_id as usize) % 26).unwrap()
                    })
                    .collect::<String>()
            })
            .collect()
    }
}

pub(crate) fn solve() {
    let stdin = io::stdin();
    let valid_rooms = stdin.lock().lines()
        .map(|l| parse_room(&l.unwrap()).to_result().unwrap())
        .filter(|r| r.calculate_checksum() == r.checksum)
        .collect::<Vec<Room>>();

    let sector_id_sum = valid_rooms.iter()
        .map(|r| r.sector_id)
        .sum::<i32>();

    println!("Part 1: {}", sector_id_sum);

    let northpole_sector_id = valid_rooms.iter()
        .filter_map(|r| {
            if join(r.decrypt(), " ") == "northpole object storage" {
                Some(r.sector_id)
            } else {
                None
            }
        })
        .take(1)
        .next()
        .expect("did not find \"northpole object storage\"");

    println!("Part 2: {}", northpole_sector_id);
}
