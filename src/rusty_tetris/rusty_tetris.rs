extern crate doryen_rs; use doryen_rs::Console;
use crate::data::*;
use crate::Bag;

use crate::DEBUG_MOVEMENT;

const DEFAULT_MOVE_Y_COOLDOWN: u32 = 240;

use super::HasBag;
use super::InputHandler;
use super::routine_handler::RoutineHandler;
// use super::HasBag;

// defines the values that the move_intent resets to
pub const RESET_MOVE_INTENT_MANUAL: (i8, i8) = (0, 0);
pub const RESET_MOVE_INTENT_AUTO: (i8, i8) = (0, 4);

// sizes of the playfield array
pub const PLAYFIELD_WIDTH: u8 = 10;
pub const PLAYFIELD_HEIGHT: u8 = 24;

// defines the size of each block of a Tetromino
pub const BLOCK_SCALE: u8 = 2;

// enum that defines the current state of a RustyTetris run
pub enum RunState {
    Start,
    Playing,
    Paused,
    Over,
}

// Rusty Tetris engine definition
pub struct RustyTetris {
    pub playfield: [[Option<RTColor>; PLAYFIELD_HEIGHT as usize]; PLAYFIELD_WIDTH as usize],
    pub playfield_con: Option<Console>,
    pub move_y_cooldown: u32,
    pub bag_queue: Option<Bag>,
    pub cur_tetromino: Option<Tetromino>,
    pub cur_con: Option<Console>,
    pub next_con: Option<Console>,
    pub cur_pos: (i8, i8),
    pub move_intent: (i8, i8),
    pub score: i32,
    pub run_state: RunState,
    pub mouse_pos: (f32, f32),
    pub inputmap: Vec::<super::KeyMap>,
    pub routines: Vec::<super::Routine>,
    pub player: Option<usize>,

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
    pub fn singleplayer () -> Self {
        Self::new(None)
    }

    // create a new instance for Some player
    pub fn versus (player: usize) -> Self {
        println!("new rusty tetris instance for player {}", player);
        Self::new(Some(player))
    }
    
    // create a new instance with defined player
    pub fn new (player: Option<usize>) -> Self {
        Self {
            playfield: Self::create_playfield(),
            playfield_con: Some(Console::new((PLAYFIELD_WIDTH * BLOCK_SCALE) as u32 + 2, (PLAYFIELD_HEIGHT * BLOCK_SCALE) as u32 + 2)),
            move_y_cooldown: DEFAULT_MOVE_Y_COOLDOWN,
            bag_queue: None,
            cur_tetromino: Default::default(),
            cur_con: None,
            next_con: None,
            cur_pos: (0, 0),
            move_intent: (0, 1),
            score: 0,
            run_state: RunState::Start,
            mouse_pos: (0.0,0.0),
            inputmap: vec![],
            routines: vec![],
            player,
        }
    }

    // sets the state of the run
    pub fn set_state (&mut self, new_state: RunState) {
        self.run_state = new_state;
    }
    
    // pauses / resumes the game
    pub fn pause (&mut self) {
        match self.run_state {
            RunState::Playing => self.set_state(RunState::Paused),
            RunState::Paused => self.set_state(RunState::Playing),
            _=> {}
        }
    }

    // resets the game
    pub fn reset(&mut self) {

        // create an empty playfield
        self.playfield = Self::create_playfield();

        // set default game speed 
        self.move_y_cooldown = DEFAULT_MOVE_Y_COOLDOWN;
        
        // register the input keys
        // use super::InputHandler;
        self.register_inputs();

        // register the routines 
        self.initialize_routines();

        // initialize the score to 0
        self.score = 0;

        // initialize state back to Start
        self.run_state = RunState::Start;

        // call next to start the game 
        self.next();
    }
    
    // define the next Tetromino of the match
    pub fn next (&mut self) {
        let t = self.bag_next();
        // let t = Tetromino { grid: vec![vec![true;1];1], color: RTColor::Green };
        let size = (t.grid[0].len() as u32, t.grid.len() as u32);
        self.cur_tetromino = Some(t);
        self.cur_con = Some(Console::new(size.0 * BLOCK_SCALE as u32, size.1 * BLOCK_SCALE as u32));
        self.cur_pos = ((PLAYFIELD_WIDTH as i8 / 2) - (size.0 as i8 / 2), 0);
        
        self.next_con = Some(Console::new(6 * BLOCK_SCALE as u32, 8 * BLOCK_SCALE as u32));

        if get_rot_correction(&self.cur_tetromino.clone().unwrap().grid, self.cur_pos, &self.playfield) != 0 {
            self.set_state(RunState::Over) 
        };
    }

    pub fn get_skip_steps (&self, t: &Tetromino) -> i8 {
        let mut steps = 0;
        loop {
            let simulated = simulate_move_y(&t, (self.cur_pos.0, self.cur_pos.1 + steps), (0, 1), &self.playfield);
            // println!("{} steps | simulated: {:?}", steps, simulated);
            if simulated.2 != 0 || simulated.3 != 0 { break steps }
            steps += 1;
        }
    }

