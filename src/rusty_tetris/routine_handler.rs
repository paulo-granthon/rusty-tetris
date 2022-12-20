
use super::rusty_tetris::RustyTetris;

pub struct Routine {
    key: String,
    category: String,
    cooldown: Option<u8>,
    timer: u8,
}

impl Routine {
    pub fn new (key: &str, category: &str, cooldown: Option<u8>) -> Self {
        Self { key: key.to_owned(), category: category.to_owned(), cooldown, timer: match cooldown { Some(t) => t, None => 0 } }
    }

    pub fn trigger (&mut self) -> bool {
        match self.cooldown {
            Some(cooldown) => {
                if self.timer < cooldown {
                    self.timer = (self.timer + 1).min(cooldown);
                    return false;
                }
                self.timer = 0;
                true
            }
            None => true
        }
    }
}

pub trait RoutineHandler {

    // to register routines 
    fn register_routines (&mut self);

    // to verify and trigger routines
    fn handle_routines(&mut self, category: &str);

    // to reset the timer of a routine
    fn reset_timer (&mut self, key: &str, category: Option<&str>);

}

// defines the RoutineHandler trait to state the following functions on a Struct that implements it
impl RoutineHandler for RustyTetris {

    // registers the following routines
    fn register_routines (&mut self) {
        self.routines = vec![
            Routine::new("move_x", "not_paused", Some(6)),
            Routine::new("move_y", "not_paused", Some(30)),
        ];
    }

    // verifies each routine and triggers them
    fn handle_routines(&mut self, category: &str) {

        // loop through all registered routines
        for index in 0..self.routines.len() {

            if self.routines[index].category != category { continue; }

            // if trigger returns true, match the key to call the function
            if self.routines[index].trigger().to_owned() { match self.routines[index].key.as_str() {

                // before paused check game routines
                "move_x"          => self.move_x(),
                "move_y"          => self.move_y(),

                // no key ? probably a overlook
                _=> println!("{}.handle_routines: Key '{}' is registered but not mapped!", std::any::type_name::<Self>(), self.routines[index].key)
            }}
        }
    }

    // resets the timer of given routine
    fn reset_timer (&mut self, key: &str, category: Option<&str>) {

        // loop through all registered routines
        for index in 0..self.routines.len() {

            // if Some category is specified and it doesn't match routine's category, skip
            match category { Some(c) => if self.routines[index].category != c { continue; }, None => {}}
            
            // if key doesn't match, skip
            if self.routines[index].key != key { continue; }

            // resets the timer of the routine
            self.routines[index].timer = 0;
        }
    }
}