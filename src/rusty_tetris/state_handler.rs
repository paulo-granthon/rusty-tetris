use doryen_rs::{Engine, DoryenApi, UpdateEvent};
use super::RustyEngine;

pub enum GameState {
    MainMenu(super::MainMenu),
    Game(Option<super::RustyTetris>),
    Scores,
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
            Self::Scores => {},
        }
    }
    fn update(&mut self, api: &mut dyn DoryenApi) -> (Option<GameState>, Option<UpdateEvent>) {
        match self {
            Self::MainMenu(mm) => mm.update(api),
            Self::Game(rt) => match rt { Some(game) => game.update(api), None => (None, None)},
            Self::Scores => (None, None),
        }
    }
    fn render(&mut self, api: &mut dyn DoryenApi) {
        match self {
            Self::MainMenu(mm) => mm.render(api),
            Self::Game(rt) => match rt { Some(game) => game.render(api), None => {}},
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
            Some(state) => self.set_state(state),
            _=>{}
        }
        state_update_result.1
    }

    fn render(&mut self, api: &mut dyn DoryenApi) {
        self.state.render(api)
    }
    
}