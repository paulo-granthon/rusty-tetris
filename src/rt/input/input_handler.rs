use crate::GameEvent;
use doryen_rs::InputApi;

// defines the InputHandler trait to state the following functions on a Struct that implements it
pub trait InputHandler {

    // to register the inputs that should be verified
    fn register_inputs (&mut self);

    // to verify and trigger previously registered inputs 
    fn handle_input(&mut self, input: &mut dyn InputApi, category: &str) -> Option<GameEvent>;
}
