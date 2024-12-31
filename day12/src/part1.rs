use std::collections::{HashMap, HashSet};

fn find_groups(
    garden: &Vec<Vec<char>>,
    groups: &mut Vec<Vec<i32>>,
    probe: char,
    id: i32,
    i: usize,
    j: usize,
    visited: &mut HashSet<(usize, usize)>,
) -> (i32, i32) {
    let nrows = garden.len() as i32;
    let ncols = garden[0].len() as i32;

    let mut stack = Vec::new();
    stack.push((i, j));

    let mut count = 0;
    let mut perimeter = 0;
    while stack.len() != 0 {
        let directions = vec![(0, 1), (1, 0), (0, -1), (-1, 0)];
        let (x, y) = stack.pop().unwrap();

        groups[x][y] = id;
        if visited.contains(&(x, y)) {
            continue;
        }

        visited.insert((x, y));
        perimeter += 4;
        count += 1;
        for direction in directions {
            let (dx, dy) = direction;
            let xx = x as i32 + dx;
            let yy = y as i32 + dy;
            if (xx >= 0 && xx < nrows) && (yy >= 0 && yy < ncols) {
                if !visited.contains(&(xx as usize, yy as usize))
                    && garden[xx as usize][yy as usize] == probe
                {
                    stack.push((xx as usize, yy as usize));
                }

                if garden[xx as usize][yy as usize] == probe
                    && visited.contains(&(xx as usize, yy as usize))
                {
                    perimeter -= 2;
                }
            }
        }
    }

    (count, perimeter)
}
pub fn solve(garden_plots: &Vec<Vec<char>>) {
    let mut visited = HashSet::new();

    let nrows = garden_plots.len();
    let ncols = garden_plots[0].len();

    let mut group_id = 0;
    let mut ids = HashMap::new();
    let mut area_perimeter = HashMap::new();

    let mut groups = vec![vec![-1; ncols]; nrows];
    for i in 0..garden_plots.len() {
        for j in 0..garden_plots[0].len() {
            if !visited.contains(&(i, j)) {
                let value = ids.entry(&garden_plots[i][j]).or_insert(group_id);
                let entry = area_perimeter
                    .entry(&garden_plots[i][j])
                    .or_insert(Vec::new());
                entry.push(find_groups(
                    &garden_plots,
                    &mut groups,
                    garden_plots[i][j],
                    *value,
                    i,
                    j,
                    &mut visited,
                ));
                group_id += 1;
            }
        }
    }

    let mut sum: i32 = 0;
    for (_, entries) in area_perimeter {
        sum += entries.into_iter().map(|(a, p)| a * p).sum::<i32>();
    }

    println!("Total price pt1: {sum}");
}
