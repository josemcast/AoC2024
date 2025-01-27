fn move_to_direction(current: &char) -> (i32, i32) {
    match current {
        '^' => (-1, 0),
        '>' => (0, 1),
        'v' => (1, 0),
        '<' => (0, -1),
        _ => panic!("Invalid move"),
    }
}

fn find_robot(map: &Vec<Vec<char>>) -> (i32, i32) {
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            if map[i][j] == '@' {
                return (i as i32, j as i32);
            }
        }
    }
    panic!("There's no robot < @ > in the grid");
}

fn calculate_gps_coord(map: &Vec<Vec<char>>) -> i64 {
    let mut sum = 0;
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            if map[i][j] == 'O' {
                sum += 100 * i as i64 + j as i64;
            }
        }
    }

    sum
}

fn perform_moves(map: &mut Vec<Vec<char>>, moves: &Vec<char>, robot: (i32, i32)) {
    let mut robot = robot;
    for mov in moves {
        let dir = move_to_direction(mov);
        let mut next_pos = (robot.0 + dir.0, robot.1 + dir.1);
        if map[next_pos.0 as usize][next_pos.1 as usize] == '.' {
            map[next_pos.0 as usize][next_pos.1 as usize] = '@';
            map[robot.0 as usize][robot.1 as usize] = '.';
            robot = next_pos;
            continue;
        } else {
            let mut index = 0;
            while map[next_pos.0 as usize][next_pos.1 as usize] != '.'
                && map[next_pos.0 as usize][next_pos.1 as usize] != '#'
            {
                next_pos = (next_pos.0 + dir.0, next_pos.1 + dir.1);
                index += 1;
            }

            if map[next_pos.0 as usize][next_pos.1 as usize] == '#' {
                continue;
            } else {
                let rev_dir = (-dir.0, -dir.1);
                while index >= 0 {
                    let one_previous = (next_pos.0 + rev_dir.0, next_pos.1 + rev_dir.1);
                    let aux = map[one_previous.0 as usize][one_previous.1 as usize];
                    map[one_previous.0 as usize][one_previous.1 as usize] =
                        map[next_pos.0 as usize][next_pos.1 as usize];
                    map[next_pos.0 as usize][next_pos.1 as usize] = aux;
                    next_pos = one_previous;
                    index -= 1;
                }

                robot = (robot.0 + dir.0, robot.1 + dir.1);
            }
        }
    }
}
pub fn solve(map: &mut Vec<Vec<char>>, moves: &Vec<char>) {
    let pos = find_robot(map);

    perform_moves(map, moves, pos);
    let sum = calculate_gps_coord(map);
    println!("Sum pt1. {sum}");
}
