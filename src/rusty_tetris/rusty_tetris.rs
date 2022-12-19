extern crate doryen_rs; use doryen_rs::Console;
use crate::data::*;
use crate::Bag;

use crate::DEBUG_MOVEMENT;

use super::HasBag;

// defines the values that the move_intent resets to
pub const RESET_MOVE_INTENT_MANUAL: (i8, i8) = (0, 0);
pub const RESET_MOVE_INTENT_AUTO: (i8, i8) = (0, 1);

// sizes of the playfield array
pub const PLAYFIELD_WIDTH: u8 = 10;
pub const PLAYFIELD_HEIGHT: u8 = 24;

// defines the size of each block of a Tetromino
pub const BLOCK_SCALE: u8 = 2;

// Rusty Tetris engine definition
pub struct RustyTetris {
    pub playfield: [[Option<RTColor>; PLAYFIELD_HEIGHT as usize]; PLAYFIELD_WIDTH as usize],
    pub playfield_con: Option<Console>,
    pub some_bag: Option<Bag>,
    pub cur_tetromino: Option<Tetromino>,
    pub cur_con: Option<Console>,
    pub cur_pos: (i8, i8),
    pub move_intent: (i8, i8),
    pub score: i32,
    pub t:usize,
    pub paused: bool,
    pub mouse_pos: (f32, f32),
    pub inputmap: Vec::<super::KeyMap>,

}

// engine implementation
impl RustyTetris {

    // creates a blank playfield
    fn create_playfield() -> [[Option<RTColor>; PLAYFIELD_HEIGHT as usize]; PLAYFIELD_WIDTH as usize] {
        [[Default::default(); PLAYFIELD_HEIGHT as usize]; PLAYFIELD_WIDTH as usize]
    }

    // replaces the playfield
    pub fn set_playfield (&mut self, new_playfield: [[Option<RTColor>; PLAYFIELD_HEIGHT as usize]; PLAYFIELD_WIDTH as usize]) {
        self.playfield = new_playfield;
    }

    // create a new instance
    pub fn new() -> Self {
        Self {
            playfield: Self::create_playfield(),
            playfield_con: Some(Console::new((PLAYFIELD_WIDTH * BLOCK_SCALE) as u32 + 2, (PLAYFIELD_HEIGHT * BLOCK_SCALE) as u32 + 2)),
            some_bag: None,
            cur_tetromino: Default::default(),
            cur_con: None,
            cur_pos: (0, 0),
            move_intent: (0, 1),
            score: 0,
            t: 0,
            paused: false,
            mouse_pos: (0.0,0.0),
            inputmap: vec![],
        }
    }

    // pauses / resumes the game
    pub fn pause (&mut self) { self.paused = !self.paused }

    // resets the game
    pub fn reset(&mut self) {
        self.playfield = Self::create_playfield();
        self.score = 0;
        self.next();
    }

    // define the next Tetromino of the match
    pub fn next (&mut self) {
        let t = self.bag_next();
        // let t = Tetromino { grid: vec![vec![true;1];1], color: RTColor::Green };
        let size = (t.grid[0].len() as u32, t.grid.len() as u32);
        self.cur_tetromino = Some(t);
        self.cur_con = Some(Console::new(size.0 * BLOCK_SCALE as u32, size.1 * BLOCK_SCALE as u32));
        self.cur_pos = ((PLAYFIELD_WIDTH as i8 / 2) - (size.0 as i8 / 2), 0)
    }

    // rotates the current Tetromino
    pub fn rotate (&mut self, clockwise: bool) {
        match &mut self.cur_tetromino {
            Some(t) => {
                let rotated = t.get_rotated(clockwise).to_owned();
                t.set_grid(rotated);
            },
            None => {}
        }
        let correction = get_rot_correction(&self.cur_tetromino.as_ref().unwrap().grid, self.cur_pos, &self.playfield);
        if correction != 0 { self._move_x(correction); }
    }

    // moves the current Tetromino
    fn move_cur (&mut self, dir: (i8, i8)) -> bool {

        // check if Some current Tetromino
        match &self.cur_tetromino {
            Some(tetromino) => {  
                
                let width = tetromino.grid[0].len();
                let height = tetromino.grid.len();
                
                // calculate the new position of the Tetromino by clamping it a bit over the palyfield
                // since collision is defined by the Tetromino's grid instead of bounding box
                let new_pos = clamp_boundaries(
                    (self.cur_pos.0 + dir.0, self.cur_pos.1 + dir.1),
                    (-(width as i8), -(height as i8)),
                    (PLAYFIELD_WIDTH as i8, PLAYFIELD_HEIGHT as i8),
                );

                // calculate the correcttion value to further clamp the Tetromino inside the playfield
                let mut correction: (i8, i8) = get_correction(
                    &tetromino.grid, 
                    new_pos, 
                    dir, 
                    (PLAYFIELD_WIDTH as i8, PLAYFIELD_HEIGHT as i8)
                );

                // calculate the correction value in regards to collision with other Tetrominos on the playfield
                let collision: (i8, i8) = get_collision(
                    &tetromino.grid, 
                    self.cur_pos, 
                    dir, 
                    &self.playfield
                );

                // // debugging :D
                // if correction.0 != 0 || correction.1 != 0 || collision.0 != 0 || collision.1 != 0{
                //     println!("{}, {}", &self.playfield.len(), &self.playfield[1].len());
                //     println!("correction: ({}, {})\tcollision: ({}, {})", correction.0, correction.1, collision.0, collision.1);
                // }
                
                // pick the biggest correction as the actual correction that should be applied to the position of the Tetromino
                correction = (
                    if collision.0.abs() > correction.0.abs() { collision.0 } else {correction.0 },
                    if collision.1.abs() > correction.1.abs() { collision.1 } else {correction.1 }
                );

                // // mode debugging :D
                // if correction.0 != 0 || correction.1 != 0 { println!("correction result: {}, {}", correction.0, correction.1)}

                // apply the new position
                self.cur_pos = (new_pos.0 + correction.0, new_pos.1 + correction.1);

                // Tetronimo is still current
                return correction.1 < 0;

            }

            // no current Tetromino
            None => false
        }
    }

