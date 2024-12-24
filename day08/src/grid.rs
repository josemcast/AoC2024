use std::collections::HashMap;

#[derive(Debug)]
pub struct Grid {
    pub antennas: HashMap<char, Vec<(i32, i32)>>,
    pub nrows: usize,
    pub ncols: usize,
}

impl Grid {
    pub fn new(antennas: HashMap<char, Vec<(i32, i32)>>, nrows: usize, ncols: usize) -> Self {
        Grid {
            antennas,
            nrows,
            ncols,
        }
    }
}
