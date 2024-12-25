use std::collections::HashSet;

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

fn search_peak(
    hiker: (i32, i32, char),
    grid: &Vec<Vec<i32>>,
    visited_nines: &mut HashSet<(i32, i32)>,
) {
    if grid[hiker.0 as usize][hiker.1 as usize] == 9 {
        visited_nines.insert((hiker.0, hiker.1));
    }

    let mut next = state_machine(&hiker);

    let nrows = grid.len();
    let ncols = grid[0].len();

    if is_bound(&next, nrows, ncols)
        && ((grid[next.0 as usize][next.1 as usize] - 1)
            == grid[hiker.0 as usize][hiker.1 as usize])
    {
        search_peak(next, grid, visited_nines);
    }

    next = state_machine(&(hiker.0, hiker.1, next.2));

    if is_bound(&next, nrows, ncols)
        && ((grid[next.0 as usize][next.1 as usize] - 1)
            == grid[hiker.0 as usize][hiker.1 as usize])
    {
        search_peak(next, grid, visited_nines);
    }

    next = state_machine(&(hiker.0, hiker.1, next.2));

    if is_bound(&next, nrows, ncols)
        && ((grid[next.0 as usize][next.1 as usize] - 1)
            == grid[hiker.0 as usize][hiker.1 as usize])
    {
        search_peak(next, grid, visited_nines);
    }

    next = state_machine(&(hiker.0, hiker.1, next.2));

    if is_bound(&next, nrows, ncols)
        && ((grid[next.0 as usize][next.1 as usize] - 1)
            == grid[hiker.0 as usize][hiker.1 as usize])
    {
        search_peak(next, grid, visited_nines);
    }
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
    let mut visited_nines = HashSet::new();

    let trailheads = find_trailheads(topo_map);
    let mut sum_score = 0;
    for trailhead in trailheads {
        search_peak(
            (trailhead.0, trailhead.1, 'U'),
            topo_map,
            &mut visited_nines,
        );
        sum_score += visited_nines.len();
        visited_nines.clear();
    }
    println!("Solution pt1 {sum_score}");
}
