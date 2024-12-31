use core::panic;
use regex::Regex;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

mod part1;
mod part2;

fn parse_input(file_name: &str) -> Vec<(f64, f64)> {
    let file = File::open(file_name).unwrap();
    let file = BufReader::new(file);

    let re = Regex::new(r"X\+(\d+), Y\+(\d+)|X=(\d+), Y=(\d+)").unwrap();

    let input: Vec<_> = file
        .lines()
        .filter_map(|line| {
            let l = line.unwrap();
            if l.len() != 0 {
                let (_, [x, y]) = re.captures(&l).unwrap().extract();
                return Some((x.parse().unwrap(), y.parse().unwrap()));
            }
            None
        })
        .collect();

    input
}
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("Usage: program input.txt");
    }

    let machines = parse_input(&args[1]);

    part1::solve(&machines);
    part2::solve(&machines);
}