    // declare the intent of moving x by 'dir' in the next move_x call
    pub fn intent_x (&mut self, dir: i8) { self.move_intent.0 = (self.move_intent.0 + dir).min(1).max(-1) }

    // calls move_cur to move horizontally
    pub fn move_x (&mut self) -> bool { self._move_x(self.move_intent.0) }
    fn _move_x (&mut self, dir: i8) -> bool { self.move_cur((dir, 0)) }

    // declare the intent of moving y by 'dir' in the next move_y call
    pub fn intent_y (&mut self, dir: i8) { self.move_intent.1 = (self.move_intent.1 + dir).min(4).max(-4) }

    // calls move_cur to move vertically
    pub fn move_y (&mut self) -> bool { self._move_y(if self.move_intent.1 < 1 {1} else {self.move_intent.1.signum()}) }
    fn _move_y (&mut self, dir: i8) -> bool { self.move_cur((0, dir)) }

    // resets the current move_intent
    pub fn reset_move_intent (&mut self) { self.move_intent = if DEBUG_MOVEMENT { RESET_MOVE_INTENT_MANUAL } else { RESET_MOVE_INTENT_AUTO }}

    // adds the current Tetromino to the playfield as solid blocks
    pub fn add_to_playfield (&mut self) {

        // unwrap the current Tetromino
        match &self.cur_tetromino {

            // no bugs
            Some(t) => {

                // loop through the Tetromino's ggrid
                for y in 0..t.grid.len() as usize {
                    for x in 0..t.grid[0].len() as usize {

                        // if no block at position, skip
                        if !t.grid[x][y] { continue; }

                        // get the target x and y of the block
                        let target_x = self.cur_pos.0 + x as i8;
                        let target_y = self.cur_pos.1 + y as i8;

                        // check boundaries of playfield and skip if outside
                        if {
                            let mut oob = false;
                            if target_x < 0 || target_y < 0 || target_x >= self.playfield.len() as i8 { oob = true; }
                            if target_y >= self.playfield[0].len() as i8 { oob = true }
                            oob
                        } { continue; }

                        // add the block at the position to the playfield
                        self.playfield[target_x as usize][target_y as usize] = Some(t.color);
                    }
                }
            }

            // bugs 
            None => {}
        }

    }

    // checks if there are full rows, destroys them and increases the score if any
    pub fn check_rows (&mut self) -> Option<[[Option<RTColor>; PLAYFIELD_HEIGHT as usize]; PLAYFIELD_WIDTH as usize]> {

        // println!("\nchecking rows");

        // initialize the new playfield
        let mut new_playfield = [[None; PLAYFIELD_HEIGHT as usize]; PLAYFIELD_WIDTH as usize];

        // initialize a index offset to remove rows
        let mut y_offset = 0;

        // loop through the rows
        for y in (0..PLAYFIELD_HEIGHT as usize).rev() {
            // println!("{}", y);

            // if the row is full
            if { let mut row_is_full = true;
            // let mut row_is_full = true;

                // loop through the blocks on this row
                for x in 0..PLAYFIELD_WIDTH as usize {

                    // if any block is empty, row is not full
                    match self.playfield[x][y] {
                        None => { row_is_full = false; break; }
                        Some(_) => {  }
                    }
                }

                // println!("Row [{}] is {}", y, if row_is_full {"full"} else {"not full"});

                // return the result of the verification
                row_is_full
                
            } && y_offset == 0 {
            // if row_is_full {

                // println!("Row [{}] is full", y);

                // increase the y_offset to use the same y index on the next iteration
                y_offset += 1;
                // continue;

            }

            // else { println!("Row [{}] is empty", y); }

            // copy this row to the new playfield
            for x in 0..PLAYFIELD_WIDTH as usize {
                let pick_y = (y as i8 - y_offset as i8) as usize;
                new_playfield[x][y] = if pick_y < PLAYFIELD_HEIGHT as usize { self.playfield[x][pick_y] } else { None }
            }

        }

        // return the new playfield if at least one row is erased
        if y_offset > 0 { Some(new_playfield) } else { None }

    }

}
