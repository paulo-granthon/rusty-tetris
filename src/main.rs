pub mod data; use data::*;
pub mod render; use render::*;

extern crate doryen_rs; use doryen_rs::{App, AppOptions, DoryenApi, Engine, TextAlign, UpdateEvent, Console};

// Debug constants
const DEBUG_RENDER: bool = false;
const DEBUG_MOVEMENT: bool = false;

// doryen-rs constants
const CONSOLE_WIDTH: u32 = 80;
const CONSOLE_HEIGHT: u32 = 80;
const MAX_FPS: usize = 60;

// slows down the update rate of the game 
const UPDATE_COOLDOWN: usize = 2;

// defines the values that the move_intent resets to
const RESET_MOVE_INTENT_MANUAL: (i8, i8) = (0, 0);
const RESET_MOVE_INTENT_AUTO: (i8, i8) = (0, 1);

// sizes of the playfield array
const PLAYFIELD_WIDTH: u8 = 10;
const PLAYFIELD_HEIGHT: u8 = 24;

// defines the size of each block of a Tetromino
const BLOCK_SCALE: u8 = 2;

// render position of the playfield
const PLAYFIELD_X: i32 = CONSOLE_WIDTH as i32 / 2 - (PLAYFIELD_WIDTH * BLOCK_SCALE) as i32 / 2 - 1;
const PLAYFIELD_Y: i32 = CONSOLE_HEIGHT as i32 / 2 - (PLAYFIELD_HEIGHT * BLOCK_SCALE) as i32 / 2 - 1;

// render sizes of the playfield
const PLAYFIELD_SIZE_X: u32 = (PLAYFIELD_WIDTH * BLOCK_SCALE) as u32 + 2;
const PLAYFIELD_SIZE_Y: u32 = (PLAYFIELD_HEIGHT * BLOCK_SCALE) as u32 + 2;

// Rusty Tetris engine definition
pub struct RustyTetris {
    playfield: [[Option<RTColor>; PLAYFIELD_HEIGHT as usize]; PLAYFIELD_WIDTH as usize],
    playfield_con: Option<Console>,
    cur_tetromino: Option<Tetromino>,
    cur_con: Option<Console>,
    cur_pos: (i8, i8),
    move_intent: (i8, i8),
    score: i32,
    tick_delay:usize,
    t:usize,
    paused: bool,
    mouse_pos: (f32, f32),

}

// stores match information
impl RustyTetris {

    // creates a blank playfield
    fn create_field() -> [[Option<RTColor>; PLAYFIELD_HEIGHT as usize]; PLAYFIELD_WIDTH as usize] {
        [[Default::default(); PLAYFIELD_HEIGHT as usize]; PLAYFIELD_WIDTH as usize]
    }

    // create a new instance
    pub fn new() -> Self {
        Self {
            playfield: Self::create_field(),
            playfield_con: Some(Console::new((PLAYFIELD_WIDTH * BLOCK_SCALE) as u32 + 2, (PLAYFIELD_HEIGHT * BLOCK_SCALE) as u32 + 2)),
            cur_tetromino: Default::default(),
            cur_con: None,
            cur_pos: (0, 0),
            move_intent: (0, 1),
            score: 0,
            tick_delay: UPDATE_COOLDOWN,
            t: 0,
            paused: false,
            mouse_pos: (0.0,0.0),
        }
    }

    // resets the game
    pub fn reset(&mut self) {
        self.playfield = Self::create_field();
        self.score = 0;
        self.next();
    }

    // define the next Tetromino of the match
    pub fn next (&mut self) {
        // let t = tetro_lib::random();
        let t = Tetromino { grid: vec![vec![true;1];1], color: RTColor::Green };
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

    // calls move_cur to move horizontally
    pub fn move_x (&mut self) -> bool { self.move_cur((self.move_intent.0, 0)) }

    // calls move_cur to move vertically
    pub fn move_y (&mut self) -> bool { self.move_cur((0, self.move_intent.1)) }

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

}

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

        if input.key("Backspace") {
            self.reset();
        }

