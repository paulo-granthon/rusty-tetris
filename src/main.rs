mod rusty_tetris; use rusty_tetris::*;
extern crate doryen_rs; use doryen_rs::{App, AppOptions};


// Debug constants
const DEBUG_MOVEMENT: bool = false;
const DEBUG_RENDER: bool = false;

// doryen-rs constants
const CONSOLE_WIDTH: u32 = 80;
const CONSOLE_HEIGHT: u32 = 80;
const MAX_FPS: usize = 60;

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

    app.set_engine(Box::new(StateHandler::new()));
    app.run();

}
