use crate::{RustyEngine, GameEvent, InputHandler, RTColor, render_logo, render_button };
use crate::{CONSOLE_HEIGHT, CONSOLE_WIDTH};

// for action distinction
enum Action {
    Play,
    Versus,
    Profile,
    Scores,
    Settings,
    Exit,
}

// logic implementation for each Action
impl Action {
    fn text (&self) -> &str {
        match self {
            Action::Play     => "Play",
            Action::Versus   => "Versus",
            Action::Profile  => "Profiles",
            Action::Scores   => "Scores",
            Action::Settings => "Settings",
            Action::Exit     => "Exit",        
        }
    }
    fn color (&self) -> RTColor {
        match self {
            Action::Play     => RTColor::Cyan,
            Action::Versus   => RTColor::Magenta,
            Action::Profile  => RTColor::Yellow,
            Action::Scores   => RTColor::Green,
            Action::Settings => RTColor::Blue,
            Action::Exit     => RTColor::Red,        
        }
    }
}

// lists the possible idenfiable actions of the main_menu
const ACTIONS: [Action; 6] = [
    Action::Play,
    Action::Versus,
    Action::Profile,
    Action::Scores,
    Action::Settings,
    Action::Exit,
];

// defines the state
pub struct MainMenu {
    pub cursor_pos: usize,
    pub inputmap: Vec::<crate::KeyMap>,
}

// logic implementation for the MainMenu
impl MainMenu {

    // initializatio
    pub fn new () -> Self { Self {cursor_pos: 0, inputmap: vec![] }}

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
        match ACTIONS[self.cursor_pos] {
            
            // returns Some GameEvent matching the action
            Action::Play      => return Some(GameEvent::new_game()),
            Action::Versus    => return Some(GameEvent::new_game_versus()),
            Action::Profile   => return Some(GameEvent::profiles()),
            Action::Scores    => return Some(GameEvent::scores()),
            Action::Settings  => return Some(GameEvent::settings()),
            Action::Exit      => return Some(GameEvent::Exit),

            // unmapped action
            // _=> { println!("main_menu.action() -- unmapped action at cursor_pos '{}' ", self.cursor_pos); None }
        }
    }

    pub fn render_playing_as (&self, con: &mut doryen_rs::Console, profile: usize) {
        if profile == 0 { return; }
        use crate::profile_tracker::profile_name;
        let half_con_height = CONSOLE_HEIGHT as i32 / 2;
        let half_con_width  = CONSOLE_WIDTH as i32  / 2;
        let menu_height = ACTIONS.len() as i32 * 3;
        con.print(half_con_width, half_con_height - (menu_height / 2) - 4, format!("Playing as {}", profile_name(profile)).as_str(), doryen_rs::TextAlign::Center, Some(RTColor::White.u8()), None);
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
        use crate::Align;

        // get the console
        let con = api.con();

        // calculate half sizes of console
        let half_con_height = CONSOLE_HEIGHT as i32 / 2;
        let half_con_width  = CONSOLE_WIDTH as i32  / 2;

        // render the RustyTetris logo
        render_logo(con, half_con_width, 1);

        // reference the following colors 
        let white_colr = RTColor::White.u8();
        let fore_color = RTColor::DarkerGrey.u8();
        let black_colr = RTColor::Black.u8();

        // calulate the visual height of the menu
        let menu_height = ACTIONS.len() as i32 * 3;

        // for each action
        for i in 0..ACTIONS.len() {

            // reference the color of the action to define the render_button colors
            let text_color = ACTIONS[i].color().u8();

            // render the button with the text
            render_button(
                con,
                half_con_width,
                half_con_height - (menu_height / 2) + (i as i32 * 5),
                12, 5,
                ACTIONS[i].text(),

                // active:      black text,     white bg details,       custom bg
                // inactive:    custom text,    grey bg details,        black bg
                if i == self.cursor_pos {RTColor::Black}   else {ACTIONS[i].color()},
                if  i == self.cursor_pos {Some(white_colr)} else {Some(fore_color)},
                if  i == self.cursor_pos {Some(text_color)} else {Some(black_colr)},

                Align::center2()
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
    fn handle_input(&mut self, input: &mut dyn doryen_rs::InputApi, _: &str) -> Option<GameEvent> {

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