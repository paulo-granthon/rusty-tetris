
use doryen_rs::InputApi;

use super::RustyTetris;

// Struct that maps a key to a cooldown and a category to handle activation of inputs 
pub struct KeyMap {
    key_text: String,
    category: String,
    cooldown: Option<u8>,
    timer: u8,
}

// implementation of functionallities of InputStatus
impl KeyMap {

    // creates a new InputStatus. Initializes it's timer with it's KeyMap cooldown value if Some
    pub fn new (key_text: &str, category: &str, cooldown: Option<u8>) -> Self {
        Self { key_text: key_text.to_owned(), category: category.to_owned(), cooldown, timer: match cooldown { Some(t) => t, None => 0 } }
    }

    // checks if key is pressed / held and returns true if input triggers
    pub fn trigger (&mut self, input: &mut dyn InputApi) -> bool {

        // match input cooldown
        match self.cooldown {

            // No cooldown, input should only when key is pressed at the current frame 
            None => input.key_pressed(&self.key_text), 

            // Some cooldown is set, input should auto trigger when reaching cooldown
            Some(cooldown) => {

                // it timer since last successfull trigger is less than input's cooldown 
                if self.timer < cooldown {

                    // increase the timer by one frame and return
                    self.timer = (self.timer + 1).min(cooldown);
                    return false;
                }

                // if key is not pressed or held, also don't trigger
                if !input.key(&self.key_text) {
                    return false
                }

                // reset timer only if input returns true
                self.timer = 0;

                // successfull trigger
                true
            }
        }        
    }

}

// defines the InputHandler trait to state the following functions on a Struct that implements it
pub trait InputHandler {

    // to register the inputs that should be verified
    fn register_inputs (&mut self);

    // to verify and trigger previously registered inputs 
    fn handle_input(&mut self, input: &mut dyn InputApi, category: &str, speed: i8);
}

// implement the InputHandler trait on RustyTetris
impl InputHandler for RustyTetris {

    // register the game inputs 
    fn register_inputs (&mut self) {
        self.inputmap = vec![

            KeyMap::new("BackSpace",     "prio", None ),
            KeyMap::new("Enter",         "prio", None ),

            KeyMap::new("KeyQ",          "game", None ),
            KeyMap::new("KeyE",          "game", None ),
            KeyMap::new("ArrowUp",       "game", Some(20) ),
            KeyMap::new("ArrowDown",     "game", Some(20) ),
            KeyMap::new("ArrowLeft",     "game", Some(6) ),
            KeyMap::new("ArrowRight",    "game", Some(6) ),

            KeyMap::new("Space",         "game", None ),

        ];

    }

    // verify and trigger inputs of frame
    fn handle_input(&mut self, input: &mut dyn InputApi, category: &str, speed: i8) {

        // loop through all registered inputs
        for index in 0..self.inputmap.len() {

            if self.inputmap[index].category != category { continue; }

            // if trigger returns true, match the key to call the function
            if self.inputmap[index].trigger(input).to_owned() { match self.inputmap[index].key_text.as_str() {

                // before paused check game inputs
                "KeyQ"          => self.rotate(true),
                "KeyE"          => self.rotate(false),
                "BackSpace"     => self.reset(),
                "Enter"         => self.pause(),

                // after pause check game inputs
                "ArrowLeft"     => self.intent_x(-1),
                "ArrowRight"    => self.intent_x(1),
                "ArrowUp"       => self.intent_y(-speed),
                "ArrowDown"     => self.intent_y(speed),
                "Space"         => self.skip(),

                // no key ? probably a overlook
                _=> println!("{}.handle_input: Key '{}' is registered but not mapped!", std::any::type_name::<Self>(), self.inputmap[index].key_text)
            }}
        }



    }
}