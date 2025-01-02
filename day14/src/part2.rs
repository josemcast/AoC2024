use day14::Robot;

fn find_seconds_for_tree(robots: &Vec<Robot>, tile_x: i32, tile_y: i32) -> i32 {
    let mut grid = vec![vec!['.'; tile_x as usize]; tile_y as usize];
    let mut max_count = 0;

    'outer: for i in 103..10000 {
        for robot in robots {
            let px = (robot.px + i * robot.vx).rem_euclid(tile_x);
            let py = (robot.py + i * robot.vy).rem_euclid(tile_y);
            grid[py as usize][px as usize] = '*';
        }

        let mut current_count = 0;

        //find longest sequence of * indicating the frame of the tree
        for k in 0..tile_y {
            for j in 0..tile_x {
                if grid[k as usize][j as usize] == '*' {
                    current_count += 1;
                    max_count = max_count.max(current_count);

                    if max_count > 10 {
                        max_count = i;
                        break 'outer;
                    }
                } else {
                    current_count = 0;
                }
            }
        }

        for i in 0..tile_y {
            for j in 0..tile_x {
                grid[i as usize][j as usize] = ' ';
            }
        }
    }

    max_count
}

fn print_by_second(robots: &Vec<Robot>, seconds: i32, tile_x: i32, tile_y: i32) {
    let mut grid = vec![vec!['.'; tile_x as usize]; tile_y as usize];
    for robot in robots {
        let px = (robot.px + seconds * robot.vx).rem_euclid(tile_x);
        let py = (robot.py + seconds * robot.vy).rem_euclid(tile_y);
        grid[py as usize][px as usize] = '*';
    }

    for row in grid {
        println!("{}", row.iter().collect::<String>());
    }
}

pub fn solve(robots: &mut Vec<Robot>) {
    let seconds = find_seconds_for_tree(robots, 101, 103);
    println!("Solution pt2. {seconds}");
    println!("\n\n\n");
    print_by_second(robots, seconds, 101, 103);
}
