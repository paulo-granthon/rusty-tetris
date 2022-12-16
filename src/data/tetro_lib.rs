use crate::tetromino::*;
use std::slice::Iter;
use Tetrominos::*;

const COUNT: usize = 7;

// defines the available Tetrominos
enum Tetrominos { I, J, L, O, S, Z, T }
impl Tetrominos {

    // Returns all Tetrominos
    pub fn all() -> [Tetrominos; COUNT] {
        [I, J, L, O, S, Z, T]
    }

    // Allows the enum iteration
    pub fn iter() -> Iter<'static, Tetrominos> {
        static TETROMINOS: [Tetrominos; COUNT] = [I, J, L, O, S, Z, T];
        TETROMINOS.iter()
    }

    // Returns the Tetromino of the letter
    pub fn get(&self) -> Tetromino {
        use crate::RTColor::*;

        // returns a Tetromino corresponding to the enum
        match self {
            Tetrominos::I => Tetromino::from_array([
                [false, false, false, false],
                [false, false, false, false],
                [true,  true,  true,  true],
                [false, false, false, false],
            ], Cyan),
            Tetrominos::J => Tetromino::from_array([
                [false, false, false],
                [true,  true,  true ],
                [false, false, true ],
            ], Blue),
            Tetrominos::L => Tetromino::from_array([
                [false, false, true ],
                [true,  true,  true ],
                [false, false, false],
            ], Orange),
            Tetrominos::O => Tetromino::from_array([
                [true, true],
                [true, true],
            ], Yellow),
            Tetrominos::S => Tetromino::from_array([
                [false, true,  false],
                [false, true,  true ],
                [false, false, true ],
            ], Green),
            Tetrominos::Z => Tetromino::from_array([
                [false, false, true ],
                [false, true,  true ],
                [false, true,  false],
            ], Magenta),  
            Tetrominos::T => Tetromino::from_array([
                [false, true, false],
                [true,  true, false],
                [false, true, false],
            ], Red),
        
        }
    }
}

// returns a random Tetromino
pub fn random () -> Tetromino {
    use rand::Rng;
    Tetrominos::all()[rand::thread_rng().gen_range(0..COUNT)].get()
}
