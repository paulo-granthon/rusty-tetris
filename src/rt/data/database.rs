use crate::tetromino::*;
// use std::slice::Iter;
use TetrominoID::*;

pub const COUNT: usize = 7;

// defines the available Tetrominos

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TetrominoID { I, J, L, O, S, Z, T }
impl TetrominoID {

    // Returns all Tetrominos
    pub fn all() -> [TetrominoID; COUNT] {
        [I, J, L, O, S, Z, T]
    }

    // Allows the enum iteration
    // pub fn iter() -> Iter<'static, Tetrominos> {
    //     static TETROMINOS: [Tetrominos; COUNT] = [I, J, L, O, S, Z, T];
    //     TETROMINOS.iter()
    // }

    // Returns the Tetromino of the letter
    pub fn get(&self) -> Tetromino {
        use crate::RTColor::*;

        // returns a Tetromino corresponding to the enum
        match self {
            I => Tetromino::from_array([
                [false, false, false, false],
                [true,  true,  true,  true],
                [false, false, false, false],
                [false, false, false, false],
            ], Cyan),
            J => Tetromino::from_array([
                [false, false, false],
                [true,  true,  true ],
                [false, false, true ],
            ], Blue),
            L => Tetromino::from_array([
                [false, false, true ],
                [true,  true,  true ],
                [false, false, false],
            ], Orange),
            O => Tetromino::from_array([
                [true, true],
                [true, true],
            ], Yellow),
            S => Tetromino::from_array([
                [false, true,  false],
                [false, true,  true ],
                [false, false, true ],
            ], Green),
            Z => Tetromino::from_array([
                [false, false, true ],
                [false, true,  true ],
                [false, true,  false],
            ], Magenta),  
            T => Tetromino::from_array([
                [false, true, false],
                [true,  true, false],
                [false, true, false],
            ], Red),
        
        }
    }
}

// returns a random Tetromino id
pub fn random_enum () -> TetrominoID {
    use rand::Rng;
    TetrominoID::all()[rand::thread_rng().gen_range(0..COUNT)]
}

// returns a random Tetromino
// pub fn random () -> Tetromino {
//     random_enum().get()
// }
