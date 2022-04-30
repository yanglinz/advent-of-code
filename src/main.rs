use std::fs;
use std::path;

use clap::Parser;

mod year2015;

fn get_input() -> String {
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
    year: String,

    /// Advent calendar day
    #[clap(short, long)]
    day: String,
}

fn main() {
    let args = Args::parse();

    println!("{:?}", args);

    let input = get_input();
    let result = year2015::day3::part2(&input);
    println!("Result: {}", result);
}
