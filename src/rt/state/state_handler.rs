use doryen_rs::{Engine, DoryenApi, UpdateEvent};
use super::super::{RustyEngine, RTColor, clear, RustyTetris, MainMenu, Scores, Settings};

pub enum GameMode {
    SinglePlayer(RustyTetris),
    Versus(RustyTetris, RustyTetris),
}

impl GameMode {

    pub fn singleplayer() -> Self {
        GameMode::SinglePlayer(RustyTetris::singleplayer())
    }

    pub fn versus() -> Self {
        GameMode::Versus(RustyTetris::versus(1), RustyTetris::versus(2))
    }
    
    pub fn id (&self) -> u8 {
        match self {
            GameMode::SinglePlayer(_) => 0,
            GameMode::Versus(_, _) => 1,
        }
    }

    pub fn game_over (&self) {
        use crate::rt::serialization::score_tracker::*;
        match self {
            GameMode::SinglePlayer(game) => track_score(game.player as u8, self.id(), game.score),
            GameMode::Versus(game1, game2) => {
                track_score(game1.player as u8, self.id(), game1.score);
                track_score(game2.player as u8, self.id(), game2.score);
            }
        }
    }

    pub fn init (&mut self) {
        match self {
            GameMode::SinglePlayer(game) => game.init(),
            GameMode::Versus(game1, game2) => { game1.init(); game2.init() },
        }
    }
    
    pub fn update (&mut self, api: &mut dyn DoryenApi) -> (Option<GameEvent>, Option<UpdateEvent>) {
        match self {
            GameMode::SinglePlayer(game) => game.update(api),
            GameMode::Versus(game1, game2) => match ( game1.update(api).0, game2.update(api).0 ) {
                (Some(r1), Some(_)) => (Some(r1), None),
                _=> (None, None)
            }
        }
    }

    pub fn render (&mut self, api: &mut dyn DoryenApi) {
        match self {
            GameMode::SinglePlayer(game) => game.render(api),
            GameMode::Versus(game1, game2) => { game1.render(api); game2.render(api) }
        }
    }
}

pub enum GameState {
    MainMenu(MainMenu),
    Game(GameMode),
    Scores(Scores),
    Settings(Settings),
}

impl GameState {

    pub fn main_menu () -> Self { 
        GameState::MainMenu(MainMenu::new())
    }
    pub fn singleplayer () -> Self { 
        GameState::Game(GameMode::singleplayer())
    }
    pub fn versus () -> Self { 
        GameState::Game(GameMode::versus())
    }
    pub fn scores () -> Self { 
        GameState::Scores(Scores::new())
    }
    // pub fn settings () -> Self { 
    //     GameState::Settings(Settings::new())
    // }
}

pub enum GameEvent {
    State(GameState),
    GameOver,
    Exit,
}

impl GameEvent {

    pub fn main_menu() -> Self {
        GameEvent::State(GameState::main_menu())
    }
    pub fn new_game() -> Self {
        GameEvent::State(GameState::singleplayer())
    }
    pub fn new_game_versus() -> Self {
        GameEvent::State(GameState::versus())
    }
    pub fn scores() -> Self {
        GameEvent::State(GameState::scores())
    }
    pub fn settings() -> Self {
        todo!();
        // GameEvent::State(GameState::settings())
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
            Self::Game(gm) => gm.init(),
            Self::Scores(scores) => {},
            Self::Settings(settings) => {},
        }
    }
    fn update(&mut self, api: &mut dyn DoryenApi) -> (Option<GameEvent>, Option<UpdateEvent>) {
        match self {
            Self::MainMenu(mm) => mm.update(api),
            Self::Game(gm) => gm.update(api),
            Self::Scores(scores) => (None, None),
            Self::Settings(settings) => (None, None),
        }
    }
    fn render(&mut self, api: &mut dyn DoryenApi) {
        clear(api.con());
        match self {
            Self::MainMenu(mm) => mm.render(api),
            Self::Game(gm) => gm.render(api),
            Self::Scores(scores) => {},
            Self::Settings(settings) => {},
        }
    }
}

// defines the StateHandler, responsible for storing the current state of the game and redirecting the engine's methods 
pub struct StateHandler {
    pub state: GameState,
}

// logic implementation for StateHandler
impl StateHandler {

    // creates the StateHandler
    pub fn new () -> Self {
        Self { state: GameState::MainMenu(super::main_menu::MainMenu::new()) }
    }

    // Sets the state of the StateHandler
    pub fn set_state(&mut self, state: GameState) {
        self.state = state;
        self.state.init();
    }
}

// doryen-rs engine implementation for StateHandler. Redirects the engine methods to the state
impl Engine for StateHandler {

    // initializes the initial state of the StateHandler
    fn init(&mut self, api: &mut dyn DoryenApi) {

        // register colors 
        for color in RTColor::iter() {
            api.con().register_color(color.value().0, color.value().1);
        } 
        self.state.init()
    }

    // updates the current state
    fn update(&mut self, api: &mut dyn DoryenApi) -> Option<UpdateEvent> {

        // update the state and store the result
        let state_update_result = self.state.update(api);
        
        // match the first value of the result
        match state_update_result.0 { 

            // some GameEvent is returned from state, match the actionn
            Some(event) => match event {
                
                // state returns a redirect to another state
                GameEvent::State(state) => self.set_state(state),

                // state alerts that the player lost the game
                GameEvent::GameOver => match &self.state {
                    GameState::Game(game) => { game.game_over(); self.set_state(GameState::main_menu()) },
                    _=> {} 
                }

                // state returns an UpdateEvent::Exit to quit the application 
                GameEvent::Exit => return Some(UpdateEvent::Exit),

                // _=>{}
            },
            
            // no event is returned from state update
            _=> {}
        }

        // return the second value of the state update
        state_update_result.1
    }

    // renders the current state
    fn render(&mut self, api: &mut dyn DoryenApi) {
        self.state.render(api)
    }
    
}