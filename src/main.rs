extern crate doryen_rs;
pub mod data;
// use data::*;
use data::rusty_tetris::*;
// use doryen_rs::{App, AppOptions, DoryenApi, Engine, TextAlign, UpdateEvent};

// const CONSOLE_WIDTH: u32 = 80;
// const CONSOLE_HEIGHT: u32 = 45;

fn main() {

    let mut tetromino = Tetromino::from_array(
        [
            // [true, true, true, true],
            // [true, false, false, false],
            // [true, false, false, false],
            // [true, false, false, false]
            [false, false,  false,  false],
            [false, false,  false,  false],
            [true,  true,   true,   false],
            [false, true,   false,  false]

        ], 
        [-1,0]
    );

    // /*println!("{:?}", &mut */tetromino;/*);*/
    // println!("\n");

    /*println!("{:?}", &mut */tetromino.rotate(true).rotate(true);/*);*/
    // println!("{:?}", &mut tetromino.rotate(false));

    // println!("{:?}", &mut trim::<bool, 4, 4>(translate(tetromino.grid, [1, 1]), [1, 1]));


    // let mut app = App::new(AppOptions {
    //     console_width: CONSOLE_WIDTH,
    //     console_height: CONSOLE_HEIGHT,
    //     screen_width: CONSOLE_WIDTH * 8,
    //     screen_height: CONSOLE_HEIGHT * 8,
    //     window_title: "my roguelike".to_owned(),
    //     font_path: "terminal_8x8.png".to_owned(),
    //     vsync: true,
    //     fullscreen: false,
    //     show_cursor: true,
    //     resizable: true,
    //     intercept_close_request: false,
    //     max_fps: 0,
    // });
}
