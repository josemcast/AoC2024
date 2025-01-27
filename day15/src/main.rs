use std::env;
use std::fs::File;
use std::io::{BufReader, Read};

mod part1;
mod part2;

fn parse_input(file_name: &str) -> (Vec<Vec<char>>, Vec<char>) {
    let file = File::open(file_name).unwrap();
    let mut file = BufReader::new(file);

    let mut buffer = String::new();
    file.read_to_string(&mut buffer).unwrap();

    let mut input_iter = buffer.split("\r\n\r\n");
    let warehouse = input_iter.next().unwrap().to_string();
    let moves = input_iter.next().unwrap().to_string();

    let warehouse: Vec<Vec<char>> = warehouse
        .split_whitespace()
        .map(|line| line.chars().collect())
        .collect();

    let moves: Vec<char> = moves
        .split_whitespace()
        .collect::<String>()
        .chars()
        .collect();

    (warehouse, moves)
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("Usage: program input.txt");
    }

    let (mut map, moves) = parse_input(&args[1]);
    let mut new_map = map.clone();
    part1::solve(&mut map, &moves);
    part2::solve(&mut new_map, &moves);
}
