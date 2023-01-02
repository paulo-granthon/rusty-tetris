mod rt; use rt::*;
extern crate doryen_rs; use doryen_rs::{App, AppOptions};


// Debug constants
pub const DEBUG_MOVEMENT: bool = false;
pub const DEBUG_RENDER: bool = false;

// doryen-rs constants
pub const CONSOLE_WIDTH: u32 = 80;
pub const CONSOLE_HEIGHT: u32 = 80;
pub const MAX_FPS: usize = 60;

fn main() {

    let scores = crate::rt::serialization::score_tracker::load_scores(None, None).unwrap();
    println!("scores: {:?}", scores);
    for score in scores {
        println!("player: {}, gamemode: {}, score: {}", score.0, score.1, score.2);
    }

    return;
    
    // let number: u8 = 15;
    // let other: u8 = 15;

    // println!("number: \t\t{}\t|\tbin:\t{:#010b}", number, number);
    // println!("other:  \t\t{}\t|\tbin:\t{:#010b}", other, other);

    // let number_shifted = number << 4;
    // println!("number_shifted:\t\t{}\t|\tbin:\t{:#010b}", number_shifted, number_shifted);

    // let result = number_shifted + other;

    // println!("result: \t\t{}\t|\tbin:\t{:#010b}", result, result);

    // let number_load = result >> 4;
    // let other_load = (result << 4) >> 4;

    // println!("number_load:\t\t{}\t|\tbin:\t{:#010b}", number_load, number_load);
    // println!("other_load:\t\t{}\t|\tbin:\t{:#010b}", other_load, other_load);

    // return;

    // println!("134:\t{:#010b}", 134);
    // println!("56:\t{:#010b}", 56);
    // println!("12:\t{:#010b}", 12);
    // println!("9:\t{:#010b}", 9);
    // println!("97:\t{:#010b}", 97);

    // return;
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
