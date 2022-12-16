pub mod data; use data::*;

extern crate doryen_rs; use doryen_rs::{App, AppOptions, DoryenApi, Engine, TextAlign, UpdateEvent, Console};

const CONSOLE_WIDTH: u32 = 80;
const CONSOLE_HEIGHT: u32 = 80;

const PLAYFIELD_WIDTH: u8 = 10;
const PLAYFIELD_HEIGHT: u8 = 24;

const BLOCK_SCALE: u8 = 2;

const RENDER_DEBUG: bool = false;

pub struct RustyTetris {
    playfield: [[Option<RTColor>; PLAYFIELD_WIDTH as usize]; PLAYFIELD_HEIGHT as usize],
    cur_tetromino: Option<Tetromino>,
    cur_con: Option<Console>,
    cur_pos: (i8, i8),
    score: i32,

}

// stores match information
impl RustyTetris {

    // create a new instance
    pub fn new() -> Self {
        Self {
            playfield: [[Default::default(); PLAYFIELD_WIDTH as usize]; PLAYFIELD_HEIGHT as usize],
            cur_tetromino: Default::default(),
            cur_con: None,
            cur_pos: (0, 0),
            score: 0
        }
    }

    // define the next Tetromino of the match
    pub fn next (&mut self) {
        let t = tetro_lib::random();
        // println!("{:?}", &t);
        let size = (t.grid.len() as u32, t.grid.len() as u32);
        self.cur_tetromino = Some(t);
        self.cur_con = Some(Console::new(size.0, size.1));
        self.cur_pos = ((CONSOLE_WIDTH as i8 / 2) - (size.0 as i8 / 2), (CONSOLE_HEIGHT as i8 / 2) - (size.1 as i8 / 2))
    }

    // rotates the current Tetromino
    pub fn rotate (&mut self, clockwise: bool) {
        match &mut self.cur_tetromino {
            Some(t) => {
                // println!("\n{:?}\n", &t);
                let rotated = t.get_rotated(clockwise).to_owned();
                t.set_grid(rotated);
                // println!("\n{:?}\n", &t);
            },
            None => {}
        };
        
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
                            for y in 0..t.grid[0].len(){

                                // match value at position
                                match t.grid[x][y] {

                                    // render this position if true, render blank if false
                                    true => con.back(x as i32, y as i32, t.color.value().1),
                                    false => con.back(x as i32, y as i32, RTColor::White.value().1)
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

        // get the current input
        let input = api.input();

        // handle left/right movementt
        if input.key("ArrowLeft")  { self.cur_pos.0 = (self.cur_pos.0 - 1).max(1); } else 
        if input.key("ArrowRight") { self.cur_pos.0 = (self.cur_pos.0 + 1).min(CONSOLE_WIDTH as i8 - 2); }
        if input.key("ArrowUp")    { self.cur_pos.1 = (self.cur_pos.1 - 1).max(1); } else 
        if input.key("ArrowDown")  { self.cur_pos.1 = (self.cur_pos.1 + 1).min(CONSOLE_HEIGHT as i8 - 2); }
        // self.mouse_pos = input.mouse_pos();

        let input_text = input.text();
        // if !input_text.is_empty() { println!("{}", input_text); }

        match &input_text as &str { 
            "q" | "Q" => self.rotate(true),
            "e" | "E" => self.rotate(false),
            " " => self.next(),
            _=> {}
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

    fn render(&mut self, api: &mut dyn DoryenApi) {
        let con = api.con();
        con.clear(Some(RTColor::Black.value().1), Some(RTColor::Black.value().1), Some(' ' as u16));

        // render the playfield
        con.rectangle(
            CONSOLE_WIDTH as i32 / 2 - (PLAYFIELD_WIDTH * BLOCK_SCALE) as i32 / 2 - 1,
            CONSOLE_HEIGHT as i32 / 2 - (PLAYFIELD_HEIGHT * BLOCK_SCALE) as i32 / 2 - 1,
            (PLAYFIELD_WIDTH * BLOCK_SCALE) as u32 + 2,
            (PLAYFIELD_HEIGHT * BLOCK_SCALE) as u32 + 2,
            Some((128, 128, 128, 255)),
            Some((0, 0, 0, 255)),
            Some('.' as u16),
        );

        let cur_pos = self.cur_pos;

        self.render_cur().unwrap().blit(
            cur_pos.0 as i32,
            cur_pos.1 as i32, 
            con, 
            1.0,
            1.0, 
            if RENDER_DEBUG {None} else {Some(RTColor::White.value().1)}
        );

        // rendedr the current tetromino
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
        max_fps: 60,
    });

    app.set_engine(Box::new(RustyTetris::new()));
    app.run();

}
