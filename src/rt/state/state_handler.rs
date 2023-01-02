use doryen_rs::{Engine, DoryenApi, UpdateEvent};
use super::super::{RustyEngine, RTColor, clear, RustyTetris, MainMenu, Scores, Settings};

pub enum GameMode {
    SinglePlayer,
    Versus,
}

pub enum GameState {
    MainMenu(MainMenu),
    Game(RustyTetris),
    // SetupVersus
    Versus(RustyTetris, RustyTetris),
    Scores(Scores),
    Settings(Settings),
}

pub enum GameEvent {
    State(GameState),
    Exit,
}

impl GameEvent {

    pub fn main_menu() -> Self {
        GameEvent::State(GameState::MainMenu(MainMenu::new()))
    }
    pub fn new_game() -> Self {
        GameEvent::State(GameState::Game(RustyTetris::singleplayer()))
    }
    pub fn new_game_versus() -> Self {
        GameEvent::State(GameState::Versus(RustyTetris::versus(1), RustyTetris::versus(2)))
    }
    pub fn scores() -> Self {
        GameEvent::State(GameState::Scores(Scores::new()))
    }
    pub fn settings() -> Self {
        todo!();
        // GameEvent::State(GameState::Versus(RustyTetris::versus(1), RustyTetris::versus(2)))
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
            Self::Game(game) => game.init(),
            Self::Versus(game0, game1) => {
                game0.init();
                game1.init();
            },
            Self::Scores(scores) => {},
            Self::Settings(settings) => {},
        }
    }
    fn update(&mut self, api: &mut dyn DoryenApi) -> (Option<GameEvent>, Option<UpdateEvent>) {
        match self {
            Self::MainMenu(mm) => mm.update(api),
            Self::Game(game) => game.update(api),
            Self::Versus(game0, game1) => {
                match ( game0.update(api).0, game1.update(api).0 ) {
                    (Some(r1), Some(_)) => (Some(r1), None),
                    _=> (None, None)
                }
            }
            Self::Scores(scores) => (None, None),
            Self::Settings(settings) => (None, None),
        }
    }
    fn render(&mut self, api: &mut dyn DoryenApi) {
        clear(api.con());
        match self {
            Self::MainMenu(mm) => mm.render(api),
            Self::Game(game) => game.render(api),
            Self::Versus(game0, game1) => { game0.render(api); game1.render(api); },
            Self::Scores(scores) => {},
            Self::Settings(settings) => {},
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
        for color in RTColor::iter() {
            api.con().register_color(color.value().0, color.value().1);
        }

        self.state.init()
        // self.set_state(GameState::Game(RustyTetris::new()))
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