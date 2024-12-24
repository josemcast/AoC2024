use std::collections::HashMap;
use std::env;
use std::fs;

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

    let mut dist = 0;

    let mut list2 = HashMap::new();

    //Part 01
    for i in 0..col1.len() {
        dist += col1[i].abs_diff(col2[i]);
        list2.entry(col2[i]).and_modify(|x| *x += 1).or_insert(1);
    }

    println!("dist: {}", dist);

    //Part 02
    dist = 0;

    for i in 0..col1.len() {
        dist += match list2.get(&col1[i]) {
            Some(num) => col1[i] * num,
            None => 0,
        }
    }

    println!("Dist: {}", dist);
}
