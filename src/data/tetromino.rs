use super::{generics::*, rtcolor::RTColor};

// a Tetris piece
#[derive(Clone)]
pub struct Tetromino {
    pub grid:Vec<Vec<bool>>,
    pub color:RTColor,
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

    // create a new Tetrommino
    pub fn new(grid:Vec<Vec<bool>>, color: RTColor) -> Self { Tetromino { grid, color } }

    // create a new Tetromino from array
    pub fn from_array<const W: usize, const H: usize>(grid:[[bool;W];H], color:RTColor) -> Self {
        Tetromino { 
            grid:(0..W).map(|x| (0..W).map(|y| grid[x][y]).collect::<Vec<bool>>()).collect::<Vec<Vec<bool>>>(),
            color
        }
    }

    // returns the char at the Tetromino's grid position
    pub fn char_at (&self, x:usize, y:usize, include_empty: bool) -> Option<char> {
        if self.grid[x][y] {Some('#')} else if include_empty {Some('_')} else {None}
    }

    // returns a list of chars for a column of index i on the Tetrominno
    pub fn col_chars (&self, x:usize, include_empty: bool) -> Vec<Option<char>> {
        (0..self.grid[x].len()).map(|y| self.char_at(x, y, include_empty)).collect()
    }

    // returns a list of chars for a column of index i on the Tetrominno
    pub fn row_chars (&self, x:usize, include_empty: bool) -> Vec<Option<char>> {
        (0..self.grid.len()).map(|y| self.char_at(x, y, include_empty)).collect()
    }

    // returns all Tetromino's chars 
    pub fn all_chars (&self) -> Vec<Vec<Option<char>>> {
        (0..self.grid.len()).map(|x| self.col_chars(x, false)).collect()
    }

    // returns the Tetrommino as a String 
    pub fn str (&self) -> String {
        // (0..self.grid.len()).map(|x| String::from_iter(
        //     self.row_chars(x, true).iter().flatten()
        // )).collect::<Vec<String>>().join("\n")
        self.grid.str()
    }

    // overwrites the Tetromino's grid
    pub fn set_grid (&mut self, new_grid: Vec<Vec<bool>>) {
        self.grid = new_grid
    }

    // rotates the Tetromino
    pub fn get_rotated (&mut self, clockwise: bool) -> Vec<Vec<bool>> {

        // println!("original:\n{:?}", &self);

        // let translated = translate(&self.grid,[-self.pivot[0], -self.pivot[1]]);
        // println!("translated:\n{}", &translated.str());

        rotate(&self.grid, clockwise)
        // println!("rotated:\n{}", &rotated.str());

        // let trimmed = trim(&rotated, [self.pivot[1], self.pivot[0]]);
        // println!("trimmed:\n{}", &trimmed.str());

        // trimmed
        // rotated
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
