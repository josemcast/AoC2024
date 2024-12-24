use crate::map::{Direction, Map};
//use std::{thread, time};

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

pub fn solve(mapa: &mut Map) {
    let mut positions = 1;
    let (mut dx, mut dy) = mapa.direction.step();

    let max_x = mapa.grid.len();
    let max_y = mapa.grid[0].len();

    while is_inside_limits(
        mapa.gpos.0 + dx,
        mapa.gpos.1 + dy,
        max_x as i32,
        max_y as i32,
    ) {
        /*
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        mapa.display();
        thread::sleep(time::Duration::from_millis(100));
        */
        mapa.grid[mapa.gpos.0 as usize][mapa.gpos.1 as usize] = 'X';
        let probe_x = mapa.gpos.0 + dx;
        let probe_y = mapa.gpos.1 + dy;

        match mapa.grid[probe_x as usize][probe_y as usize] {
            '.' => {
                positions += 1;
                mapa.gpos.0 = probe_x;
                mapa.gpos.1 = probe_y;
                mapa.grid[probe_x as usize][probe_y as usize] = 'X'
            }
            'X' => {
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

    println!("Distinct Positions pt1: {}", positions);
}
