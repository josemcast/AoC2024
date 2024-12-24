use crate::grid::Grid;
use std::collections::HashSet;

fn is_in_bounds(probe: (i32, i32), nrows: i32, ncols: i32) -> bool {
    (probe.0 >= 0 && probe.0 < nrows) && (probe.1 >= 0 && probe.1 < ncols)
}

fn find_antinodes(
    antinodes: &mut HashSet<(i32, i32)>,
    r: (i32, i32),
    probe: &[(i32, i32)],
    nrows: i32,
    ncols: i32,
) {
    for (xd, yd) in probe {
        let dp = (xd - r.0, yd - r.1);
        let p1 = (r.0 - dp.0, r.1 - dp.1);
        let p2 = (xd + dp.0, yd + dp.1);

        if is_in_bounds(p1, nrows, ncols) {
            antinodes.insert((p1.0, p1.1));
        }

        if is_in_bounds(p2, nrows, ncols) {
            antinodes.insert((p2.0, p2.1));
        }
    }
}

pub fn solve(grid: &Grid) {
    let mut antinodes = HashSet::new();

    for (_, locations) in &grid.antennas {
        for (i, antenna) in locations.iter().enumerate() {
            if locations.len() > 1 {
                find_antinodes(
                    &mut antinodes,
                    *antenna,
                    &locations[(i + 1)..],
                    grid.nrows as i32,
                    grid.ncols as i32,
                );
            }
        }
    }

    println!("Antinodes count pt1: {}", antinodes.len());
}
