use crate::{RustyEngine, GameEvent, InputHandler, profile_tracker::*};

enum Action {
    Play,
    Rename,
    Delete,
}

impl Action {
    fn text (&self) -> &str {
        match self {
            Action::Play => "Play",
            Action::Rename => "Rename",
            Action::Delete => "Delete",
        }
    }
}

const ACTIONS: [Action; 3] = [
    Action::Play,
    Action::Rename,
    Action::Delete,
];

pub struct Profiles {
    profiles: Vec<String>,
    pub inputmap: Vec::<crate::KeyMap>,
    pub cursor_pos: (usize, usize),
    pub renaming: bool,
    pub rename_temp: String,
    cursor_anim: u8,
    scroll_pos: i32,
    exit: bool,
}

impl Profiles {
    pub fn new () -> Self {
        Self {
            profiles: get_profiles().unwrap(), 
            inputmap: vec![], 
            cursor_pos: (0, 0), 
            renaming: false,
            rename_temp: String::new(),
            cursor_anim: 0,
            scroll_pos: 0, 
            exit: false,
        }
    }
    pub fn blink (&mut self) {
        self.cursor_anim = (self.cursor_anim + 1) % 20;
    }

    fn create (&mut self) -> Option<GameEvent> {
        todo!()
    }

    fn play (&mut self) -> Option<GameEvent> {
        self.exit = true;
        Some(GameEvent::SetProfile(self.cursor_pos.0))
    }

    fn rename_start (&mut self) -> Option<GameEvent> {
        self.renaming = true;
        self.rename_temp = self.profiles[self.cursor_pos.0].to_string();
        None
    }

    fn rename_conclude (&mut self) -> Option<GameEvent> {
        self.renaming = false;
        // TODO: Send to back
        self.rename_temp = String::new();
        None
    }

    fn delete(&self) -> Option<GameEvent> {
        todo!()
    }

    fn action (&mut self) -> Option<GameEvent> {
        if self.renaming { 
            return self.rename_conclude();
        }
        match ACTIONS[self.cursor_pos.1] {
            Action::Play => return self.play(),
            Action::Rename => return self.rename_start(),
            Action::Delete => return self.delete(),
        };
    }

    fn escape (&mut self) -> Option<GameEvent> {
        if self.renaming {
            self.renaming = false;
            None
        } else {
            Some(GameEvent::PreviousState)
        }
    }

    // sets the position of the cursor 
    fn set_cursor(&mut self, x: i32, y: i32) -> Option<GameEvent> {
        let lenx = self.profiles.len() as i32 + 1;
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
        self.blink();
        (self.handle_input(input, ""), None)
    }

