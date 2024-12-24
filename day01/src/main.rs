use std::env;
use std::fs;

mod part1;
mod part2;

fn parse_input(file_path: &String) -> (Vec<u32>, Vec<u32>) {
    let contents = fs::read_to_string(&file_path).expect("Error");
    let contents: Vec<_> = contents.split("\n").collect();

    let mut col1: Vec<u32> = Vec::new();
    let mut col2: Vec<u32> = Vec::new();

    for ele in contents {
        match ele {
            "" => break,
            _ => {
                let ele: Vec<_> = ele.split("   ").collect();
                col1.push(ele[0].parse().expect("Error"));
                col2.push(ele[1].parse().expect("Error"));
            }
        }
    }

    (col1, col2)
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("Usage: program input.txt");
    }

    let (mut col1, mut col2) = parse_input(&args[1]);

    col1.sort();
    col2.sort();

    part1::solve(&col1, &col2);
    part2::solve(&col1, &col2);
}
