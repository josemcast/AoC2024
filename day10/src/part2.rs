fn state_machine(dir: &(i32, i32, char)) -> (i32, i32, char) {
    match dir.2 {
        'D' => (dir.0, dir.1 - 1, 'L'),
        'L' => (dir.0 - 1, dir.1, 'U'),
        'R' => (dir.0 + 1, dir.1, 'D'),
        'U' => (dir.0, dir.1 + 1, 'R'),
        _ => panic!("oh no..."),
    }
}

fn is_bound(probe: &(i32, i32, char), nrows: usize, ncols: usize) -> bool {
    (probe.0 >= 0 && probe.0 < nrows as i32) && (probe.1 >= 0 && probe.1 < ncols as i32)
}

fn search_peak(hiker: (i32, i32, char), grid: &Vec<Vec<i32>>) -> i32 {
    if grid[hiker.0 as usize][hiker.1 as usize] == 9 {
        return 1;
    }

    let mut rating = 0;
    let mut next = state_machine(&hiker);

    let nrows = grid.len();
    let ncols = grid[0].len();

    if is_bound(&next, nrows, ncols)
        && ((grid[next.0 as usize][next.1 as usize] - 1)
            == grid[hiker.0 as usize][hiker.1 as usize])
    {
        rating += search_peak(next, grid);
    }

    next = state_machine(&(hiker.0, hiker.1, next.2));

    if is_bound(&next, nrows, ncols)
        && ((grid[next.0 as usize][next.1 as usize] - 1)
            == grid[hiker.0 as usize][hiker.1 as usize])
    {
        rating += search_peak(next, grid);
    }

    next = state_machine(&(hiker.0, hiker.1, next.2));

    if is_bound(&next, nrows, ncols)
        && ((grid[next.0 as usize][next.1 as usize] - 1)
            == grid[hiker.0 as usize][hiker.1 as usize])
    {
        rating += search_peak(next, grid);
    }

    next = state_machine(&(hiker.0, hiker.1, next.2));

    if is_bound(&next, nrows, ncols)
        && ((grid[next.0 as usize][next.1 as usize] - 1)
            == grid[hiker.0 as usize][hiker.1 as usize])
    {
        rating += search_peak(next, grid);
    }

    rating
}

fn find_trailheads(topo_map: &Vec<Vec<i32>>) -> Vec<(i32, i32)> {
    let mut trailheads = Vec::new();
    for i in 0..topo_map.len() {
        for j in 0..topo_map[0].len() {
            if topo_map[i][j] == 0 {
                trailheads.push((i as i32, j as i32));
            }
        }
    }

    trailheads
}
pub fn solve(topo_map: &Vec<Vec<i32>>) {
    let trailheads = find_trailheads(topo_map);
    let mut sum_ratings = 0;
    for trailhead in trailheads {
        sum_ratings += search_peak((trailhead.0, trailhead.1, 'U'), topo_map);
    }
    println!("Solution pt2 {sum_ratings}");
}
