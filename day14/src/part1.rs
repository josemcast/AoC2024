use day14::Robot;

fn walk(robots: &Vec<Robot>, tile_x: i32, tile_y: i32) -> i32 {
    let intersection = (tile_x / 2, tile_y / 2);
    let mut quadrants = vec![0; 4];
    for robot in robots {
        let px = (robot.px + 100 * robot.vx).rem_euclid(tile_x);
        let py = (robot.py + 100 * robot.vy).rem_euclid(tile_y);

        if px < intersection.0 && py < intersection.1 {
            quadrants[0] += 1;
        } else if px > intersection.0 && py < intersection.1 {
            quadrants[1] += 1;
        } else if px < intersection.0 && py > intersection.1 {
            quadrants[2] += 1;
        } else if px > intersection.0 && py > intersection.1 {
            quadrants[3] += 1;
        }
    }

    quadrants.iter().product()
}

pub fn solve(robots: &Vec<Robot>) {
    let safety_factor = walk(robots, 101, 103);
    println!("Solution pt1: {safety_factor}");
}
