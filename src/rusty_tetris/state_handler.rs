use doryen_rs::{Engine, DoryenApi, UpdateEvent};
use super::RustyEngine;

pub enum GameState {
    MainMenu(super::MainMenu),
    Game(Option<super::RustyTetris>),
    // SetupVersus
    Versus(Option<super::RustyTetris>, Option<super::RustyTetris>),
    Scores,
}

pub enum GameEvent {
    State(GameState),
    Exit,
}

impl GameEvent {

    pub fn main_menu() -> Self {
        GameEvent::State(GameState::MainMenu(super::MainMenu::new()))
    }
    pub fn new_game() -> Self {
        GameEvent::State(GameState::Game(Some(super::RustyTetris::singleplayer())))
    }
    pub fn new_game_versus() -> Self {
        GameEvent::State(GameState::Versus(Some(super::RustyTetris::versus(1)), Some(super::RustyTetris::versus(2))))
    }

}

impl GameState {
    // pub fn iter() -> std::slice::Iter<'static, GameState> {
    //     static STATES: [GameState; 3] = [GameState::MainMenu, GameState::Game(None), GameState::Scores];
    //     STATES.iter()
    // }

    fn init(&mut self) {
        match self {
            Self::MainMenu(mm) => mm.init(),
            Self::Game(rt) => match rt { Some(game) => game.init(), None => {}},
            Self::Versus(rt0, rt1) => {
                match rt0 { Some(game) => game.init(), None => {}};
                match rt1 { Some(game) => game.init(), None => {}};
            }
            Self::Scores => {},
        }
    }
    fn update(&mut self, api: &mut dyn DoryenApi) -> (Option<GameEvent>, Option<UpdateEvent>) {
        match self {
            Self::MainMenu(mm) => mm.update(api),
            Self::Game(rt) => match rt { Some(game) => game.update(api), None => (None, None)},
            Self::Versus(rt0, rt1) => {
                match (
                    match rt0 { Some(game0) => game0.update(api).0, None => None},
                    match rt1 { Some(game1) => game1.update(api).0, None => None}
                ) {
                    (Some(r1), Some(_)) => (Some(r1), None),
                    _=> (None, None)
                }
            }
            Self::Scores => (None, None),
        }
    }
    fn render(&mut self, api: &mut dyn DoryenApi) {
        api.con().clear(Some(super::RTColor::Black.value().1), Some(super::RTColor::Black.value().1), Some(' ' as u16));
        match self {
            Self::MainMenu(mm) => mm.render(api),
            Self::Game(rt) => match rt { Some(game) => game.render(api), None => {}},
            Self::Versus(rt0, rt1) => {
                match rt0 { Some(game0) => {game0.render(api);}, None => {}};
                match rt1 { Some(game1) => {game1.render(api);}, None => {}};
            }
            Self::Scores => {},
        }
    }
}

pub struct StateHandler {
    pub state: GameState,
}

impl StateHandler {

    // creates the StateHandler
    pub fn new () -> Self {
        Self { state: GameState::MainMenu(super::main_menu::MainMenu::new()) }
    }

    // Sets the state of the 
    pub fn set_state(&mut self, state: GameState) {
        self.state = state;
        self.state.init();
    }
}

impl Engine for StateHandler {

    fn init(&mut self, api: &mut dyn DoryenApi) {

        // register colors 
        for color in super::RTColor::iter() {
            api.con().register_color(color.value().0, color.value().1);
        }

        self.state.init()
        // self.set_state(GameState::Game(Some(super::RustyTetris::new())))
    }

    fn update(&mut self, api: &mut dyn DoryenApi) -> Option<UpdateEvent> {
        // self.state.update(api)

        let state_update_result = self.state.update(api);
        match state_update_result.0 { 
            Some(event) => match event {
                GameEvent::State(state) => self.set_state(state),
                GameEvent::Exit => return Some(UpdateEvent::Exit),
                // _=>{}
            },
            _=>{}
        }
        state_update_result.1
    }

    fn render(&mut self, api: &mut dyn DoryenApi) {
        self.state.render(api)
    }
    
}