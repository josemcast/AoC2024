use regex::Regex;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::{env, u32};

fn solve_line(input: &[(u32, u32)]) -> u32 {
    let mut sum: u32 = 0;
    for ele in input {
        sum += ele.0 * ele.1;
    }

    sum
}

fn parse_input(file_name: &String) -> io::Result<()> {
    let file = File::open(file_name)?;
    let file = BufReader::new(file);

    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    let lines = file.lines().map(|line| line.unwrap()).collect::<Vec<_>>();

    let mut sum = 0;
    for line in lines {
        let matches = re
            .captures_iter(&line)
            .map(|caps| {
                let (_, [first, second]) = caps.extract();
                (
                    first.parse().expect("Could not parse"),
                    second.parse().expect("Could not parse"),
                )
            })
            .collect::<Vec<_>>();

        sum += solve_line(&matches);
    }

    println!("Sum {}", sum);
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("Usage: program input.txt");
    }

    parse_input(&args[1]).unwrap();
}
