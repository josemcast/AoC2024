use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process;

pub mod part1;
pub mod part2;

fn parse_input(args: &str) -> Vec<(i64, Vec<i64>)> {
    let file = File::open(&args).unwrap();
    let file = BufReader::new(file);

    file.lines()
        .map(|line| {
            let inner: Vec<String> = line
                .unwrap()
                .split(":")
                .map(|ele| String::from(ele))
                .collect();
            let test_value: i64 = inner[0].parse().unwrap();
            let eq_comp: Vec<i64> = inner[1]
                .trim()
                .split(" ")
                .map(|n| n.parse().unwrap())
                .collect();
            (test_value, eq_comp)
        })
        .collect()
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: program input.txt");
        process::exit(1);
    }

    let calibrations = parse_input(&args[1]);
    part1::solve(&calibrations);
    part2::solve(&calibrations);
}
