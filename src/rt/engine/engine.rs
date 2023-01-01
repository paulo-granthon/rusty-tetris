use crate::routine_handler::*;
use crate::input_handler::*;
use super::super::{RustyTetris, GameEvent, RunState, RenderEngine};
// use super::render::*;

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
        self.register_inputs();
        self.initialize_routines();

        // get the first Tetromino for the match
        self.next();
    }

    // Called every frame
    fn update(&mut self, api: &mut dyn DoryenApi) -> (Option<GameEvent>, Option<UpdateEvent>) {

        // get the current input
        let input = api.input();
        self.mouse_pos = input.mouse_pos();
        
        
        // match the current state of the run
        match self.run_state {

            // Allow player to position and rotate piece freely without y movement until player presses up/down or skip
            RunState::Start => {
                self.handle_input(input, "priority");
                self.handle_input(input, "game");
                self.handle_routines("priority");
            },

            // also handle inputs but also calls routines to move y
            RunState::Playing => {
                self.handle_input(input, "priority");
                self.handle_input(input, "game");
                self.handle_routines("priority");
                self.handle_routines("game");
            },

            // handles inputs specific to the state and maybe return GameEvent
            RunState::Paused => {
                self.handle_input(input, "priority");
                // paused should open a menu with the option to quit the run, that would return GameEvent
            },

            // handle input and return GameEvent on input
            RunState::Over => {
                // return GameEvent to return to MainMenu on keypress
                // self.reset()
                return (self.handle_input(input, "over"), None);
            },
            // _=> {}
        }
        
        // if update reaches this point, return None as the resulting GameEvent
        (None, None)

    }

    // master render method
    fn render(&mut self, api: &mut dyn DoryenApi) {
        
        // initialize the console
        let con = api.con();

        self.rt_render(con);

    }

}

