use super::{RustyTetris, RTColor, GameEvent};
use super::routine_handler::*;
use super::input_handler::*;
use super::render::*;

extern crate doryen_rs; use doryen_rs::{DoryenApi, UpdateEvent};

pub trait RustyEngine {
    fn init(&mut self);
    fn update(&mut self, api: &mut dyn DoryenApi) -> (Option<GameEvent>, Option<UpdateEvent>);
    fn render(&mut self, api: &mut dyn DoryenApi);
}

// Doryen engine implementation for RustyTetris
impl RustyEngine for RustyTetris {

    // initialize the engine
    fn init(&mut self) {
        self.register_inputs(0);
        self.initialize_routines();

        // get the first Tetromino for the match
        self.next();
    }

    // Called every frame
    fn update(&mut self, api: &mut dyn DoryenApi) -> (Option<GameEvent>, Option<UpdateEvent>) {

        // get the current input
        let input = api.input();
        self.mouse_pos = input.mouse_pos();
        self.handle_input(input, "priority");
        
        if self.paused { return (None, None) }
        
        // let game_speed = self.move_intent.1;

        // self.handle_routines("end");

        self.handle_input(input, "game");
        self.handle_routines("game");

        // self.check_game_status();

        // self.handle_input(input, "game");

        // capture the screen
        // if input.key("F12") && input.key_pressed("KeyS") {
        //     self.screenshot_idx += 1;
        //     return Some(UpdateEvent::Capture(format!(
        //         "screenshot_{:03}.png",
        //         self.screenshot_idx
        //     )));
        // }
        (None, None)
    }

    // master render method
    fn render(&mut self, api: &mut dyn DoryenApi) {
        
        // initialize the console
        let con = api.con();
        con.clear(Some(RTColor::Black.value().1), Some(RTColor::Black.value().1), Some(' ' as u16));

        self.rt_render(con);

    }

}

