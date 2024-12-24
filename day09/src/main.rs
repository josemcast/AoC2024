use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process;

mod part1;
mod part2;

fn parse_input(file_path: &str) -> (Vec<String>, HashMap<String, u32>) {
    let file = File::open(file_path).unwrap();
    let file = BufReader::new(file);

    let mut id = 0;
    let mut id_block = HashMap::new();
    let disk_map: String = file.lines().map(|line| line.unwrap()).collect();
    let disk_mapx = disk_map
        .chars()
        .enumerate()
        .fold(Vec::new(), |mut acc, (index, value)| {
            let count = value.to_digit(10).unwrap();
            if index % 2 == 0 {
                *id_block.entry(id.to_string()).or_insert(0) += count;
                for _ in 0..count {
                    acc.push(id.to_string());
                }

                id += 1;
            } else {
                for _ in 0..count {
                    acc.push(".".to_string());
                }
            }

            acc
        });

    (disk_mapx, id_block)
}
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: program input.txt");
        process::exit(1);
    }

    let (mut disk_map, id_block) = parse_input(&args[1]);
    let mut disk_map2 = disk_map.clone();
    part1::solve(&mut disk_map);
    part2::solve(&mut disk_map2, &id_block);
}
