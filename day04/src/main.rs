use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub mod part1;
pub mod part2;

fn parse_input(file_name: &str) -> Vec<Vec<char>> {
    let file = File::open(&file_name).unwrap();
    let file = BufReader::new(file);

    let mut grid: Vec<Vec<char>> = Vec::new();

    for line in file.lines() {
        let l = line.unwrap();
        if l != "" {
            grid.push(l.chars().collect());
        }
    }

    grid
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("Usage: program input.txt");
    }

    let grid = parse_input(&args[1]);

    part1::solve(&grid);
    part2::solve(&grid);
}
