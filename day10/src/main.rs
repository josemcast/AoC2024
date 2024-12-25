use std::fs::File;
use std::io::{BufRead, BufReader};

mod part1;
mod part2;

fn parse_input(file_path: &'static str) -> Vec<Vec<i32>> {
    let file = File::open(file_path).unwrap();
    let file = BufReader::new(file);

    file.lines().fold(Vec::new(), |mut acc, line| {
        acc.push(
            line.unwrap()
                .chars()
                .map(|c| c.to_digit(10).unwrap() as i32)
                .collect::<Vec<_>>(),
        );
        acc
    })
}

fn main() {
    let topo_map = parse_input("day10/input.txt");

    part1::solve(&topo_map);
    part2::solve(&topo_map);
}
