use crate::{CONSOLE_HEIGHT, CONSOLE_WIDTH};

use super::super::super::{InputHandler, render_button, RTColor, GameEvent, render_logo, RustyEngine};

// lists the possible idenfiable actions of the main_menu
const ACTIONS: &'static [(&'static str, RTColor); 5] = &[
    ("Play",        RTColor::Cyan),
    ("Versus",      RTColor::Magenta),
    ("Scores",      RTColor::Green),
    ("Settings",    RTColor::Blue),
    ("Exit",        RTColor::Red),
];

// defines the state
pub struct MainMenu {
    pub cursor_pos: usize,
    pub inputmap: Vec::<crate::KeyMap>,
}

// logic implementation for the MainMenu
impl MainMenu {

    // initializatio
    pub fn new() -> Self { Self {cursor_pos: 0, inputmap: vec![] }}

    // sets the position of the cursor 
    fn set_cursor(&mut self, pos: i32) -> Option<GameEvent> {
        let len = ACTIONS.len() as i32;
        self.cursor_pos = (((pos % len) + len) % len) as usize;
        None
    }

    // moves the cursor towards target direction
    fn move_cursor(&mut self, dir: i32) -> Option<GameEvent> {
        self.set_cursor(self.cursor_pos as i32 + dir)
    }

    // triggers the currently selected action
    fn action (&self) -> Option<GameEvent> {
        match ACTIONS[self.cursor_pos].0 {
            
            // returns Some GameEvent matching the action
            "Play"      => return Some(GameEvent::new_game()),
            "Versus"    => return Some(GameEvent::new_game_versus()),
            "Scores"    => return Some(GameEvent::scores()),
            "Settings"  => return Some(GameEvent::settings()),
            "Exit"      => return Some(GameEvent::Exit),

            // unmapped action
            _=> { println!("main_menu.action() -- unmapped action at cursor_pos '{}' ", self.cursor_pos); None }
        }
    }
}

// handle doryen-rs calls
impl RustyEngine for MainMenu {

    // engine initialization
    fn init(&mut self) {
        self.register_inputs()
    }

    // engine update
    fn update(&mut self, api: &mut dyn doryen_rs::DoryenApi) -> (Option<GameEvent>, Option<doryen_rs::UpdateEvent>) {
        let input = api.input();
        (self.handle_input(input, ""), None)
    }

    // rendering
    fn render(&mut self, api: &mut dyn doryen_rs::DoryenApi) {

        // get the console
        let con = api.con();

        // calculate half sizes of console
        let half_con_height = CONSOLE_HEIGHT as i32 / 2;
        let half_con_width  = CONSOLE_WIDTH as i32  / 2;

        // render the RustyTetris logo
        render_logo(con, half_con_width, 1);

        // reference the following colors 
        let white_colr = RTColor::White.value().1;
        let fore_color = RTColor::DarkerGrey.value().1;
        let black_colr = RTColor::Black.value().1;

        // calulate the visual height of the menu
        let menu_height = ACTIONS.len() as i32 * 3;

        // for each action
        for i in 0..ACTIONS.len() {

            // reference the color of the action to define the render_button colors
            let text_color = ACTIONS[i].1.value().1;

            // render the button with the text
            render_button(
                con,
                half_con_width,
                half_con_height - (menu_height / 2) + (i as i32 * crate::render::gui::BUTTON_HEIGHT as i32),
                12,
                ACTIONS[i].0,

                // active:      black text,     white bg details,       custom bg
                // inactive:    custom text,    grey bg details,        black bg
                if i == self.cursor_pos {black_colr}       else {text_color},
                if  i == self.cursor_pos {Some(white_colr)} else {Some(fore_color)},
                if  i == self.cursor_pos {Some(text_color)} else {Some(black_colr)},
            );
        }

    }
}

// input handling 
impl InputHandler for MainMenu {

    // register the inputs without category distinction
    fn register_inputs (&mut self) {
        self.inputmap = vec![
            crate::KeyMap::new("Enter",         "", None ),
            crate::KeyMap::new("ArrowUp",       "", Some(4) ),
            crate::KeyMap::new("ArrowDown",     "", Some(4) ),
        ];
    }

    // handle per frame inputs
    fn handle_input(&mut self, input: &mut dyn doryen_rs::InputApi, _category: &str) -> Option<GameEvent> {

        // loop through all registered inputs
        for index in 0..self.inputmap.len() {

            // if trigger returns true, match the key to call the function
            if self.inputmap[index].trigger(input).to_owned() { match self.inputmap[index].key_text.as_str() {

                // before paused check game inputs
                "Enter"         => return self.action(),
                "ArrowUp"       => return self.move_cursor(-1),
                "ArrowDown"     => return self.move_cursor(1),

                // no key ? probably a overlook
                _=> { println!("{}.handle_input: Key '{}' is registered but not mapped!", std::any::type_name::<Self>(), self.inputmap[index].key_text); return None }
            }}
        }

        // no result
        None
    }
}