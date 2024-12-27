use std::fs::File;
use std::io::{BufRead, BufReader};

mod part1;
mod part2;

fn parse_input(file_name: &'static str) -> Vec<i64> {
    let file = File::open(file_name).unwrap();
    let file = BufReader::new(file);

    file.lines()
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .map(|number| number.parse().unwrap())
        .collect()
}

fn main() {
    let stones = parse_input("day11/input.txt");
    part1::solve(&stones);
    part2::solve(&stones);
}
