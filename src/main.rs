use std::collections::HashMap;
use std::fs;
use std::path;

use clap::Parser;

mod year2015;

fn get_input(year: u16, day: u8) -> String {
    let data_file = path::Path::new("./src/year2015/data/day3.txt");
    let content = fs::read_to_string(data_file);

    match content {
        Ok(str) => str,
        Err(e) => panic!("Could not read input: {:?}", e),
    }
}

/// Advent of code solutions
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Advent calendar year
    #[clap(short, long)]
    year: u16,

    /// Advent calendar day
    #[clap(short, long)]
    day: u8,
}

fn main() {
    let args = Args::parse();

    let year_2015_days = HashMap::from([
        (1, year2015::day1::part1),
        (2, year2015::day2::part2),
        (3, year2015::day3::part2),
    ]);

    // let mapping

    println!("{:?}", args);

    let input = get_input(2015, 1);
    let result = year2015::day3::part2(&input);
    println!("Result: {}", result);
}
