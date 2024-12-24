use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process;

use grid::Grid;

mod grid;
mod part1;
mod part2;

fn parse_input(file_path: &str) -> Grid {
    let file = File::open(file_path).unwrap();
    let file = BufReader::new(file);

    let grid: Vec<_> = file
        .lines()
        .map(|line| {
            let line = line.unwrap();
            line.chars().collect::<Vec<_>>()
        })
        .collect();

    let mut antennas = HashMap::new();
    for (i, row) in grid.iter().enumerate() {
        for (j, ch) in row.iter().enumerate() {
            if ch.is_alphanumeric() {
                antennas
                    .entry(*ch)
                    .or_insert(Vec::new())
                    .push((i as i32, j as i32));
            }
        }
    }

    Grid::new(antennas, grid.len(), grid[0].len())
}
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: program input.txt");
        process::exit(1);
    }

    let grid = parse_input(&args[1]);
    part1::solve(&grid);
    part2::solve(&grid);
}
