pub mod data; use data::*;

extern crate doryen_rs; use doryen_rs::{App, AppOptions, DoryenApi, Engine, TextAlign, UpdateEvent, Console};

const CONSOLE_WIDTH: u32 = 80;
const CONSOLE_HEIGHT: u32 = 80;
const MAX_FPS: usize = 60;

const UPDATE_COOLDOWN: usize = 2;

const PLAYFIELD_X: i32 = CONSOLE_WIDTH as i32 / 2 - (PLAYFIELD_WIDTH * BLOCK_SCALE) as i32 / 2 - 1;
const PLAYFIELD_Y: i32 = CONSOLE_HEIGHT as i32 / 2 - (PLAYFIELD_HEIGHT * BLOCK_SCALE) as i32 / 2 - 1;


const PLAYFIELD_SIZE_X: u32 = (PLAYFIELD_WIDTH * BLOCK_SCALE) as u32 + 2;
const PLAYFIELD_SIZE_Y: u32 = (PLAYFIELD_HEIGHT * BLOCK_SCALE) as u32 + 2;

const PLAYFIELD_WIDTH: u8 = 10;
const PLAYFIELD_HEIGHT: u8 = 24;

const BLOCK_SCALE: u8 = 2;

const DEBUG_RENDER: bool = false;

