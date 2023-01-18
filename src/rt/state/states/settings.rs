use crate::{RustyEngine, GameEvent, InputHandler, Controller, config_tracker::*, InputID, rt::render::render_popup_window};

enum Action {
    Set,
    Reset,
}

impl Action {
    pub fn text (&self) -> &str {
        match self {
            Action::Set => "Select Key",
            Action::Reset => "Reset to default",
        }
    }
}

const ACTIONS: [Action; 2] = [Action::Set, Action::Reset];

enum SubState {
    Main,
    KeySelect,
}

pub struct Settings {
    defaults: [Controller; 3],
    pub controllers: [Controller; 3],
    state: SubState,
    cursor: usize,
    button: usize,
    tab: usize,
    inputmap: Vec::<crate::KeyMap>,
}   

impl Settings {
    pub fn new () -> Self {
        Self {
            defaults: [Controller::default(), Controller::default_versus1(), Controller::default_versus2()],
            controllers: match get_controllers() { Ok(controllers) => controllers, Err(err) => panic!("state/states/settings::new() -- Error loading controllers: {}", err)},
            state: SubState::Main,
            cursor: 0,
            button: 0,
            tab: 0,
            inputmap: vec![],
        }
    }

    // resets the input at tab + cursor to it's default value
    fn reset_at (&mut self) {
        self.controllers[self.tab].set_at(self.cursor, self.defaults[self.tab].get_at(self.cursor))
    }

    // triggers the action at tab + cursor + button
    fn action (&mut self) -> Option<GameEvent> {
        match ACTIONS[self.button] {
            Action::Set => self.state = SubState::KeySelect,
            Action::Reset => self.reset_at()
        }
        None
    }

    // logic redirect to toggle_action or tab acordingly
    fn horizontal_input(&mut self, right: bool) -> Option<GameEvent> {
        match right {
            true => if self.button == 1 { self.tab(1); self.toggle_action(-1); } else { self.toggle_action(1); }
            false => if self.button == 0 { self.tab(-1); self.toggle_action(1); } else { self.toggle_action(-1); }
        };
        None
    }

    // toggles between the buttons at cursor
    fn toggle_action (&mut self, dir: i8) {
        self.button = ((((self.button as i8 + dir) % 2) + 2) % 2) as usize;
    }

    // switches between tabs
    fn tab (&mut self, dir: i8) {
        let new = ((self.tab as i8 + dir) + 3) as usize % 3;
        if new != self.tab {
            self.tab = new;
        }
    }

    // scrolls through the content 
    fn scroll (&mut self, dir: i8) -> Option<GameEvent> {
        self.cursor = ((self.cursor as i8 + dir) + 8) as usize % 8;
        None
    }

    // Escape command to back track
    fn escape (&mut self) -> Option<GameEvent> {

        // if currently waiting for key select, back to settings
        match self.state {
            SubState::KeySelect => {
                self.state = SubState::Main;
                None
            },
            SubState::Main => {
                println!("saving config");
                let _ = save_controllers(&mut self.controllers);
                Some(GameEvent::PreviousState)
            }
        }        
    }

}

impl RustyEngine for Settings {

    // engine initialization
    fn init(&mut self) {
        self.register_inputs()
    }

    // engine update
    fn update(&mut self, api: &mut dyn doryen_rs::DoryenApi) -> (Option<GameEvent>, Option<doryen_rs::UpdateEvent>) {
        let input = api.input();
        (self.handle_input(input, match self.state {
            SubState::Main => "main",
            SubState::KeySelect => "selk",
        }), None)
    }

    fn render(&mut self, api: &mut dyn doryen_rs::DoryenApi) {
        use crate::{RTColor, Align, render_rect, render_button, CONSOLE_HEIGHT, CONSOLE_WIDTH};
        
        // get the console
        let con = api.con();

        let white = RTColor::White;
        let red = RTColor::Red;
        let dark_gray = RTColor::DarkGray.u8();
        let darker_gray = RTColor::DarkerGray.u8();
        let black = RTColor::Black;

        // get the current controller
        let controller = &self.controllers[self.tab];

        // scrollbar
        // render_rect(con, CONSOLE_WIDTH as i32, 8, 3, CONSOLE_HEIGHT - 11, Some(('|', darker_gray)), Some(black.u8()), (Align::End, Align::Start));
        //     let max_list_len = 8 as i32;
        //     let scrollbar_height = (CONSOLE_HEIGHT as i32 - 11 - (max_list_len - 14)).max(1) as u32;
        //         render_rect(con, CONSOLE_WIDTH as i32, 8 + self.scroll_pos as i32, 3, scrollbar_height, Some((' ', darker_gray)), Some(dark_gray), (Align::End, Align::Start));

        // keys
        for i in 0..8 {
            render_rect(con, 0, i as i32 * 5 + 5, CONSOLE_WIDTH - 3, 5, None, Some(darker_gray), Align::start2());
            con.print(11, i as i32 * 5 + 7, InputID::from_index(i).as_str(), doryen_rs::TextAlign::Center, Some(white.u8()), None);
            con.print(30, i as i32 * 5 + 7, controller.get_at(i), doryen_rs::TextAlign::Center, Some(white.u8()), None);
            for j in 0..ACTIONS.len() {
                let selected = self.cursor == i && self.button == j;
                let color = if selected { red } else { white };
                let fore = if selected { Some(white.u8()) } else { None };
                let back = Some(if selected { white.u8() } else { darker_gray });    
                render_button(con, j as i32 * 18 + 40, i as i32 * 5 + 6, 18, 3, ACTIONS[j].text(), color, fore, back, Align::start2());
            }
        }

        
        // render title
        let title = format!("Controller Settings: {}", ["Singleplayer", "Versus: Player 1", "Versus: Player 2"][self.tab]);
        render_button(con, 0, 0, CONSOLE_WIDTH, 5, title.as_str(), [RTColor::Cyan, RTColor::Magenta, RTColor::Green][self.tab], Some(darker_gray), None, (Align::Start, Align::Start));

        // renders the Esc button 
        render_button(con, 0, 0, 7, 5, "Esc", red, Some(darker_gray), None, (Align::Start, Align::Start));

        match self.state { 
            SubState::KeySelect => {
                let half_con_width = CONSOLE_WIDTH as i32 / 2;
                let half_con_height = CONSOLE_HEIGHT as i32 / 2;
        
                render_popup_window(con, half_con_width, half_con_height, 48, 24, Align::center2(), Some(dark_gray), Some(black.u8()), Some(0));

                con.print(half_con_width, half_con_height - 2, InputID::from_index(self.cursor).as_str(), doryen_rs::TextAlign::Center, Some(white.u8()), None);
                con.print(half_con_width, half_con_height, "Press (almost) any key to rebind", doryen_rs::TextAlign::Center, Some(white.u8()), None);
            },
            _=> {}
        }


    }
}