    fn render(&mut self, api: &mut dyn doryen_rs::DoryenApi) {
        use crate::{RTColor, Align, render_rect, render_button, CONSOLE_HEIGHT, CONSOLE_WIDTH};
        
        // get the console
        let con = api.con();

        let white = RTColor::White;
        let red = RTColor::Red;
        let dark_gray = RTColor::DarkGrey.u8();
        let darker_gray = RTColor::DarkerGrey.u8();
        let black = RTColor::Black;

        // scrollbar
        render_rect(con, CONSOLE_WIDTH as i32, 8, 3, CONSOLE_HEIGHT - 11, Some(('|', darker_gray)), Some(black.u8()), (Align::End, Align::Start));
        if self.profiles.len() > 0 {
            let max_list_len = self.profiles.len() as i32;
            let scrollbar_height = (CONSOLE_HEIGHT as i32 - 11 - (max_list_len - 14)).max(1) as u32;
                render_rect(con, CONSOLE_WIDTH as i32, 8 + self.scroll_pos as i32, 3, scrollbar_height, Some((' ', darker_gray)), Some(dark_gray), (Align::End, Align::Start));
        }

        // profiles
        for i in 0..self.profiles.len() {
            render_rect(con, 0, i as i32 * 5 + 5, CONSOLE_WIDTH - 3, 5, None, Some(darker_gray), Align::start2());
            let mut text = &format!("{}{}", self.rename_temp, if self.cursor_anim < 12 { '|' } else { ' ' });
            if !self.renaming || self.cursor_pos.0 != i { text = &self.profiles[i] };
            con.print(11, i as i32 * 5 + 7, text.as_str(), doryen_rs::TextAlign::Center, Some(white.u8()), None);
            for j in 0..ACTIONS.len() {
                let selected = self.cursor_pos == (i, j) && !self.renaming;
                let color = if selected { red } else { white };
                let fore = if selected { Some(white.u8()) } else { None };
                let back = Some(if selected { white.u8() } else { darker_gray });    
                render_button(con, j as i32 * 18 + 22, i as i32 * 5 + 6, 18, 3, ACTIONS[j].text(), color, fore, back, Align::start2());
            }
        }

        let selected = self.profiles.len() == self.cursor_pos.0 && !self.renaming;
        let color = if selected { red } else { white };
        let fore = if selected { Some(white.u8()) } else { None };
        let back = Some(if selected { white.u8() } else { darker_gray });
        render_button(con, 5, self.profiles.len() as i32 * 5 + 7, CONSOLE_WIDTH - 13, 3, "new", color, fore, back, (Align::Start, Align::center()));
        
        render_rect(con, CONSOLE_WIDTH as i32, 5, 3, 3, Some(('-', darker_gray)), Some(dark_gray), (Align::End, Align::Start));
        con.ascii(CONSOLE_WIDTH as i32 - 2, 6, 30);
        con.fore(CONSOLE_WIDTH as i32 - 2, 6, black.u8());
        render_rect(con, CONSOLE_WIDTH as i32, CONSOLE_HEIGHT as i32, 3, 3, Some(('-', darker_gray)), Some(dark_gray), (Align::End, Align::End));
        con.ascii(CONSOLE_WIDTH as i32 - 2, CONSOLE_HEIGHT as i32 - 2,31);
        con.fore(CONSOLE_WIDTH as i32 - 2, CONSOLE_HEIGHT as i32 - 2,black.u8());

        // render title
        render_button(con, 0, 0, CONSOLE_WIDTH, 5, "Profiles", white, Some(darker_gray), None, (Align::Start, Align::Start));

        // renders the Esc button 
        render_button(con, 0, 0, 7, 5, "Esc", red, Some(darker_gray), None, (Align::Start, Align::Start));

    }
}

// input handling 
impl InputHandler for Profiles {

    // register the inputs without category distinction
    fn register_inputs (&mut self) {
        self.inputmap = vec![
            crate::KeyMap::new("Escape",        "", None ),
            crate::KeyMap::new("Enter",         "", None ),
            crate::KeyMap::new("ArrowUp",       "", Some(6) ),
            crate::KeyMap::new("ArrowDown",     "", Some(6) ),
            crate::KeyMap::new("ArrowLeft",     "", Some(6) ),
            crate::KeyMap::new("ArrowRight",    "", Some(6) ),
        ];
    }

    // handle per frame inputs
    fn handle_input(&mut self, input: &mut dyn doryen_rs::InputApi, _: &str) -> Option<GameEvent> {

        if self.renaming {
            if input.key_pressed("Enter") { return self.action() }
            if input.key_pressed("Escape") { return self.escape() }

            let txt = input.text();
            if !txt.is_empty() {
                self.rename_temp.push_str(&txt);
            }
            // handle backspace
            if input.key_pressed("Backspace") && !self.rename_temp.is_empty() {
                self.rename_temp.pop();
            }
            return None
        }

        // loop through all registered inputs
        for index in 0..self.inputmap.len() {

            // if trigger returns true, match the key to call the function
            if self.inputmap[index].trigger(input).to_owned() { match self.inputmap[index].key_text.as_str() {

                // before paused check game inputs
                "Escape"        => return self.escape(),
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