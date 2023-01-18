use crate::{InputHandler, GameEvent, RustyEngine};
use crate::{CONSOLE_HEIGHT, CONSOLE_WIDTH};
use crate::{Align, RTColor, render_rect, render_button};

// for action distinction
enum Action {
    Tab(i8),
    Scroll(i8),
    Exit,
    None,
}

impl Action {
    fn text(&self) -> String {
        match self {
            Action::Tab(x) => format!("Tab({})", x),
            Action::Scroll(y) => format!("Scroll({})", y),
            Action::Exit => "Exit".to_string(),
            Action::None => "None".to_string(),
        }
    }
}

// defines the state "Scores"
pub struct Scores {
    scores: Vec<(String, Vec<(u8, u8, i32)>, Vec<(u8, u8, i32)>)>,
    profiles: Vec<String>,
    cursor: usize,
    tab: usize,
    inputmap: Vec::<crate::KeyMap>,
    actions: [[Action; 3]; 3]
}

// implements initialization for the state
impl Scores {

    // initialize the state, loading the score data of all profiles
    pub fn new () -> Self {
        use crate::rt::serialization::score_tracker::*;
        use crate::rt::serialization::profile_tracker::get_profiles;

        // initialize player index and scores vec
        let mut scores = vec![(
            "All".to_owned(),
            load_history(None, None).unwrap(),
            load_best(None, None).unwrap()
        )];

        let profiles = get_profiles().expect("error loading profiles");

        for i in 0..profiles.len() {
            let player = Some(i as u8 + 1);

            match (load_history(player, None), load_best(player, None)) {

                (Ok(hist), Ok(best)) if hist.len() > 0 || best.len() > 0 => {
                    scores.push((profiles[i].to_string(), hist, best));
                },
                _=> {}
            }

        }


        Self {
            scores,
            profiles,
            cursor: 0,
            tab: 0,
            inputmap: vec![],
            actions: [
                [Action::None,    Action::Scroll(-1), Action::None   ],
                [Action::Tab(-1), Action::Exit,       Action::Tab( 1)],
                [Action::None,    Action::Scroll( 1), Action::None   ],
            ]
        }
    }

    // idenfifies the action triggered by the input and call the corresponding functionallity
    fn action (&mut self, x: usize, y: usize) -> Option<GameEvent> {
        println!("action: {}", self.actions[y][x].text());
        match self.actions[y][x] {
            Action::Tab(dir)    => self.tab(dir),
            Action::Scroll(dir) => self.scroll(dir as i32),
            Action::Exit => Some(GameEvent::main_menu()),
            Action::None => None,
        }
    }

    // switches between players
    fn tab (&mut self, dir: i8) -> Option<GameEvent> {
        let new = ((self.tab as i8 + dir) + self.scores.len() as i8) as usize % self.scores.len();
        if new != self.tab {
            self.tab = new;
            self.cursor = 0;
        }
        None
    }

    // scrolls the contents of the list 
    fn scroll (&mut self, dir: i32) -> Option<GameEvent> {
        let len = (self.scores[self.tab].0.len().max(self.scores[self.tab].1.len()) as i32).max(1);
        self.cursor = (self.cursor as i32 + dir).max(0).min(len - 14) as usize;
        None
    }


}

// implements the doryen-rs engine for the state
impl RustyEngine for Scores {

    // engine initialization
    fn init(&mut self) {
        println!("Scores.init() -- history lenth: {}", self.scores[0].0.len());
        self.register_inputs()
    }

    // engine update
    fn update(&mut self, api: &mut dyn doryen_rs::DoryenApi) -> (Option<GameEvent>, Option<doryen_rs::UpdateEvent>) {
        let input = api.input();
        (self.handle_input(input, ""), None)
    }

