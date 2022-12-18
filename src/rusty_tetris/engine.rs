use super::{RustyTetris, RTColor};
use super::render::*;
extern crate doryen_rs; use doryen_rs::{DoryenApi, Engine, TextAlign, UpdateEvent};

use crate::DEBUG_MOVEMENT;
use crate::DEBUG_RENDER;
use crate::CONSOLE_WIDTH;
use crate::PLAYFIELD_WIDTH;
use crate::CONSOLE_HEIGHT;
use crate::PLAYFIELD_HEIGHT;
use crate::BLOCK_SCALE;

// render position of the playfield
const R_PLAYFIELD_X: i32 = CONSOLE_WIDTH as i32 / 2 - (PLAYFIELD_WIDTH * BLOCK_SCALE) as i32 / 2 - 1;
const R_PLAYFIELD_Y: i32 = CONSOLE_HEIGHT as i32 / 2 - (PLAYFIELD_HEIGHT * BLOCK_SCALE) as i32 / 2 - 1;

// render sizes of the playfield
const R_PLAYFIELD_SIZE_X: u32 = (PLAYFIELD_WIDTH * BLOCK_SCALE) as u32 + 2;
const R_PLAYFIELD_SIZE_Y: u32 = (PLAYFIELD_HEIGHT * BLOCK_SCALE) as u32 + 2;

// Doryen engine implementation for RustyTetris
impl Engine for RustyTetris {

    // initialize the engine
    fn init(&mut self, api: &mut dyn DoryenApi) {

        // register colors 
        for color in RTColor::iter() {
            api.con().register_color(color.value().0, color.value().1);
        }

        // get the first Tetromino for the match
        self.next();
    }

    // Called every frame
    fn update(&mut self, api: &mut dyn DoryenApi) -> Option<UpdateEvent> {

        // get the current input
        let input = api.input();

        self.mouse_pos = input.mouse_pos();

        let input_text = input.text();
        // if !input_text.is_empty() { println!("{}", input_text); }

        match &input_text as &str { 
            "q" | "Q" => self.rotate(true),
            "e" | "E" => self.rotate(false),
            " " => self.next(),
            _=> {}
        };

        // if input.key_pressed("q") { self.rotate(true) }
        // if input.key_pressed("e") { self.rotate(false) }

        if input.key("Backspace") {
            self.reset();
        }

        if input.key_pressed("Enter") {
            println!("Paused");
            self.paused = !self.paused;
        }

        if self.paused { return None }

        let update_cooldown = 
            if self.move_intent.1 < 0 { self.tick_delay * (self.move_intent.1 * -1) as usize } 
            else if self.move_intent.1 > 0 { self.tick_delay / self.move_intent.1 as usize } 
            else { self.tick_delay };

        // println!("{} -> {}", self.move_intent.1, update_cooldown);
        if self.t < update_cooldown {
            // println!("{}/{}", self.t, self.tick_delay);
            self.t += 1;
            return None;
        }
        self.t = 0;

        self.move_intent = (

            // // move left/right
            if      input.key("ArrowLeft")  { (self.move_intent.0 - 1).max(-1) } 
            else if input.key("ArrowRight") { (self.move_intent.0 + 1).min( 1) } 
            else { 0 },

            // automatic down movement if not debugging
            if !DEBUG_MOVEMENT {

                // auto move down + speedup/slowdown
                if      input.key("ArrowUp")   { (self.move_intent.1 - 1).max(-4) } 
                else if input.key("ArrowDown") { (self.move_intent.1 + 1).min( 4) } 
                else {
                    let mut new_move_intent_y = self.move_intent.1 - self.move_intent.1.signum();
                    if new_move_intent_y == 0 { new_move_intent_y = 1; }
                    new_move_intent_y
                }
            } 
            
            // manual move if debug
            else {

                // manual move up/down
                if      input.key("ArrowUp")   { -1  } 
                else if input.key("ArrowDown") {  1  } 
                else                           {  0  }
            }
        );

        // apply movement to Tetromino
        if self.move_intent.0 != 0 || self.move_intent.1 != 0 {
            println!("{},{}", self.move_intent.0, self.move_intent.1);

            // apply the horizontal movement queued up by the player
            self.move_x();

            // apply the vertical movement; 
            // true: Tetromino collided with something
            if self.move_y() {

                // add the Tetromino the the playfield
                self.add_to_playfield();

                let mut next_playfield = self.check_rows();
                let mut score_sum = 0;

                // repeat as long as there's a next playfield
                loop {
                    match next_playfield {
                        Some(playfield) => {
                            self.set_playfield(playfield);
                            // println!("new playfield");
                            next_playfield = self.check_rows();
                            score_sum += 1;
                        },
                        None => {
                        break
                        }
                    }

                    // TODO: this is where some sort of animation comes into play

                    // replace the playfield with the next one
                    // self.set_playfield(next_playfield);

                    // println!("new playfield");
                    
                }
                    
                self.score += score_sum * score_sum * 10;
                println!("score: {}", self.score);

                // lose control over the Tetromino and get the next one
                self.next()
            }

            // reset the move_intent after fulfilling both movement axes
            // self.reset_move_intent();
        } 

        // capture the screen
        // if input.key("ControlLeft") && input.key_pressed("KeyS") {
        //     self.screenshot_idx += 1;
        //     return Some(UpdateEvent::Capture(format!(
        //         "screenshot_{:03}.png",
        //         self.screenshot_idx
        //     )));
        // }
        None
    }

