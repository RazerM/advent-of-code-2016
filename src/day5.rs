use crypto::md5::Md5;
use crypto::digest::Digest;
use hex;
use termion::clear;
use termion::cursor;

use super::util::stdin_as_string;

struct SimplePassword<'a> {
    key: &'a [u8],
    hasher: Md5,
    count: i32,
}

impl<'a> SimplePassword<'a> {
    fn new(door_id: &'a [u8]) -> Self {
        SimplePassword {
            key: door_id,
            hasher: Md5::new(),
            count: 0,
        }
    }
}

impl<'a> Iterator for SimplePassword<'a> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            self.hasher.input(self.key);
            self.hasher.input(self.count.to_string().as_bytes());

            let mut output = [0; 16];
            self.hasher.result(&mut output);
            self.hasher.reset();
            self.count += 1;

            if output[..2] == [0, 0] && output[2] <= 0x0F {
                return Some(hex::encode(&output).chars().nth(5).unwrap());
            }
        }
    }
}

struct AdvancedPassword<'a> {
    key: &'a [u8],
    hasher: Md5,
    count: i32,
}

impl<'a> AdvancedPassword<'a> {
    fn new(door_id: &'a [u8]) -> Self {
        AdvancedPassword {
            key: door_id,
            hasher: Md5::new(),
            count: 0,
        }
    }
}

impl<'a> Iterator for AdvancedPassword<'a> {
    type Item = (usize, char);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            self.hasher.input(self.key);
            self.hasher.input(self.count.to_string().as_bytes());

            let mut output = [0; 16];
            self.hasher.result(&mut output);
            self.hasher.reset();
            self.count += 1;

            if output[..2] == [0, 0] && output[2] <= 0x0F {
                let hash = hex::encode(&output);
                if let Some(pos) = hash.chars().nth(5).unwrap().to_digit(10) {
                    if pos < 8 {
                        let pchar = hash.chars().nth(6).unwrap();
                        return Some((pos as usize, pchar));
                    }
                }
            }
        }
    }
}

pub fn solve() {
    let door_id = stdin_as_string();

    let p = SimplePassword::new(door_id.as_bytes())
        .take(8)
        .collect::<String>();

    println!("Part 1: {}", p);

    let ap = AdvancedPassword::new(door_id.as_bytes());
    let mut adv_pass = [None; 8];

    println!("Part 2: ________");

    for (pos, pchar) in ap {
        if adv_pass[pos].is_none() {
            adv_pass[pos] = Some(pchar);
            println!(
                "{up}\r{clear}Part 2: {pass}",
                up = cursor::Up(1),
                clear = clear::AfterCursor,
                pass = adv_pass.iter().map(|c| c.unwrap_or('_')).collect::<String>());

            if adv_pass.iter().all(|c| c.is_some()) {
                break;
            }
        }
    }
}