    fn render(&mut self, api: &mut dyn doryen_rs::DoryenApi) {
        
        // get the console
        let con = api.con();

        // calculate half sizes of console
        // let half_con_height = CONSOLE_HEIGHT as i32 / 2;
        // let half_con_width  = CONSOLE_WIDTH as i32  / 2;

        // reference the following colors 
        let white = RTColor::White;
        let blue = RTColor::Blue;
        let red = RTColor::Red;
        // let gray = RTColor::Grey.u8();
        let dark_gray = RTColor::DarkGray.u8();
        let darker_gray = RTColor::DarkerGray.u8();
        let black = RTColor::Black.u8();

        // render lists bg
        render_rect  (con, 0,  5, 40, CONSOLE_HEIGHT - 5, None, Some(darker_gray), (Align::Start, Align::Start));
        render_rect  (con, 40, 5, 37, CONSOLE_HEIGHT - 5, None, Some(darker_gray), (Align::Start, Align::Start));

        // render scrollbar
        render_rect(con, CONSOLE_WIDTH as i32, 8, 3, CONSOLE_HEIGHT - 11, Some(('|', darker_gray)), Some(black), (Align::End, Align::Start));
        if self.scores.len() > 0 {
            let max_list_len = self.scores[self.tab].0.len().max(self.scores[self.tab].1.len()) as i32;
            let scrollbar_height = (CONSOLE_HEIGHT as i32 - 11 - (max_list_len - 14)).max(1) as u32;
                render_rect(con, CONSOLE_WIDTH as i32, 8 + self.cursor as i32, 3, scrollbar_height, Some((' ', darker_gray)), Some(dark_gray), (Align::End, Align::Start));
        }
        
        // backgrounds
        render_rect(con, CONSOLE_WIDTH as i32, 5, 3, 3, Some(('-', darker_gray)), Some(dark_gray), (Align::End, Align::Start));
        con.ascii(CONSOLE_WIDTH as i32 - 2, 6, 30);
        con.fore(CONSOLE_WIDTH as i32 - 2, 6, black);
        render_rect(con, CONSOLE_WIDTH as i32, CONSOLE_HEIGHT as i32, 3, 3, Some(('-', darker_gray)), Some(dark_gray), (Align::End, Align::End));
        con.ascii(CONSOLE_WIDTH as i32 - 2, CONSOLE_HEIGHT as i32 - 2,31);
        con.fore(CONSOLE_WIDTH as i32 - 2, CONSOLE_HEIGHT as i32 - 2,black);
        // println!("{} | {}", self.cursor, self.scores[self.tab].1.len());

        // render best
        for i in 0..self.scores[self.tab].2.len() {
            
            // get the score record
            let record = self.scores[self.tab].2[i];

            // render 
            // render_button(con, 0, 10 + (i as i32 - self.cursor as i32) * 5, 40, 5, format!("{}º Player: #[red]{}#[white] | GM: #[blue]{}#[white] | Score: #[green]{}", i+1, record.0, record.1, record.2).as_str(), white, None, None, Align::start2());
            con.area(0, 10 + (i as i32 - self.cursor as i32) * 5, 40, 1, None, Some(darker_gray), Some(0));
            con.area(0, 14 + (i as i32 - self.cursor as i32) * 5, 40, 1, None, Some(darker_gray), Some(0));
            con.rectangle(0, 10 + (i as i32 - self.cursor as i32) * 5, 40, 5, Some(white.u8()), None, Some(0));
            con.print(4, 10 + (i as i32 - self.cursor as i32) * 5, format!("{}º", i+1).as_str(), doryen_rs::TextAlign::Right, Some(white.u8()), None);
            con.print(38, 10 + (i as i32 - self.cursor as i32) * 5, ["Singleplayer", "Versus"][record.1 as usize], doryen_rs::TextAlign::Right, Some([RTColor::Cyan, RTColor::Magenta][record.1 as usize].u8()), None);
            con.print(1, 10 + (i as i32 - self.cursor as i32) * 5 + 2, if record.0 == 0 { "Guest" } else { self.profiles[record.0 as usize - 1].as_str() }, doryen_rs::TextAlign::Left, Some(blue.u8()), None);
            con.print_color(37, 10 + (i as i32 - self.cursor as i32) * 5 + 2, format!("#[green]{}", record.2).as_str(), doryen_rs::TextAlign::Right, None);
        }

        // render history
        for i in 0..self.scores[self.tab].1.len() {
            
            // get the score record
            let record = self.scores[self.tab].1[i];

            // render 
            con.area(40, 10 + (i as i32 - self.cursor as i32) * 5, 37, 1, None, Some(darker_gray), Some(0));
            con.area(40, 14 + (i as i32 - self.cursor as i32) * 5, 37, 1, None, Some(darker_gray), Some(0));
            con.rectangle(40, 10 + (i as i32 - self.cursor as i32) * 5, 37, 5, Some(white.u8()), None, Some(0));
            con.print(75, 10 + (i as i32 - self.cursor as i32) * 5, ["Singleplayer", "Versus"][record.1 as usize], doryen_rs::TextAlign::Right, Some([RTColor::Cyan, RTColor::Magenta][record.1 as usize].u8()), None);
            con.print(41, 10 + (i as i32 - self.cursor as i32) * 5 + 2, if record.0 == 0 { "Guest" } else { self.profiles[record.0 as usize - 1].as_str() }, doryen_rs::TextAlign::Left, Some(blue.u8()), None);
            con.print_color(74, 10 + (i as i32 - self.cursor as i32) * 5 + 2, format!("#[green]{}", record.2).as_str(), doryen_rs::TextAlign::Right, None);
        }
        
        // render title
        render_button(con, 0, 0, CONSOLE_WIDTH, 5, format!("Scores: {}", self.scores[self.tab].0).as_str(), blue, Some(darker_gray), None, (Align::Start, Align::Start));

        // renders the Esc button 
        render_button(con, 0, 0, 7, 5, "Esc", red, Some(darker_gray), None, (Align::Start, Align::Start));

        // render labels
        render_button(con, 0,  5, 40, 5, "#[cyan]Best Scores", RTColor::Gray, Some(darker_gray), None, (Align::Start, Align::Start));
        render_button(con, 40, 5, 37, 5, "#[magenta]History", RTColor::Gray, Some(darker_gray), None, (Align::Start, Align::Start));

    }
}

impl InputHandler for Scores {

    fn register_inputs (&mut self) {
        self.inputmap = vec![
            crate::KeyMap::new("Escape",        "", None ),
            crate::KeyMap::new("ArrowUp",       "", Some(4) ),
            crate::KeyMap::new("ArrowDown",     "", Some(4) ),
            crate::KeyMap::new("ArrowLeft",     "", Some(6) ),
            crate::KeyMap::new("ArrowRight",    "", Some(6) ),
        ];
    }

    fn handle_input(&mut self, input: &mut dyn doryen_rs::InputApi, _: &str) -> Option<GameEvent> {
        
        // loop through all registered inputs
        for index in 0..self.inputmap.len() {

            // if trigger returns true, match the key to call the function
            if self.inputmap[index].trigger(input).to_owned() { match self.inputmap[index].key_text.as_str() {

                // before paused check game inputs
                "Escape"        => return self.action(1, 1),
                "ArrowUp"       => return self.action(1, 0),
                "ArrowDown"     => return self.action(1, 2),
                "ArrowLeft"     => return self.action(0, 1),
                "ArrowRight"    => return self.action(2, 1),

                // no key ? probably a overlook
                _=> { println!("{}.handle_input: Key '{}' is registered but not mapped!", std::any::type_name::<Self>(), self.inputmap[index].key_text); return None }
            }}
        }

        // no result
        None
    }
}