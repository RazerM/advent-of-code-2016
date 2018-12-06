#![deny(bare_trait_objects)]
#![deny(elided_lifetimes_in_paths)]
#![deny(missing_copy_implementations)]
#![deny(missing_debug_implementations)]
#![deny(trivial_casts)]
#![deny(trivial_numeric_casts)]
#![deny(unreachable_pub)]
#![deny(unused_extern_crates)]
#![deny(unused_import_braces)]
#![deny(unused_qualifications)]

#[macro_use] extern crate nom;

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
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;
mod assembunny;
mod parsers;
mod util;

fn main() {
    let matches = App::new("aoc")
        .version(clap::crate_version!())
        .author(clap::crate_authors!(", "))
        .arg(Arg::with_name("day")
            .required(true)
            .help("Day of the advent calendar"))
        .get_matches();

    let day = clap::value_t!(matches.value_of("day"), u32)
        .unwrap_or_else(|e| e.exit());

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
        20 => day20::solve(),
        21 => day21::solve(),
        22 => day22::solve(),
        23 => day23::solve(),
        24 => day24::solve(),
        25 => day25::solve(),
        day => println!("No solution found for day {}", day),
    }
}
