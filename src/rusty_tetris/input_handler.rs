
use doryen_rs::InputApi;

use super::{RustyTetris, GameEvent};

// Struct that maps a key to a cooldown and a category to handle activation of inputs 
pub struct KeyMap {
    pub key_text: String,
    pub category: String,
    pub cooldown: Option<u8>,
    pub timer: u8,
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

const CONTROLLER: [[&str; 2]; 7] = [
    ["KeyQ",        "Coma"   ],
    ["KeyE",        "Period" ],
    ["ArrowUp",     "Numpad8"],
    ["ArrowDown",   "Numpad5"],
    ["ArrowLeft",   "Numpad4"],
    ["ArrowRight",  "Numpad6"],
    ["Space",       "KeyM"   ],
];

// defines the InputHandler trait to state the following functions on a Struct that implements it
pub trait InputHandler {

    // to register the inputs that should be verified
    fn register_inputs (&mut self);

    // to verify and trigger previously registered inputs 
    fn handle_input(&mut self, input: &mut dyn InputApi, category: &str) -> Option<GameEvent>;
}

// implement the InputHandler trait on RustyTetris
impl InputHandler for RustyTetris {

    // register the game inputs 
    fn register_inputs (&mut self) {
        let player = match self.player { Some(p) => p, None=> 0 };
        println!("inputmap init for player {}: ", player);

        println!("CONTROLLER[0][player]: {}", CONTROLLER[0][player]);
        println!("CONTROLLER[1][player]: {}", CONTROLLER[1][player]);
        println!("CONTROLLER[2][player]: {}", CONTROLLER[2][player]);
        println!("CONTROLLER[3][player]: {}", CONTROLLER[3][player]);
        println!("CONTROLLER[4][player]: {}", CONTROLLER[4][player]);
        println!("CONTROLLER[5][player]: {}", CONTROLLER[5][player]);
        println!("CONTROLLER[6][player]: {}", CONTROLLER[6][player]);

        self.inputmap = vec![

            // KeyMap::new("Backspace",                "priority", None ),
            KeyMap::new("Enter",                    "priority", None ),

            KeyMap::new(CONTROLLER[0][player],     "game", Some(6) ),
            KeyMap::new(CONTROLLER[1][player],     "game", Some(6) ),
            KeyMap::new(CONTROLLER[2][player],     "game", Some(0) ),
            KeyMap::new(CONTROLLER[3][player],     "game", Some(0) ),
            KeyMap::new(CONTROLLER[4][player],     "game", Some(6) ),
            KeyMap::new(CONTROLLER[5][player],     "game", Some(6) ),

            KeyMap::new(CONTROLLER[6][player],     "game", None ),

        ];

    }

    // verify and trigger inputs of frame
    fn handle_input(&mut self, input: &mut dyn InputApi, category: &str) -> Option<GameEvent> {

        // loop through all registered inputs
        for index in 0..self.inputmap.len() {

            if self.inputmap[index].category != category { continue; }

            // if trigger returns true, match the key to call the function
            if self.inputmap[index].trigger(input).to_owned() { //match self.inputmap[index].key_text.as_str() {

                let key = self.inputmap[index].key_text.as_str();
                let player = match self.player { Some(p) => p, None=> 0 };

                if key == "" {}

                // priority (checked before paused)
                else if key == "Backspace"                { self.reset() }
                else if key == "Enter"                    { self.pause() }

                // default game inputs (checked if not paused)
                else if key == CONTROLLER[0][player] { self.rotate(true) }
                else if key == CONTROLLER[1][player] { self.rotate(false) }
                else if key == CONTROLLER[2][player] { self.intent_y(2) }
                else if key == CONTROLLER[3][player] { self.intent_y(16) }
                else if key == CONTROLLER[4][player] { self.intent_x(-1) }
                else if key == CONTROLLER[5][player] { self.intent_x(1) }
                else if key == CONTROLLER[6][player] { self.skip() }

                // no key ? probably a overlook
                /*_=>*/ else { println!("{}.handle_input: Key '{}' is registered but not mapped!", std::any::type_name::<Self>(), self.inputmap[index].key_text)}
            }
        }
        None

    }
}