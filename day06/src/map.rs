use std::usize;

#[derive(Debug, PartialEq)]
pub enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
    NOTDEFINED,
}

impl Direction {
    pub fn step(&self) -> (i32, i32) {
        match self {
            Direction::UP => (-1, 0),
            Direction::DOWN => (1, 0),
            Direction::LEFT => (0, -1),
            Direction::RIGHT => (0, 1),
            Direction::NOTDEFINED => (0, 0),
        }
    }
}
pub struct Map {
    pub grid: Vec<Vec<char>>,
    pub gpos: (i32, i32),
    pub direction: Direction,
}

impl Map {
    pub fn find_guard(&mut self, guard: char) -> Result<usize, String> {
        for i in 0..self.grid.len() {
            for j in 0..self.grid[0].len() {
                if self.grid[i][j] == guard {
                    self.gpos = (i as i32, j as i32);
                    self.grid[i][j] = '.';
                    match guard {
                        '>' => self.direction = Direction::RIGHT,
                        '<' => self.direction = Direction::LEFT,
                        '^' => self.direction = Direction::UP,
                        _ => self.direction = Direction::NOTDEFINED,
                    }
                    return Ok(0);
                }
            }
        }

        Err(String::from("Guard not found"))
    }

    pub fn display(&self) {
        for i in 0..self.grid.len() {
            for j in 0..self.grid[0].len() {
                print!("{}", self.grid[i][j]);
            }
            println!("");
        }
    }
}
