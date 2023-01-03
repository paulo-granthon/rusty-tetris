
use super::super::Game;

pub struct Routine {
    key: String,
    category: String,
    pub cooldown: Option<u32>,
    pub timer: u32,
}

impl Routine {
    pub fn new (key: &str, category: &str, cooldown: Option<u32>) -> Self {
        Self { key: key.to_owned(), category: category.to_owned(), cooldown, timer: match cooldown { Some(t) => t, None => 0 } }
    }

    pub fn trigger (&mut self) -> bool {
        // if self.key == "move_y" { println!("{}/{:?}", self.timer, self.cooldown)}
        match self.cooldown {
            Some(cooldown) => {
                if self.timer < cooldown {
                    self.set_timer(self.timer + 1);
                    return false;
                }
                self.timer = 0;
                true
            }
            None => true
        }
    }

    pub fn set_cooldown(&mut self, new_cooldown :Option<u32>) {
        // println!("new {} cooldown: {:?} -> {:?}", self.key, self.cooldown, new_cooldown);
        self.cooldown = new_cooldown
    }

    pub fn set_timer(&mut self, new_timer: u32) {
        match self.cooldown {
            Some(cooldown) => self.timer = new_timer.min(cooldown),
            None => self.timer = new_timer
        }
    }
}

pub trait RoutineHandler {

    // to register routines 
    fn initialize_routines (&mut self);

    // to verify and trigger routines
    fn handle_routines(&mut self, category: &str);

    // to reset the timer of a routine
    fn reset_timer (&mut self, key: &str, category: Option<&str>);

    // to access routine data
    fn get_routine(&mut self, key: &str, category: &str) -> Option<&mut Routine>;

    // // to register a new routine
    // fn register_routine(&mut self, key: &str, category: &str, cooldown: Option<u8>) -> &Routine;

    // // to unregister a routine
    // fn unregister_routine(&mut self, key: &str, category: &str);

    // // to replace a routine
    // fn replace_routine (&mut self, key: &str, category: &str, cooldown: Option<u8>) -> &Routine;

}

// defines the RoutineHandler trait to state the following functions on a Struct that implements it
impl RoutineHandler for Game {

    // registers the following routines
    fn initialize_routines (&mut self) {
        self.routines = vec![
            Routine::new("move_x", "priority", None),
            Routine::new("move_y", "game", None),
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

    // returns the routine with given key and category
    fn get_routine(&mut self, key: &str, category: &str) -> Option<&mut Routine> {
        for index in 0..self.routines.len() {
            if self.routines[index].category != category { continue; }
            if self.routines[index].key != key { continue; }
            return Some(&mut self.routines[index]);
        }
        return None;
    }

    // registers a new routine
    // fn register_routine(&mut self, key: &str, category: &str, cooldown: Option<u8>) -> &Routine {
    //     let routine = Routine::new(key, category, cooldown);
    //     self.routines.push(routine);
    //     let index = self.routines.iter().position(|r| &r.key == key && &r.category == category).unwrap();
    //     &self.routines[index]
    // }

    // // removes a routine
    // fn unregister_routine(&mut self, key: &str, category: &str) {
    //     let index = self.routines.iter().position(|r| &r.key == key && &r.category == category).unwrap();
    //     self.routines.swap_remove(index);
    // }

    // // replaces a routine
    // fn replace_routine (&mut self, key: &str, category: &str, cooldown: Option<u8>) -> &Routine{
    //     self.unregister_routine(key, category);
    //     self.register_routine(key, category, cooldown)
    // }
    
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
            break;
        }
    }


}