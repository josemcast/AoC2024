use std::env;
use std::fs;

mod part1;
mod part2;

fn parse_input(filepath: &String) -> Vec<String> {
    let content = fs::read_to_string(&filepath).expect("Could not read");
    let content: Vec<String> = content.split("\n")
                                .filter(|s| *s != "")
                                .map(|s| s.to_string())
                                .collect();
    content
}

fn main() {
    let arg: Vec<String> = env::args().collect();

    if arg.len() < 2 {
        panic!("Usage -> main input.txt");
    }

    let reports = parse_input(&arg[1]);

    part1::solve(&reports);
    part2::solve(&reports);

    println!("");
}
