use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub mod part1;
pub mod part2;

fn get_rules(file_name: &str) -> HashMap<String, Vec<String>> {
    let file = File::open(file_name).unwrap();
    let file = BufReader::new(file);

    file.lines()
        .filter_map(|line| {
            let l = line.unwrap();
            let mut parts = l.split("|").map(|s| String::from(s));
            match (parts.next(), parts.next()) {
                (Some(key), Some(value)) => Some((key.to_string(), value.to_string())),
                _ => None,
            }
        })
        .fold(HashMap::new(), |mut acc, (key, value)| {
            acc.entry(key).or_insert(Vec::new()).push(value);
            acc
        })
}

fn get_updates(file_name: &str) -> Vec<Vec<String>> {
    let file = File::open(file_name).unwrap();
    let file = BufReader::new(file);

    file.lines()
        .map(|line| {
            let l = line.unwrap();
            l.split(",").map(|s| String::from(s)).collect::<Vec<_>>()
        })
        .fold(Vec::new(), |mut acc, inner| {
            acc.push(inner);
            acc
        })
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        panic!("Usage: program rules.txt input.txt");
    }

    let mut rules = get_rules(&args[1]);
    let updates = get_updates(&args[2]);

    let mut not_valids = part1::solve(&mut rules, &updates);
    part2::solve(&mut rules, &mut not_valids);
}
