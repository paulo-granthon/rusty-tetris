use super::{generics::*, rt_color::RTColor, TetrominoID};

// a Tetris piece
#[derive(Clone)]
pub struct Tetromino {
    pub id: TetrominoID,
    pub grid: Vec<Vec<bool>>,
    pub color: RTColor,
    pub rotation: u8,
}

// debug formatter
use core::fmt;
impl fmt::Debug for Tetromino {
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.str())
    }
}

// Tetromino's mechanics implementation
impl Tetromino {

    // create a new Tetromino from array
    pub fn from_array<const W: usize, const H: usize>(id: TetrominoID, grid:[[bool;W];H], color:RTColor) -> Self {
        Tetromino {
            id,
            grid:(0..W).map(|x| (0..W).map(|y| grid[x][y]).collect::<Vec<bool>>()).collect::<Vec<Vec<bool>>>(),
            color,
            rotation: 0,
        }
    }

    // returns the Tetrommino as a String 
    pub fn str (&self) -> String {
        self.grid.str()
    }

    // overwrites the Tetromino's grid
    pub fn set_grid (&mut self, new_grid: Vec<Vec<bool>>) {
        self.grid = new_grid
    }

    // rotates the Tetromino
    pub fn get_rotated (&mut self, clockwise: bool) -> Vec<Vec<bool>> {
        rotate(&self.grid, clockwise)
    }

}

trait _2D {
    fn str (&self) -> String;
}

impl _2D for Vec<Vec<bool>> {

    // returns the Tetrommino as a String 
    fn str (&self) -> String {
        (0..self[0].len()).map(|y| String::from_iter(
            (0..self.len()).map(|x| 
                if self[x][y] {'#'} else {'_'}   
            ).collect::<Vec<char>>()
        )).collect::<Vec<String>>().join("\n")
    }
}
impl fmt::Debug for dyn _2D {
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.str())
    }
}
