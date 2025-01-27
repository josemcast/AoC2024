fn move_to_direction(current: &char) -> (i32, i32) {
    match current {
        '^' => (-1, 0),
        '>' => (0, 1),
        'v' => (1, 0),
        '<' => (0, -1),
        _ => panic!("Invalid move"),
    }
}

fn expand_map(map: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let nrows = map.len();
    let ncols = map[0].len();

    let mut new_map = vec![vec!['A'; 2 * ncols]; nrows];

    for i in 0..nrows {
        for j in 0..ncols {
            match map[i][j] {
                '.' => {
                    new_map[i][2 * j] = '.';
                    new_map[i][2 * j + 1] = '.'
                }
                '#' => {
                    new_map[i][2 * j] = '#';
                    new_map[i][2 * j + 1] = '#'
                }
                '@' => {
                    new_map[i][2 * j] = '@';
                    new_map[i][2 * j + 1] = '.'
                }
                'O' => {
                    new_map[i][2 * j] = '[';
                    new_map[i][2 * j + 1] = ']'
                }
                _ => panic!("Invalid char"),
            }
        }
    }

    new_map
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
            if map[i][j] == '[' {
                let mut k = j + 2;
                while k < map[0].len() {
                    k += 1;
                }

                let j_min = j.min(k);
                sum += 100 * i as i64 + j_min as i64;
            }
        }
    }

    sum
}

fn perform_moves(map: &mut Vec<Vec<char>>, moves: &Vec<char>, robot: (i32, i32)) {
    let mut robot = robot;
    for mov in moves {
        let dir = move_to_direction(mov);
        let next_pos = (robot.0 + dir.0, robot.1 + dir.1);
        if map[next_pos.0 as usize][next_pos.1 as usize] == '.' {
            map[next_pos.0 as usize][next_pos.1 as usize] = '@';
            map[robot.0 as usize][robot.1 as usize] = '.';
            robot = next_pos;
        } else if map[next_pos.0 as usize][next_pos.1 as usize] == '#' {
        } else {
            let mut index = 0;
            let mut stack = Vec::new();
            stack.push((next_pos.0, next_pos.1));
            let mut visited = Vec::new();
            let mut next = (0, 0);
            while stack.len() != 0 {
                next = stack.pop().unwrap();
                if map[next.0 as usize][next.1 as usize] == '#' {
                    break;
                }

                if map[next.0 as usize][next.1 as usize] != '.' {
                    if *mov == '>' || *mov == '<' {
                        stack.push((next.0 + 2 * dir.0, next.1 + 2 * dir.1));
                        index += 2;
                    } else {
                        if !visited.contains(&(next.0, next.1)) {
                            visited.push((next.0, next.1));
                        }
                        if map[next.0 as usize][next.1 as usize] == '[' {
                            if !visited.contains(&(next.0, next.1 + 1)) {
                                stack.push((next.0, next.1 + 1));
                            }
                            stack.push((next.0 + dir.0, next.1 + dir.1));
                        } else {
                            if !visited.contains(&(next.0, next.1 - 1)) {
                                stack.push((next.0, next.1 - 1));
                            }
                            stack.push((next.0 + dir.0, next.1 + dir.1));
                        }
                    }
                }
            }

            if map[next.0 as usize][next.1 as usize] == '#' {
            } else {
                if *mov == '>' || *mov == '<' {
                    let rev_dir = (-dir.0, -dir.1);
                    while index >= 0 {
                        let one_previous = (next.0 + rev_dir.0, next.1 + rev_dir.1);
                        let aux = map[one_previous.0 as usize][one_previous.1 as usize];
                        map[one_previous.0 as usize][one_previous.1 as usize] =
                            map[next.0 as usize][next.1 as usize];
                        map[next.0 as usize][next.1 as usize] = aux;
                        next = one_previous;
                        index -= 1;
                    }

                    robot = (robot.0 + dir.0, robot.1 + dir.1);
                } else {
                    while visited.len() != 0 {
                        let ele = visited.pop().unwrap();
                        let new_pos = (ele.0 + dir.0, ele.1 + dir.1);
                        //println!("ele: {:?}, new_pos: {:?}", ele, new_pos);
                        if map[new_pos.0 as usize][new_pos.1 as usize] == '.' {
                            map[new_pos.0 as usize][new_pos.1 as usize] =
                                map[ele.0 as usize][ele.1 as usize];
                            map[ele.0 as usize][ele.1 as usize] = '.';
                        } else {
                            visited.insert(0, (ele.0, ele.1));
                        }
                    }

                    map[robot.0 as usize][robot.1 as usize] = '.';
                    robot = (robot.0 + dir.0, robot.1 + dir.1);
                    map[robot.0 as usize][robot.1 as usize] = '@';
                }
            }
        }
        //println!("move: {mov}");
        //for row in map.clone() {
        //    println!("{}", row.iter().collect::<String>());
        //}
        //println!("\n");
    }
}
pub fn solve(map: &mut Vec<Vec<char>>, moves: &Vec<char>) {
    //expand map call
    let mut new_map = expand_map(map);

    let pos = find_robot(&mut new_map);

    perform_moves(&mut new_map, moves, pos);

    let sum = calculate_gps_coord(&new_map);
    println!("Sum pt2. {sum}");
}
