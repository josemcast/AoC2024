use crate::map::{Direction, Map};
use std::collections::{HashMap, HashSet};

fn state_machine(direction: &Direction) -> Direction {
    match direction {
        Direction::UP => Direction::RIGHT,
        Direction::RIGHT => Direction::DOWN,
        Direction::DOWN => Direction::LEFT,
        Direction::LEFT => Direction::UP,
        Direction::NOTDEFINED => panic!("Oh boy...."),
    }
}

fn is_inside_limits(g_x: i32, g_y: i32, maxx: i32, maxy: i32) -> bool {
    g_x > -1 && g_x < maxx && g_y > -1 && g_y < maxy
}

fn is_loop(mapa: &Map, max_x: i32, max_y: i32) -> bool {
    let initial_x = mapa.gpos.0;
    let initial_y = mapa.gpos.1;
    let mut current_x = initial_x;
    let mut current_y = initial_y;
    let mut direction = match mapa.direction {
        Direction::UP => Direction::UP,
        Direction::DOWN => Direction::DOWN,
        Direction::RIGHT => Direction::RIGHT,
        Direction::LEFT => Direction::LEFT,
        Direction::NOTDEFINED => Direction::NOTDEFINED,
    };
    let (mut dx, mut dy) = mapa.direction.step();
    let mut same_pos = 0;

    let mut visited_walls = HashMap::new();

    while is_inside_limits(current_x + dx, current_y + dy, max_x as i32, max_y as i32) {
        let probe_x = current_x + dx;
        let probe_y = current_y + dy;

        if probe_x == initial_x && probe_y == initial_y {
            same_pos += 1;
            if same_pos > 4 {
                return true;
            }
        }

        match mapa.grid[probe_x as usize][probe_y as usize] {
            '.' => {
                current_x = probe_x;
                current_y = probe_y;
            }
            '#' => {
                *visited_walls.entry((probe_x, probe_y)).or_insert(0) += 1;
                if *visited_walls.get(&(probe_x, probe_y)).unwrap() > 4 {
                    return true;
                }
                direction = state_machine(&direction);
                (dx, dy) = direction.step();
            }
            _ => continue,
        }
    }

    false
}

pub fn solve(mapa: &mut Map) {
    let mut positions = 0;
    let (mut dx, mut dy) = mapa.direction.step();

    let max_x = mapa.grid.len();
    let max_y = mapa.grid[0].len();

    let mut visited = HashSet::new();

    visited.insert((mapa.gpos.0, mapa.gpos.1));
    while is_inside_limits(
        mapa.gpos.0 + dx,
        mapa.gpos.1 + dy,
        max_x as i32,
        max_y as i32,
    ) {
        let probe_x = mapa.gpos.0 + dx;
        let probe_y = mapa.gpos.1 + dy;

        match mapa.grid[probe_x as usize][probe_y as usize] {
            '.' => {
                mapa.grid[probe_x as usize][probe_y as usize] = '#';
                if !visited.contains(&(probe_x, probe_y))
                    && is_loop(&mapa, max_x as i32, max_y as i32)
                {
                    positions += 1;
                }
                visited.insert((probe_x, probe_y));
                mapa.grid[probe_x as usize][probe_y as usize] = '.';
                mapa.gpos.0 = probe_x;
                mapa.gpos.1 = probe_y;
            }
            '#' => {
                mapa.direction = state_machine(&mapa.direction);
                (dx, dy) = mapa.direction.step();
            }
            _ => continue,
        }
    }

    println!("Distinct pt2: {}", positions);
}
