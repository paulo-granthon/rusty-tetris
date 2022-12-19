
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
    pub fn new (key_text: String, category: String, cooldown: Option<u8>) -> Self {
        Self { key_text, category, cooldown, timer: match cooldown { Some(t) => t, None => 0 } }
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
    fn handle_input(&mut self, input: &mut dyn InputApi, category: &str);
}

// implement the InputHandler trait on RustyTetris
impl InputHandler for RustyTetris {

    // register the game inputs 
    fn register_inputs (&mut self) {
        self.inputmap = vec![

            // inputs while in game
            KeyMap::new("KeyQ".to_owned(),          "game".to_owned(), None ),
            KeyMap::new("KeyE".to_owned(),          "game".to_owned(), None ),
            KeyMap::new("BackSpace".to_owned(),     "game".to_owned(), None ),
            KeyMap::new("Enter".to_owned(),         "game".to_owned(), None ),
            // KeyMap::new("ArrowUp".to_owned(),       "game".to_owned(), Some(0) ),
            KeyMap::new("ArrowDown".to_owned(),     "game".to_owned(), Some(0) ),

            KeyMap::new("ArrowLeft".to_owned(),     "game".to_owned(), Some(6) ),
            KeyMap::new("ArrowRight".to_owned(),    "game".to_owned(), Some(6) ),

        ];

    }

    // verify and trigger inputs of frame
    fn handle_input(&mut self, input: &mut dyn InputApi, category: &str) {

        // loop through all registered inputs
        for key in 0..self.inputmap.len() {

            if self.inputmap[key].category != category { continue; }

            // if trigger returns true, match the key to call the function
            if self.inputmap[key].trigger(input).to_owned() { match self.inputmap[key].key_text.as_str() {

                // before paused check game inputs
                "KeyQ"          => self.rotate(true),
                "KeyE"          => self.rotate(false),
                "BackSpace"     => self.reset(),
                "Enter"         => self.pause(),

                // after pause check gae inputs
                "ArrowLeft"     => self.intent_x(-1),
                "ArrowRight"    => self.intent_x(1),
                "ArrowUp"       => self.intent_y(-1),
                "ArrowDown"     => self.intent_y(1),

                // no key ? probably a overlook
                _=> println!("{}.handle_input: Key '{}' is registered but no mapped to anything!", std::any::type_name::<Self>(), self.inputmap[key].key_text)
            }}
        }



    }
}