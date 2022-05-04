use std::fs;
use std::path;

use clap::Parser;

mod year2015;

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
    let year = args.year;
    let day = args.day;

    fn get_input(year: u16, day: u8) -> String {
        let year_dir = format!("year{}", year);
        let day_file = format!("day{}.txt", day);
        let input_path = path::Path::new("./src")
            .join(year_dir)
            .join("data")
            .join(day_file);
        let content = fs::read_to_string(input_path);
        match content {
            Ok(s) => s,
            Err(e) => panic!("Could not read input: {:?}", e),
        }
    }

    let input = get_input(year, day);
    let results = match (year, day) {
        (2015, 1) => (
            year2015::day1::part1(&input).to_string(),
            year2015::day1::part2(&input).to_string(),
        ),
        (2015, 2) => (
            year2015::day2::part1(&input).to_string(),
            year2015::day2::part2(&input).to_string(),
        ),
        (2015, 3) => (
            year2015::day3::part1(&input).to_string(),
            year2015::day3::part2(&input).to_string(),
        ),
        (2015, 4) => (
            year2015::day4::part1(&input).to_string(),
            year2015::day4::part2(&input).to_string(),
        ),
        (2015, 5) => (
            year2015::day5::part1(&input).to_string(),
            year2015::day5::part2(&input).to_string(),
        ),
        _ => panic!("Wrong input"),
    };

    println!("Part1 Results: {}", results.0);
    println!("Part1 Results: {}", results.1);
}
