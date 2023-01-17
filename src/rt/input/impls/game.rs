use crate::{InputHandler, KeyMap, GameEvent, InputID, Game, DEBUG_MOVEMENT, rt::state::{RunState, PauseMenuAction}};

// implement the InputHandler trait on RustyTetris
impl InputHandler for Game {

    // register the game inputs 
    fn register_inputs (&mut self) {

        self.inputmap = vec![

            // KeyMap::new("Backspace",                "priority", None ),
            KeyMap::new(self.controller.get(InputID::Pause),   "priority", None ),
            KeyMap::new(self.controller.get(InputID::Left),    "priority", Some(6) ),
            KeyMap::new(self.controller.get(InputID::Right),   "priority", Some(6) ),

            KeyMap::new(self.controller.get(InputID::Up),       "game", Some(0) ),
            KeyMap::new(self.controller.get(InputID::Down),     "game", Some(0) ),
            // KeyMap::new(self.controller.get(InputID::Left),     "game", Some(6) ),
            // KeyMap::new(self.controller.get(InputID::Right),    "game", Some(6) ),
            KeyMap::new(self.controller.get(InputID::RotateL),  "game", Some(8) ),
            KeyMap::new(self.controller.get(InputID::RotateR),  "game", Some(8) ),
            KeyMap::new(self.controller.get(InputID::Skip),     "game", None ),

            KeyMap::new(self.controller.get(InputID::Pause),   "over", None ),

        ];

    }

    // verify and trigger inputs of frame
    fn handle_input(&mut self, input: &mut dyn doryen_rs::InputApi, category: &str) -> Option<GameEvent> {

        // loop through all registered inputs
        for index in 0..self.inputmap.len() {

            if self.inputmap[index].category != category { continue; }

            // if trigger returns true, match the key to call the function
            if self.inputmap[index].trigger(input).to_owned() { //match self.inputmap[index].key_text.as_str() {

                let key = self.inputmap[index].key_text.as_str();

                if key == "" {}

                // priority (checked before paused)
                else if key == "Backspace"                { self.reset() }

                // Pause
                else if key == self.controller.get(InputID::Pause) { 
                    match &mut self.run_state {
                        RunState::Paused(menu) => match menu.action() {
                            PauseMenuAction::Continue => { self.pause(); return None },
                            PauseMenuAction::Exit => { self.pause(); self.set_state(RunState::Over); return None }
                        },
                        _=> {
                            if self.inputmap[index].category == "over".to_owned() { return Some(GameEvent::GameOver); }
                            else { self.pause(); }        
                        }
                    }
                }

                // default game inputs (checked if not paused)
                // UP
                else if key == self.controller.get(InputID::Up) { 
                    match DEBUG_MOVEMENT {
                        true => { self.intent_y(-1); self.move_y() },
                        false => self.intent_y(2)
                    }
                }
                else if key == self.controller.get(InputID::Down) { 
                    match DEBUG_MOVEMENT {
                        true => { self.intent_y(1); self.move_y() },
                        false => self.intent_y(16)
                    }
                }
                else if key == self.controller.get(InputID::Left) { 
                    match &mut self.run_state {
                        RunState::Paused(menu) => menu.move_cursor(-1),
                        _=> match DEBUG_MOVEMENT {
                            true => { self.intent_x(-1); self.move_x() },
                            false => self.intent_x(-1)
                        }
                    }
                }
                else if key == self.controller.get(InputID::Right) { 
                    match &mut self.run_state {
                        RunState::Paused(menu) => menu.move_cursor(1),
                        _=> match DEBUG_MOVEMENT {
                            true => { self.intent_x(1); self.move_x() },
                            false => self.intent_x(1)
                        }
                    }
                }
                else if key == self.controller.get(InputID::RotateL)    { self.rotate(true) }
                else if key == self.controller.get(InputID::RotateR)    { self.rotate(false) }
                else if key == self.controller.get(InputID::Skip)       { self.skip() }

                // no key ? probably a overlook
                /*_=>*/ else { println!("{}.handle_input: Key '{}' is registered but not mapped!", std::any::type_name::<Self>(), self.inputmap[index].key_text)}
            }
        }
        None

    }
}