        if input.key("Enter") {
            println!("Paused");
            self.paused = true;
        }

        if self.paused { return None }

        if self.t < self.tick_delay {
            // println!("{}/{}", self.t, self.tick_delay);
            self.t += 1;
            return None;
        }
        self.t = 0;

        self.move_intent = (

            // // move left/right
            if input.key("ArrowLeft") {
                (self.move_intent.0 - 1).max(-1)
            } else if input.key("ArrowRight") {
                (self.move_intent.0 + 1).min(1)
            } else {
                self.move_intent.0
            },

            // automatic down movement if not debugging
            if !DEBUG_MOVEMENT {

                // auto move down + speedup/slowdown
                if input.key("ArrowUp")   {
                    (self.move_intent.1 - 1).max(1)
                } else if input.key("ArrowDown")  {
                    (self.move_intent.1 + 2).min(4)
                } else {
                    self.move_intent.1
                }
            } 
            
            // manual move if debug
            else {

                // manual move up/down
                if input.key("ArrowUp")   {
                    (self.move_intent.1 - 1).max(-1)
                } else if input.key("ArrowDown")  {
                    (self.move_intent.1 + 1).min(1)
                } else {
                    self.move_intent.1
                }
            }
        );

        // apply movement to Tetromino
        if self.move_intent.0 != 0 || self.move_intent.1 != 0 {

            // apply the horizontal movement queued up by the player
            self.move_x();

            // apply the vertical movement; 
            // true: Tetromino collided with something
            if self.move_y() {

                self.add_to_playfield();

                self.next()
            }
            self.reset_move_intent();
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

        match render_playfield(self.playfield_con.as_mut(), &self.playfield, (PLAYFIELD_SIZE_X, PLAYFIELD_SIZE_Y), BLOCK_SCALE as i32) {
            Some(pfcon) => pfcon.blit(
                PLAYFIELD_X,
                PLAYFIELD_Y,
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
            (((self.mouse_pos.0 as i32 - PLAYFIELD_X) / BLOCK_SCALE as i32) as usize).min(self.playfield.len()-1).max(0),
            (((self.mouse_pos.1 as i32 - PLAYFIELD_Y) / BLOCK_SCALE as i32) as usize).min(self.playfield[0].len()-1).max(0)
        );

        con.print_color(
            (CONSOLE_WIDTH / 2) as i32,
            (CONSOLE_HEIGHT - 3) as i32,
            &format!(
                "#[white]{}: #[green]{}, {} #[white]| #[blue]{}, {}",
                if (self.mouse_pos.0 as i32) < PLAYFIELD_X || ((self.mouse_pos.0 as i32) >= PLAYFIELD_X + PLAYFIELD_SIZE_X as i32) || 
                   (self.mouse_pos.1 as i32) < PLAYFIELD_Y || (self.mouse_pos.1 as i32) >= PLAYFIELD_Y + PLAYFIELD_SIZE_Y as i32 { "oob"
                } else { match &self.playfield[(PLAYFIELD_WIDTH - 1) as usize - grid_mouse_pos.0][(PLAYFIELD_HEIGHT - 1) as usize - grid_mouse_pos.1] {
                    Some(color) => &color.value().0,
                    None => "none"
                }},
                grid_mouse_pos.0, grid_mouse_pos.1,
                self.mouse_pos.0 as i32, self.mouse_pos.1 as i32,
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

fn main() {
    let mut app = App::new(AppOptions {
        console_width: CONSOLE_WIDTH,
        console_height: CONSOLE_HEIGHT,
        screen_width: CONSOLE_WIDTH * 8,
        screen_height: CONSOLE_HEIGHT * 8,
        window_title: "Rusty Tetris by Paulo Granthon".to_owned(),
        font_path: "terminal_8x8.png".to_owned(),
        vsync: true,
        fullscreen: false,
        show_cursor: true,
        resizable: false,
        intercept_close_request: false,
        max_fps: MAX_FPS,
    });

    app.set_engine(Box::new(RustyTetris::new()));
    app.run();

}
