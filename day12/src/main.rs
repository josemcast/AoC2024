use core::panic;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

mod part1;
mod part2;

fn parse_input(file_name: &str) -> Vec<Vec<char>> {
    let file = File::open(file_name).unwrap();
    let file = BufReader::new(file);

    file.lines()
        .map(|line| line.unwrap().chars().collect())
        .collect()
}
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("Usage program input.txt");
    }
    let garden_plots = parse_input(&args[1]);

    part1::solve(&garden_plots);
    part2::solve(&garden_plots);
}
