use core::panic;
use regex::Regex;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

use day14::Robot;

mod part1;
mod part2;

fn parse_input(file_name: &str) -> Vec<Robot> {
    let file = File::open(file_name).unwrap();
    let file = BufReader::new(file);

    let re = Regex::new(r"p=(.+),(.+) v=(.+),(.+)").unwrap();

    file.lines().fold(Vec::new(), |mut acc, line| {
        let l = line.unwrap();
        let (_, [px, py, vx, vy]) = re.captures(&l).unwrap().extract();
        let px = px.parse().unwrap();
        let py = py.parse().unwrap();
        let vx = vx.parse().unwrap();
        let vy = vy.parse().unwrap();
        acc.push(Robot { px, py, vx, vy });
        acc
    })
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("usage program input.txt");
    }

    let mut robots = parse_input(&args[1]);

    part1::solve(&robots);
    part2::solve(&mut robots);
}
