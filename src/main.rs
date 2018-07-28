#![feature(slice_rotate)]

#[macro_use] extern crate clap;
extern crate crypto;
extern crate hex;
extern crate iterslide;
extern crate itertools;
#[macro_use] extern crate maplit;
#[macro_use] extern crate ndarray;
#[macro_use] extern crate nom;
extern crate rayon;
extern crate termion;

use clap::{Arg, App};

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod parsers;
mod util;

fn main() {
    let matches = App::new("aoc")
        .version(crate_version!())
        .author(crate_authors!(", "))
        .arg(Arg::with_name("day")
            .required(true)
            .help("Day of the advent calendar"))
        .get_matches();

    let day = value_t!(matches.value_of("day"), u32).unwrap_or_else(|e| e.exit());

    match day {
        1 => day1::solve(),
        2 => day2::solve(),
        3 => day3::solve(),
        4 => day4::solve(),
        5 => day5::solve(),
        6 => day6::solve(),
        7 => day7::solve(),
        8 => day8::solve(),
        9 => day9::solve(),
        10 => day10::solve(),
        11 => day11::solve(),
        12 => day12::solve(),
        13 => day13::solve(),
        14 => day14::solve(),
        15 => day15::solve(),
        16 => day16::solve(),
        17 => day17::solve(),
        18 => day18::solve(),
        19 => day19::solve(),
        day => println!("No solution found for day {}", day),
    }
}
