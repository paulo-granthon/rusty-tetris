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
    pub fn trigger (&mut self, input: &mut dyn doryen_rs::InputApi) -> bool {

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