impl InputHandler for Settings {
    fn register_inputs (&mut self) {
        self.inputmap = vec![
            crate::KeyMap::new("Enter",         "main", None ),
            crate::KeyMap::new("Escape",        "main", None ),
            crate::KeyMap::new("ArrowUp",       "main", Some(4) ),
            crate::KeyMap::new("ArrowDown",     "main", Some(4) ),
            crate::KeyMap::new("ArrowLeft",     "main", Some(6) ),
            crate::KeyMap::new("ArrowRight",    "main", Some(6) ),
        ];
    }

    fn handle_input(&mut self, input: &mut dyn doryen_rs::InputApi, category: &str) -> Option<GameEvent> {

        if category == "selk" {

            if input.key_pressed("Escape") { return self.escape() }

            let valid_keys = [
                "Digit1",
                "Digit2",
                "Digit3",
                "Digit4",
                "Digit5",
                "Digit6",
                "Digit7",
                "Digit8",
                "Digit9",
                "Digit0",
                "KeyA",
                "KeyB",
                "KeyC",
                "KeyD",
                "KeyE",
                "KeyF",
                "KeyG",
                "KeyH",
                "KeyI",
                "KeyJ",
                "KeyK",
                "KeyL",
                "KeyM",
                "KeyN",
                "KeyO",
                "KeyP",
                "KeyQ",
                "KeyR",
                "KeyS",
                "KeyT",
                "KeyU",
                "KeyV",
                "KeyW",
                "KeyX",
                "KeyY",
                "KeyZ",
                "F1",
                "F2",
                "F3",
                "F4",
                "F5",
                "F6",
                "F7",
                "F8",
                "F9",
                "F10",
                "F11",
                "F12",
                "Snapshot",
                "Insert",
                "Home",
                "Delete",
                "End",
                "PageDown",
                "PageUp",
                "ArrowLeft",
                "ArrowUp",
                "ArrowRight",
                "ArrowDown",
                "Backspace",
                "Enter",
                "Space",
                "Numpad5",
                "NumpadAdd",
                "Backslash",
                "Colon",
                "Comma",
                "NumpadDivide",
                "Equal",
                "Backquote",
                "BracketLeft",
                "Minus",
                "Period",
                "BracketRight",
                "NumpadSubtract",
                "Tab",                
            ];

            for i in 0..valid_keys.len() {
                if input.key_pressed(valid_keys[i]) {

                    self.controllers[self.tab].set_at(self.cursor, valid_keys[i]);
                    self.state = SubState::Main;
                    return None
                }

            }
        }

        // loop through all registered inputs
        for index in 0..self.inputmap.len() {
            if self.inputmap[index].category != category { continue; }

            // let im = &mut self.inputmap[index];

            // if trigger returns true, match the key to call the function
            if self.inputmap[index].trigger(input).to_owned() {
                match (self.inputmap[index].key_text.as_str(), self.inputmap[index].category.as_str()) {

                    // before paused check game inputs
                    ("Enter", "main")         => return self.action(),
                    ("Escape", "main")        => return self.escape(),
                    ("ArrowUp", "main")       => return self.scroll(-1),
                    ("ArrowDown", "main")     => return self.scroll(1),
                    ("ArrowLeft", "main")     => return self.horizontal_input(false),
                    ("ArrowRight", "main")    => return self.horizontal_input(true),

                    // no key ? probably a overlook
                    _=> { println!("{}.handle_input: Key '{}' is registered but not mapped!", std::any::type_name::<Self>(), self.inputmap[index].key_text); return None }
                }
            }
        }

        // no result
        None
    }
}