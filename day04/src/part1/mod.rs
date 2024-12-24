fn find_xmas(grid: &[Vec<char>], i: usize, j: usize) -> usize {
    let mut count: usize = 0;

    let row_size = grid[0].len();
    let col_size = grid.len();

    let xmas = "XMAS";

    if j < (row_size - 3) {
        if (0..4).all(|k| grid[i][j + k] == xmas.chars().nth(k).unwrap()) {
            count += 1;
        }

        if i < (col_size - 3) {
            if (0..4).all(|k| grid[i + k][j + k] == xmas.chars().nth(k).unwrap()) {
                count += 1;
            }
        }

        if i > 2 {
            if (0..4).all(|k| grid[i - k][j + k] == xmas.chars().nth(k).unwrap()) {
                count += 1;
            }
        }
    }

    if j > 2 {
        if (0..4).all(|k| grid[i][j - k] == xmas.chars().nth(k).unwrap()) {
            count += 1;
        }

        if i < (col_size - 3) {
            if (0..4).all(|k| grid[i + k][j - k] == xmas.chars().nth(k).unwrap()) {
                count += 1;
            }
        }

        if i > 2 {
            if (0..4).all(|k| grid[i - k][j - k] == xmas.chars().nth(k).unwrap()) {
                count += 1;
            }
        }
    }

    if i < (col_size - 3) {
        if (0..4).all(|k| grid[i + k][j] == xmas.chars().nth(k).unwrap()) {
            count += 1;
        }
    }

    if i > 2 {
        if (0..4).all(|k| grid[i - k][j] == xmas.chars().nth(k).unwrap()) {
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
                .filter(|(_, value)| **value == 'X')
                .map(|(j, _)| find_xmas(&grid, i, j))
                .sum::<usize>()
        })
        .sum::<usize>();

    println!("Count pt.1: {:?}", count);
}