    // skips to the next tetromino, finishing the trajectory of the current
    pub fn skip (&mut self) {
        let mut start_run = false;
        match &self.cur_tetromino {
            None => {
                println!("RustyTetris.skip() -- NO CURRENT TETROMINO (XD????)")
            },
            Some (t) => {
                match self.run_state {
                    RunState::Start => start_run = true,
                    _=> {}
                }        
                let steps = self.get_skip_steps(&t);
                self.move_cur((0, steps));
                if steps > 0 { self.reset_timer("move_y", Some("game")); }
            }
        }
        if start_run { self.set_state(RunState::Playing) }
    }

    // rotates the current Tetromino
    pub fn rotate (&mut self, clockwise: bool) {
        
        // match current tetromino
        match &mut self.cur_tetromino {

            // some tetromino
            Some(t) => {

                // get the rotated grid
                let rotated = t.get_rotated(clockwise).to_owned();

                // get the correction value 
                let correction = get_rot_correction(&rotated, self.cur_pos, &self.playfield);

                // replace the tetromino's grid
                t.set_grid(rotated);

                // if correction is not none, move the tetromino
                if correction != 0 { self._move_x(correction); }
        
            },
            
            // no current
            None => {}
        }
    }

    // moves the current Tetromino. Returns true when losing controll of cur_tetromino
    fn move_cur (&mut self, dir: (i8, i8)) -> bool {

        // check if Some current Tetromino
        match &self.cur_tetromino {
            Some(tetromino) => {  

                let simulated = simulate_move_y(&tetromino, self.cur_pos, dir, &self.playfield);

                // apply the new position
                self.cur_pos = (simulated.0 + simulated.2, simulated.1 + simulated.3);

                // Tetronimo is still current
                return simulated.3 < 0;

            }

            // no current Tetromino
            None => false
        }
    }

    // declare the intent of moving x by 'dir' in the next move_x call
    pub fn intent_x (&mut self, dir: i8) { self.move_intent.0 = (self.move_intent.0 as i32 + dir as i32).min(127) as i8 /*/.min(1).max(-1) */}

    // calls move_cur to move horizontally
    fn _move_x (&mut self, dir: i8) { self.move_cur((dir, 0)); }
    pub fn move_x (&mut self) {

        // if no intent to move x, cancel
        if self.move_intent.1 == 0 { return }

        // apply the x move
        self._move_x(self.move_intent.0);

        // reset x intent
        self.move_intent.0 = if DEBUG_MOVEMENT { RESET_MOVE_INTENT_MANUAL.0 } else { RESET_MOVE_INTENT_AUTO.0 };
    }

    // declare the intent of moving y by 'dir' in the next move_y call
    pub fn intent_y (&mut self, dir: i8) {
        match self.run_state {
            RunState::Start => self.set_state(RunState::Playing),
            _=> {}
        }
        self.move_intent.1 = dir;
        let speed = self.move_y_cooldown;
        match self.get_routine("move_y", "game") {
            None => {},
            Some(routine) => {
                routine.set_cooldown(
                    if dir < 0 { Some(speed * dir.abs() as u32) }
                    else if dir > 1 { Some(speed / dir as u32) } else { Some(speed) }
                )
            }
        }
        
    }

    // calls move_cur to move vertically
    fn _move_y (&mut self, dir: i8) -> bool { self.move_cur((0, dir)) }
    pub fn move_y (&mut self) {

        // cancel if no intent to move y
        if self.move_intent.1 == 0 { return; }

        // apply the y move
        let result = !self._move_y(if self.move_intent.1 < 1 {1} else {self.move_intent.1.signum()});

        // println!("{}", self.move_intent.1);

        self.intent_y(if DEBUG_MOVEMENT { RESET_MOVE_INTENT_MANUAL.1 } else { RESET_MOVE_INTENT_AUTO.1 });
        
        if result {return};
        
        // add the Tetromino the the playfield
        self.add_to_playfield();

        // let mut next_playfield = self.check_rows();
        let mut score_sum = 0;

        // enter dynamic loop
        loop {

            // check rows and match accordingly
            match self.check_rows() {

                // one line was cleared this iteration, new playfield returned
                Some(playfield) => {

                    // TODO: this is where some sort of animation comes into play

                    // replace the playfield
                    self.set_playfield(playfield);

                    // increase the score by 1
                    score_sum += 1;
                },

                // no lines cleared, exit loop
                None => break
            }
        }

        // if score is not 0
        if score_sum != 0 {

            // calculate and add to score
            self.score += score_sum * score_sum * 10;
            self.move_y_cooldown = (self.move_y_cooldown as i32 - (self.move_y_cooldown as i32 / 30 * score_sum)).max(0) as u32;
            println!("score: {} (+{}) | new speed: {}", self.score, score_sum * score_sum * 10, self.move_y_cooldown);
        }                    

        // lose control over the Tetromino and get the next one
        self.next();

    }

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
