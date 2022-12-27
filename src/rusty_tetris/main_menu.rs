use crate::{CONSOLE_HEIGHT, CONSOLE_WIDTH};

use super::{GameState, InputHandler, render_button, RTColor, RustyTetris};

const ACTIONS: &'static [(&'static str, RTColor); 5] = &[
    ("Play",        RTColor::Cyan),
    ("Versus",      RTColor::Magenta),
    ("Scores",      RTColor::Green),
    ("Settings",    RTColor::Blue),
    ("Exit",        RTColor::Red),
];

pub struct MainMenu {
    pub cursor_pos: usize,
    pub inputmap: Vec::<super::KeyMap>,
}

impl MainMenu {
    pub fn new() -> Self { Self {cursor_pos: 0, inputmap: vec![] }}

    fn move_cursor(&mut self, dir: i32) -> Option<GameState> {
        self.cursor_pos = (self.cursor_pos as i32 + dir).max(0).min(ACTIONS.len() as i32 - 1) as usize;
        println!("{}", self.cursor_pos);
        None
    }

    fn action (&self) -> Option<GameState> {
        match ACTIONS[self.cursor_pos].0 {

            "Play"      => return Some(GameState::Game(Some(RustyTetris::new()))),
            "Versus"    => return None,
            "Scores"    => return None,
            "Settings"  => return None,
            "Exit"      => return None,

            _=> { println!("main_menu -- Error: unmapped action at cursor_pos '{}' ", self.cursor_pos); None }
        }
    }
}

impl super::RustyEngine for MainMenu {
    fn init(&mut self) {
        self.register_inputs()
    }

    fn update(&mut self, api: &mut dyn doryen_rs::DoryenApi) -> (Option<GameState>, Option<doryen_rs::UpdateEvent>) {
        // (Some(GameState::Game(Some(super::RustyTetris::new()))), None)

        let input = api.input();
        // self.mouse_pos = input.mouse_pos();
        (self.handle_input(input, ""), None)
    }

    fn render(&mut self, api: &mut dyn doryen_rs::DoryenApi) {

        let con = api.con();
        con.clear(Some(RTColor::Black.value().1), Some(RTColor::Black.value().1), Some(' ' as u16));

        let white_colr = RTColor::White.value().1;
        let fore_color = RTColor::Grey.value().1;
        let black_colr = RTColor::Black.value().1;

        let half_con_height = CONSOLE_HEIGHT as i32 / 2;
        let half_con_width  = CONSOLE_WIDTH as i32  / 2;

        let menu_height = ACTIONS.len() as i32 * 3;

        for i in 0..ACTIONS.len() {
            let text_color = ACTIONS[i].1.value().1;
            render_button(
                con,
                half_con_width,
                half_con_height - (menu_height / 2) + (i as i32 * super::gui::BUTTON_HEIGHT as i32),
                12,
                ACTIONS[i].0,
                if i == self.cursor_pos {black_colr}       else {text_color},
                if  i == self.cursor_pos {Some(white_colr)} else {Some(fore_color)},
                if  i == self.cursor_pos {Some(text_color)} else {Some(black_colr)},
            );
            
        }

    }
}

impl super::InputHandler for MainMenu {
    fn register_inputs (&mut self) {
        self.inputmap = vec![
            super::KeyMap::new("Enter",         "", None ),
            super::KeyMap::new("ArrowUp",       "", Some(12) ),
            super::KeyMap::new("ArrowDown",     "", Some(12) ),
        ];
    }

    fn handle_input(&mut self, input: &mut dyn doryen_rs::InputApi, _category: &str) -> Option<GameState> {

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

        None
    }
}