pub struct RustyTetris {
    playfield: [[Option<RTColor>; PLAYFIELD_HEIGHT as usize]; PLAYFIELD_WIDTH as usize],
    playfield_con: Option<Console>,
    cur_tetromino: Option<Tetromino>,
    cur_con: Option<Console>,
    cur_pos: (i8, i8),
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
        let t = tetro_lib::random();
        // let t = Tetromino { grid: vec![vec![true;1];1], color: RTColor::Green };
        let size = (t.grid[0].len() as u32, t.grid.len() as u32);
        self.cur_tetromino = Some(t);
        self.cur_con = Some(Console::new(size.0 * BLOCK_SCALE as u32, size.1 * BLOCK_SCALE as u32));
        self.cur_pos = ((PLAYFIELD_WIDTH as i8 / 2) - (size.0 as i8 / 2), 0)
    }

    // moves the current Tetromino
    pub fn move_cur (&mut self, dir: (i8, i8)) -> bool {

        // check if Some current Tetromino
        match &self.cur_tetromino {
            Some(tetromino) => {  
                
                let width = tetromino.grid[0].len();
                let height = tetromino.grid.len();
                
                // calculate the new position of the Tetromino by clamping it a bit over the palyfield
                // since collision is defined by the Tetromino's grid instead of bounding box
                let new_pos = (
                    (self.cur_pos.0 + dir.0).min(PLAYFIELD_WIDTH as i8).max(-(width as i8)), 
                    (self.cur_pos.1 + dir.1).min(PLAYFIELD_HEIGHT as i8).max(-(height as i8))
                );

                // if new_pos.1 >= ((PLAYFIELD_HEIGHT) * BLOCK_SCALE) as i8 { println!("TOUCHING!!!") }

                // calculate the correcttion value to further clamp the Tetromino inside the playfield
                let mut correction: (i8, i8) = (0, 0);
                let mut cur_cor_x: i8 = 0;
                let mut cur_cor_y: Vec<i8> = vec![0; height];
                for y in 0..height {
                    let mut detected_at_y = false;
                    for x in 0..width {
                        
                        // skip if no block on grid at x y
                        if !tetromino.grid[x][y] { continue; }

                        let coll_target = ((new_pos.0 + x as i8) as usize, (new_pos.1 + y as i8) as usize);

                        // println!("y: collision target: {},{}", coll_target.0, coll_target.1); 
                        if coll_target.0 >= PLAYFIELD_WIDTH as usize || coll_target.1 >= PLAYFIELD_HEIGHT as usize {}
                        else if { match self.playfield[coll_target.0][coll_target.1] { 
                            Some(_color) => {
                                // println!("y: detected: {}", -dir.1.signum());
                                let coll_dir = (dir.0.signum(), dir.1.signum());
                                if coll_dir.0 != 0 {
                                    cur_cor_x -= coll_dir.0;
                                }
                                if coll_dir.1 != 0 && ! detected_at_y { 
                                    cur_cor_y[y] -= coll_dir.1;
                                    detected_at_y = true;
                                }
                                true
                            },
                            None => false
                        }} { continue; }

                        // check x boundary
                        if  new_pos.0 + x as i8 >= PLAYFIELD_WIDTH as i8  { cur_cor_x    -= 1 }
                        else if (new_pos.0 + x as i8) < 0                 { cur_cor_x    += 1 }

                        // skip y collision if collision is already detected for this (y,x)
                        if detected_at_y { continue; }

                        // check y boundary
                        if  new_pos.1 + y as i8 >= PLAYFIELD_HEIGHT as i8 { cur_cor_y[y] -= 1; detected_at_y = true; }
                        else if (new_pos.1 + y as i8) < 0                 { cur_cor_y[y] += 1; detected_at_y = true; }

                    }
                    if cur_cor_x.abs() > correction.0.abs() { correction.0 = cur_cor_x }
                    cur_cor_x = 0;
                }
                for cor_y in cur_cor_y {
                    if cor_y.abs() > correction.1.abs() { correction.1 = cor_y }
                }

                // if correction.0 != 0 || correction.1 != 0 { println!("correction: {}, {}", correction.0, correction.1)}

                // apply the new position
                self.cur_pos = (new_pos.0 + correction.0, new_pos.1 + correction.1);

                // Tetronimo is still current
                return correction.1 < 0;

            }
            None => false
        }
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

    // renders the playfield grid
    pub fn render_playfield (&mut self) -> Option<&mut Console> {

        match &mut self.playfield_con {
            Some(pfcon) => {
                // let mut pfcon = Console::new(PLAYFIELD_Xs, PLAYFIELD_Ys);
                pfcon.clear(None, Some(RTColor::White.value().1), Some(' ' as u16));

                // render the playfield
                pfcon.rectangle(
                    0,
                    0,
                    PLAYFIELD_SIZE_X,
                    PLAYFIELD_SIZE_Y,
                    Some((128, 128, 128, 255)),
                    Some((0, 0, 0, 255)),
                    Some('.' as u16),
                );

                for x in 0..self.playfield.len() {
                    for y in 0..self.playfield[x].len() {
                        match self.playfield[x][y] {
                            Some(color) => {
                                for xs in 0..BLOCK_SCALE as i32 {
                                    for ys in 0..BLOCK_SCALE as i32 {
                                        let target_x = 1 + xs + (x as u8 * BLOCK_SCALE) as i32;
                                        let target_y = 1 + ys + (y as u8 * BLOCK_SCALE) as i32;
                                        pfcon.ascii(target_x, target_y, 0);
                                        // pfcon.fore(target_x, target_y, color.value().1);
                                        pfcon.back(target_x, target_y, color.value().1);
                                    }
                                }
                            },
                            None => continue
                        };
                    }
                }
                Some(pfcon)

            }
            None => None
        }


    }

    // render the current Tetromino 
    pub fn render_cur (&mut self) -> Option<&mut Console> {

        // get a reference to the current Tetromino
        let curt = self.cur_tetromino.as_ref();

        // match Some / None
        match &mut self.cur_con {

            // if Some
            Some(con) => {

                // clear the Tetromino's console 
                con.clear(None, Some(RTColor::White.value().1), Some(' ' as u16));

                match curt {
                    Some(t) => {

                        // for each position on the Tetromino's grid
                        for x in 0..t.grid.len() {
                            for y in 0..t.grid[0].len() {

                                for xs in 0..BLOCK_SCALE as i32 {
                                    for ys in 0..BLOCK_SCALE as i32 {

                                        // render this position if true, render blank if false
                                        con.back(xs + (x as u8 * BLOCK_SCALE) as i32, ys + (y as u8 * BLOCK_SCALE) as i32, if t.grid[x][y] {
                                            t.color.value().1
                                        } else {
                                            RTColor::White.value().1
                                        });
                                    }
                                }
            
                                
                            }
                        };
                    }
                    None => return None,
                }

                // return the console reference
                Some(con)
            },

            // no current Tetromino, return nothing
            None => None
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
        if self.t < self.tick_delay {
            // println!("{}/{}", self.t, self.tick_delay);
            self.t += 1;
            return None;
        }
        self.t = 0;

        // get the current input
        let input = api.input();

        self.mouse_pos = input.mouse_pos();

        if input.key("Backspace") {
            self.reset();
        }

        if input.key("Enter") {
            println!("Paused");
            self.paused = true;
        }

        if self.paused { return None }

        let input_text = input.text();
        // if !input_text.is_empty() { println!("{}", input_text); }

        match &input_text as &str { 
            "q" | "Q" => self.rotate(true),
            "e" | "E" => self.rotate(false),
            " " => self.next(),
            _=> {}
        };

        // handle left/right movementt
        if self.move_cur((
                if input.key("ArrowLeft") {-1} else if input.key("ArrowRight") { 1 } else { 0 },
                if input.key("ArrowUp") {-1} else if input.key("ArrowDown") { 1 } else { 0 }
                // 1
            )
        ) {
            match &self.cur_tetromino {
                Some(t) => {
                    for y in 0..t.grid.len() as usize {
                        for x in 0..t.grid[0].len() as usize {
                            if !t.grid[x][y] { continue; }
                            // println!("pf: [{},{}]/[{},{}] | t:[{},{}]/[{},{}]", self.cur_pos.0 as usize + x, self.cur_pos.1 as usize + y, self.playfield.len(), self.playfield[0].len(), x, y, t.grid.len(), t.grid.len());
                            let target_x = self.cur_pos.0 + x as i8;
                            let target_y = self.cur_pos.1 + y as i8;
                            if target_x < 0 || target_y < 0 || target_x >= self.playfield.len() as i8 || target_y >= self.playfield[0].len() as i8 { continue; }
                            self.playfield[target_x as usize][target_y as usize] = Some(t.color);
                        }
                    }
                }
                None => {}
            }

            self.next()

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

        match self.render_playfield() {
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
        match self.render_cur() {
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
            (((self.mouse_pos.0 as i32 - PLAYFIELD_X) / BLOCK_SCALE as i32) as usize).min(self.playfield.len()-1),
            (((self.mouse_pos.1 as i32 - PLAYFIELD_X) / BLOCK_SCALE as i32) as usize).min(self.playfield[0].len()-1)
        );

        con.print_color(
            (CONSOLE_WIDTH / 2) as i32,
            (CONSOLE_HEIGHT - 3) as i32,
            &format!(
                "#[white]{}: #[green]{}, {} #[white]| #[blue]{}, {} #[white]| #[red]{}, {}",
                if (self.mouse_pos.0 as i32) < PLAYFIELD_X || ((self.mouse_pos.0 as i32) >= PLAYFIELD_X + PLAYFIELD_SIZE_X as i32) || 
                   (self.mouse_pos.1 as i32) < PLAYFIELD_Y || (self.mouse_pos.1 as i32) >= PLAYFIELD_Y + PLAYFIELD_SIZE_Y as i32 { ""
                } else { match &self.playfield[grid_mouse_pos.0][grid_mouse_pos.1] {
                    Some(color) => &color.value().0,
                    None => ""
                }},
                grid_mouse_pos.0, grid_mouse_pos.1,
                self.mouse_pos.0 as i32, self.mouse_pos.1 as i32,
                self.mouse_pos.0, self.mouse_pos.1
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
        con.back(
            self.mouse_pos.0 as i32,
            self.mouse_pos.1 as i32,
            (255, 255, 255, 255),
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
