
use super::rusty_tetris::RustyTetris;

pub struct Routine {
    key: String,
    cooldown: Option<u8>,
    timer: u8,
}

impl Routine {
    pub fn new (key: &str, cooldown: Option<u8>) -> Self {
        Self { key: key.to_owned(), cooldown, timer: match cooldown { Some(t) => t, None => 0 } }
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
            None => false
        }
    }
}

pub trait RoutineHandler {

    fn register_routines (&mut self);

    fn handle_routines(&mut self);

}

impl RoutineHandler for RustyTetris {
    fn register_routines (&mut self) {
        self.routines = vec![
            Routine::new("move", Some(20))
        ];
    }
    fn handle_routines(&mut self) {

        // loop through all registered inputs
        for key in 0..self.routines.len() {

            // if self.routines[key].category != category { continue; }

            // if trigger returns true, match the key to call the function
            if self.routines[key].trigger().to_owned() { match self.routines[key].key.as_str() {

                // before paused check game inputs
                "move"          => self.update(),

                // no key ? probably a overlook
                _=> println!("{}.handle_routines: Key '{}' is registered but not mapped!", std::any::type_name::<Self>(), self.routines[key].key)
            }}
        }


    }
}