use crate::data::generics::*;

pub struct RustyTetris {

}


// a Tetris piece
#[derive(Clone)]
pub struct Tetromino {
    pub grid:Vec<Vec<bool>>,
    pub pivot:[i8; 2],
}
use core::fmt;
impl fmt::Debug for Tetromino {
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.str())
    }
}

impl Tetromino {

    // create a new Tetrommino
    pub fn new(grid:Vec<Vec<bool>>, pivot:[i8; 2]) -> Self {
        Tetromino { grid, pivot }
    }

    // create a new Tetromino from array
    pub fn from_array<const N: usize, const M: usize>(grid:[[bool;N];M], pivot:[i8; 2]) -> Self {
        Tetromino { 
            grid:(0..M).map(|x| (0..N).map(|y| grid[x][y]).collect::<Vec<bool>>()).collect::<Vec<Vec<bool>>>(),
            pivot
        }
    }

    // returns a list of chars for a row of index i on the Tetrominno
    pub fn row_chars(&self, i:usize) -> Vec<char> {
        (0..self.grid[i].len()).map(|j| if self.grid[i][j] { '#' } else {'_'}).collect()
    }

    // returns the Tetrommino as a String 
    pub fn str(&self) -> String {
        (0..self.grid.len()).map(|i| String::from_iter(self.row_chars(i))).collect::<Vec<String>>().join("\n")
    }

    // rotates the Tetromino
    pub fn rotate(mut self, clockwise: bool) -> Self {
        self.grid = translate(self.grid, [-self.pivot[0], -self.pivot[1]]);
        self.grid = rotate::<bool>(self.grid, clockwise);
        self.grid = trim(self.grid, [self.pivot[1], self.pivot[0]]);
        self
    }

}
