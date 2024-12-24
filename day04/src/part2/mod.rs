fn find_x_mas(grid: &[Vec<char>], i: usize, j: usize) -> usize {
    let mut count: usize = 0;

    let row_size = grid[0].len();
    let col_size = grid.len();

    if i == 0 || i == (col_size - 1) || j == 0 || j == (row_size - 1) {
        return count;
    }

    let mut probe: String = (-1..2)
        .into_iter()
        .map(|n: i32| grid[(i as i32 + n) as usize][(j as i32 + n) as usize])
        .collect();

    if probe == "MAS" || probe == "SAM" {
        probe.clear();

        probe = (-1..2)
            .into_iter()
            .map(|n: i32| grid[(i as i32 + n) as usize][(j as i32 - n) as usize])
            .collect();

        if probe == "MAS" || probe == "SAM" {
            count += 1;
        }
    }

    count
}

pub fn solve(grid: &[Vec<char>]) {
    let count = grid
        .iter()
        .enumerate()
        .map(|(i, v)| {
            v.iter()
                .enumerate()
                .filter(|(_, value)| **value == 'A')
                .map(|(j, _)| find_x_mas(&grid, i, j))
                .sum::<usize>()
        })
        .sum::<usize>();

    println!("Count pt.2: {:?}", count);
}
