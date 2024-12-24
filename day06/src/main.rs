use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub mod map;
pub mod part1;
pub mod part2;

fn get_grid(file_name: &str) -> Vec<Vec<char>> {
    let file = File::open(file_name).unwrap();
    let file = BufReader::new(file);

    file.lines()
        .map(|line| line.unwrap().chars().collect::<Vec<_>>())
        .fold(Vec::new(), |mut acc, inner| {
            acc.push(inner);
            acc
        })
}
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("Usage: program input.txt");
    }

    let grid = get_grid(&args[1]);

    let mut mapa = map::Map {
        grid,
        gpos: (0, 0),
        direction: map::Direction::NOTDEFINED,
    };

    let mut mapa2 = map::Map {
        grid: mapa.grid.clone(),
        gpos: (0, 0),
        direction: map::Direction::NOTDEFINED,
    };

    mapa.find_guard('^').unwrap();
    mapa2.find_guard('^').unwrap();
    part1::solve(&mut mapa);
    part2::solve(&mut mapa2);
}
