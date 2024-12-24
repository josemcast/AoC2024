use regex::Regex;
use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn solve(file_name: &String) -> io::Result<()> {
    let file = File::open(file_name)?;
    let file = BufReader::new(file);

    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    let lines = file.lines().map(|line| line.unwrap()).collect::<Vec<_>>();

    let mut sum = 0;
    for line in lines {
        sum += re
            .captures_iter(&line)
            .map(|caps| {
                let (_, [first, second]) = caps.extract();
                first.parse::<i32>().expect("Could not parse")
                    * second.parse::<i32>().expect("Could not parse")
            })
            .sum::<i32>()
    }

    println!("Sum {}", sum);
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("Usage: program input.txt");
    }

    solve(&args[1]).unwrap();
    println!("To Solve part2 run the program with the filtered input.");
}