    // master render method
    fn render(&mut self, api: &mut dyn DoryenApi) {
        
        // initialize the console
        let con = api.con();
        con.clear(Some(RTColor::Black.value().1), Some(RTColor::Black.value().1), Some(' ' as u16));

        match render_playfield(self.playfield_con.as_mut(), &self.playfield, (R_PLAYFIELD_SIZE_X, R_PLAYFIELD_SIZE_Y), BLOCK_SCALE as i32) {
            Some(pfcon) => pfcon.blit(
                R_PLAYFIELD_X,
                R_PLAYFIELD_Y,
                con,
                1.0,
                1.0,
                None
            ),
            None => {}
        }

        // get a reference to the current position of the Tetromino
        let cur_pos = self.cur_pos;

        // render the current Tetromino
        match render_tetromino(self.cur_con.as_mut(), &self.cur_tetromino, BLOCK_SCALE as i32) {
            Some(cur_con) => cur_con.blit(
                (CONSOLE_WIDTH as i32 / 2) + (cur_pos.0 as i32 - (PLAYFIELD_WIDTH as i32 / 2)) * BLOCK_SCALE as i32,
                (CONSOLE_HEIGHT as i32 / 2) + (cur_pos.1 as i32 - (PLAYFIELD_HEIGHT as i32 / 2)) * BLOCK_SCALE as i32,
                con, 
                1.0,
                1.0, 
                if DEBUG_RENDER {None} else {Some(RTColor::White.value().1)}
            ),
            None => {}
        }

        let grid_mouse_pos = (
            (((self.mouse_pos.0.floor() - R_PLAYFIELD_X as f32) / BLOCK_SCALE as f32).floor() as usize).min(self.playfield.len()-1).max(0),
            (((self.mouse_pos.1.floor() - R_PLAYFIELD_Y as f32) / BLOCK_SCALE as f32).floor() as usize).min(self.playfield[0].len()-1).max(0)
        );

        con.print_color(
            (CONSOLE_WIDTH / 2) as i32,
            (CONSOLE_HEIGHT - 3) as i32,
            &format!(
                "#[white]{}: #[green]{}, {} #[white]| #[blue]{}, {}",
                if (self.mouse_pos.0 as i32) < R_PLAYFIELD_X || ((self.mouse_pos.0 as i32) >= R_PLAYFIELD_X + R_PLAYFIELD_SIZE_X as i32) || 
                   (self.mouse_pos.1 as i32) < R_PLAYFIELD_Y || (self.mouse_pos.1 as i32) >= R_PLAYFIELD_Y + R_PLAYFIELD_SIZE_Y as i32 { "oob"
                } else { match &self.playfield[grid_mouse_pos.0][grid_mouse_pos.1] {
                    Some(color) => &color.value().0,
                    None => "none"
                }},
                grid_mouse_pos.0, grid_mouse_pos.1,
                self.mouse_pos.0.floor() as i32, self.mouse_pos.1.floor() as i32,
            ),
            TextAlign::Center,
            None,
        );

        // match &self.cur_con {
        //     Some(x) => x.blit(self.cur_pos.0.into(), self.cur_pos.1.into(), con, 1.0, 1.0, None),
        //     _=>{}
        // }
        // con.area(
        //     10,
        //     10,
        //     5,
        //     5,
        //     Some((255, 64, 64, 255)),
        //     Some((128, 32, 32, 255)),
        //     Some('&' as u16),
        // );
        // con.ascii(0, 1, '@' as u16);
        // // con.fore(self.player_pos.0, self.player_pos.1, (255, 255, 255, 255));
        // con.print_color(
        //     (CONSOLE_WIDTH / 2) as i32,
        //     (CONSOLE_HEIGHT - 1) as i32,
        //     "#[red]arrows#[white] : move - #[red]CTRL-S#[white] : save screenshot",
        //     TextAlign::Center,
        //     None,
        // );
        // con.print_color(
        //     (CONSOLE_WIDTH / 2) as i32,
        //     (CONSOLE_HEIGHT - 3) as i32,
        //     &format!(
        //         "#[white]Mouse coordinates: #[red]{}, {}",
        //         self.mouse_pos.0, self.mouse_pos.1
        //     ),
        //     TextAlign::Center,
        //     None,
        // );
        // con.print_color(
        //     5,
        //     5,
        //     "#[blue]This blue text contains a #[red]red#[] word",
        //     TextAlign::Left,
        //     None,
        // );
        // con.back(
        //     self.mouse_pos.0 as i32,
        //     self.mouse_pos.1 as i32,
        //     (255, 255, 255, 255),
        // );
        render_block(
            con, 
            self.mouse_pos.0 as i32 / BLOCK_SCALE as i32, 
            self.mouse_pos.1 as i32 / BLOCK_SCALE as i32, 
            RTColor::White.value().1, 
            BLOCK_SCALE as i32, 
            0, 0
        );
    }
}

