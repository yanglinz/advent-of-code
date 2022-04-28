use std::fs;
use std::path;

mod year2015;

fn get_input() -> String {
    let data_file = path::Path::new("./src/year2015/data/day1.txt");
    let content = fs::read_to_string(data_file);

    match content {
        Ok(str) => str,
        Err(e) => panic!("Could not read input: {:?}", e),
    }
}

fn main() {
    let input = get_input();
    let result = year2015::day1::part1(&input);
    println!("Result: {}", result);
}
