use std::collections::{HashMap, HashSet};

fn find_groups(
    garden: &Vec<Vec<char>>,
    probe: char,
    i: usize,
    j: usize,
    visited: &mut HashSet<(usize, usize)>,
) -> (i32, i32) {
    let nrows = garden.len() as i32;
    let ncols = garden[0].len() as i32;

    let mut stack = Vec::new();
    stack.push((i, j)); // DFS - Depth-first search

    let mut area = 0;
    let mut perimeter = 0;
    while stack.len() != 0 {
        let (x, y) = stack.pop().unwrap();

        if visited.contains(&(x, y)) {
            continue;
        }

        visited.insert((x, y));
        area += 1;
        let mut neighbors = Vec::new();
        let mut neighbors_diff = Vec::new();

        let directions = vec![(0, 1), (1, 0), (0, -1), (-1, 0)];
        for direction in directions {
            let (dx, dy) = direction;
            let xx = x as i32 + dx;
            let yy = y as i32 + dy;

            if (xx >= 0 && xx < nrows) && (yy >= 0 && yy < ncols) {
                //For counting corners
                if garden[xx as usize][yy as usize] == probe {
                    neighbors.push((xx, yy));
                } else {
                    neighbors_diff.push((xx, yy));
                }

                if !visited.contains(&(xx as usize, yy as usize))
                    && garden[xx as usize][yy as usize] == probe
                {
                    stack.push((xx as usize, yy as usize));
                }
            } else {
                neighbors_diff.push((xx, yy)); //for counting corners
            }
        }

        // Count corners for neighbors that have the same value as probe
        for (index, neighbor) in neighbors.iter().enumerate() {
            for ele in (index + 1)..neighbors.len() {
                let neighbor2 = neighbors[ele];
                let diff = neighbor.0.abs_diff(neighbor2.0) + neighbor.1.abs_diff(neighbor2.1);
                if diff == 2 {
                    let diag = (neighbor.0 + neighbor2.0, neighbor.1 + neighbor2.1);
                    let diag = ((diag.0 - x as i32), (diag.1 - y as i32));
                    if diag.0 >= 0 && diag.0 < nrows && diag.1 >= 0 && diag.1 < ncols {
                        if garden[diag.0 as usize][diag.1 as usize] != probe {
                            perimeter += 1;
                        }
                    }
                }
            }
        }

        // Count corners for neighbors with value different from probe, even if they lay outside
        // the grid bounds
        for (index, neighbor) in neighbors_diff.iter().enumerate() {
            for ele in (index + 1)..neighbors_diff.len() {
                let neighbor2 = neighbors_diff[ele];
                let diff = (
                    neighbor.0.abs_diff(neighbor2.0),
                    neighbor.1.abs_diff(neighbor2.1),
                );
                if (diff.0 + diff.1) == 2 && diff.0 != 0 && diff.1 != 0 {
                    perimeter += 1;
                }
            }
        }
    }

    (area, perimeter)
}
pub fn solve(garden_plots: &Vec<Vec<char>>) {
    let mut visited = HashSet::new();

    let mut groups = HashMap::new();

    for i in 0..garden_plots.len() {
        for j in 0..garden_plots[0].len() {
            if !visited.contains(&(i, j)) {
                let entry = groups.entry(&garden_plots[i][j]).or_insert(Vec::new());
                entry.push(find_groups(
                    &garden_plots,
                    garden_plots[i][j],
                    i,
                    j,
                    &mut visited,
                ));
            }
        }
    }

    let mut sum: i32 = 0;
    for (_, entries) in groups {
        sum += entries.into_iter().map(|(a, p)| a * p).sum::<i32>();
    }

    println!("Total price pt2: {sum}");
}
