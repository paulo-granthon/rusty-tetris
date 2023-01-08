use crate::{RustyEngine, GameEvent, InputHandler, profile_tracker::*};

enum Action {
    Play,
    // Rename,
    // Delete,
}

const ACTIONS: [Action; 1] = [
    Action::Play,
    // Action::Rename,
    // Action::Delete,
];

pub struct Profiles {
    profiles: Vec<String>,
    pub inputmap: Vec::<crate::KeyMap>,
    pub cursor_pos: (usize, usize),
    scroll_pos: i32,
    exit: bool,
}

impl Profiles {
    pub fn new () -> Self {
        Self {
            profiles: get_profiles().unwrap(), 
            inputmap: vec![], 
            cursor_pos: (0, 0), 
            scroll_pos: 0, 
            exit: false
        }
    }

    fn play (&mut self) -> GameEvent {
        self.exit = true;
        GameEvent::SetProfile(self.cursor_pos.0)
    }

    fn action (&mut self) -> Option<GameEvent> {
        match ACTIONS[self.cursor_pos.1] {
            Action::Play => return Some(self.play()),
        };
        // None
    }

    // sets the position of the cursor 
    fn set_cursor(&mut self, x: i32, y: i32) -> Option<GameEvent> {
        let lenx = self.profiles.len() as i32;
        let leny = ACTIONS.len() as i32;
        self.cursor_pos = ((((x % lenx) + lenx) % lenx) as usize, (((y % leny) + leny) % leny) as usize);
        None
    }

    // moves the cursor towards target direction
    fn move_cursor(&mut self, x: i32, y: i32) -> Option<GameEvent> {
        if self.cursor_pos.0 >= 6 + self.scroll_pos as usize { self.scroll(x); };
        self.set_cursor(self.cursor_pos.0 as i32 + x, self.cursor_pos.1 as i32 + y)
    }

    // scrolls the contents of the list 
    fn scroll (&mut self, dir: i32) -> Option<GameEvent> {
        let len = (self.profiles.len() as i32 + 1).max(1);
        self.scroll_pos = (self.scroll_pos as i32 + dir - 14).max(0).min(len);
        None
    }


}

impl RustyEngine for Profiles {
    fn init(&mut self) {
        self.register_inputs()
    }

    fn update(&mut self, api: &mut dyn doryen_rs::DoryenApi) -> (Option<GameEvent>, Option<doryen_rs::UpdateEvent>) {
        if self.exit { return (Some(GameEvent::PreviousState), None) }
        let input = api.input();
        (self.handle_input(input, ""), None)
    }

    fn render(&mut self, api: &mut dyn doryen_rs::DoryenApi) {
        use crate::{RTColor, Align, render_rect, render_button, CONSOLE_HEIGHT, CONSOLE_WIDTH};
        
        // get the console
        let con = api.con();

        let white = RTColor::White;
        let red = RTColor::Red;
        let dark_gray = RTColor::DarkGrey.value().1;
        let darker_gray = RTColor::DarkerGrey.value().1;
        let black = RTColor::Black.value().1;

        render_rect(con, CONSOLE_WIDTH as i32, 8, 3, CONSOLE_HEIGHT - 11, Some(('|', darker_gray)), Some(black), (Align::End, Align::Start));
        if self.profiles.len() > 0 {
            let max_list_len = self.profiles.len() as i32;
            let scrollbar_height = (CONSOLE_HEIGHT as i32 - 11 - (max_list_len - 14)).max(1) as u32;
                render_rect(con, CONSOLE_WIDTH as i32, 8 + self.scroll_pos as i32, 3, scrollbar_height, Some((' ', darker_gray)), Some(dark_gray), (Align::End, Align::Start));
        }

        for i in 0..self.profiles.len() {
            let fore = if i == self.cursor_pos.0 { Some(darker_gray) } else { None };
            let back = Some(if i == self.cursor_pos.0 { white.value().1 } else { darker_gray });
            render_button(con, 1, i as i32 * 5 + 5, 20, self.profiles[i].as_str(), red, fore, back, Align::start2());
        }
        
        render_rect(con, CONSOLE_WIDTH as i32, 5, 3, 3, Some(('-', darker_gray)), Some(dark_gray), (Align::End, Align::Start));
        con.ascii(CONSOLE_WIDTH as i32 - 2, 6, 30);
        con.fore(CONSOLE_WIDTH as i32 - 2, 6, black);
        render_rect(con, CONSOLE_WIDTH as i32, CONSOLE_HEIGHT as i32, 3, 3, Some(('-', darker_gray)), Some(dark_gray), (Align::End, Align::End));
        con.ascii(CONSOLE_WIDTH as i32 - 2, CONSOLE_HEIGHT as i32 - 2,31);
        con.fore(CONSOLE_WIDTH as i32 - 2, CONSOLE_HEIGHT as i32 - 2,black);

        // render title
        render_button(con, 0, 0, CONSOLE_WIDTH, "Profiles", white, Some(darker_gray), None, (Align::Start, Align::Start));

        // renders the Esc button 
        render_button(con, 0, 0, 7, "Esc", red, Some(darker_gray), None, (Align::Start, Align::Start));

    }
}

// input handling 
impl InputHandler for Profiles {

    // register the inputs without category distinction
    fn register_inputs (&mut self) {
        self.inputmap = vec![
            crate::KeyMap::new("Enter",         "", None ),
            crate::KeyMap::new("ArrowUp",       "", Some(4) ),
            crate::KeyMap::new("ArrowDown",     "", Some(4) ),
        ];
    }

    // handle per frame inputs
    fn handle_input(&mut self, input: &mut dyn doryen_rs::InputApi, _: &str) -> Option<GameEvent> {

        // loop through all registered inputs
        for index in 0..self.inputmap.len() {

            // if trigger returns true, match the key to call the function
            if self.inputmap[index].trigger(input).to_owned() { match self.inputmap[index].key_text.as_str() {

                // before paused check game inputs
                "Enter"         => return self.action(),
                "ArrowUp"       => return self.move_cursor(-1, 0),
                "ArrowDown"     => return self.move_cursor(1,  0),
                "ArrowLeft"     => return self.move_cursor(0, -1),
                "ArrowRight"    => return self.move_cursor(0,  1),

                // no key ? probably a overlook
                _=> { println!("{}.handle_input: Key '{}' is registered but not mapped!", std::any::type_name::<Self>(), self.inputmap[index].key_text); return None }
            }}
        }

        // no result
        None
    }
}