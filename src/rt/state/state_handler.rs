use doryen_rs::{Engine, DoryenApi, UpdateEvent};
use crate::states:: {Game, MainMenu, Scores, Settings};
use super::super::{RustyEngine, RTColor, clear};

// wrapper for Game 
pub enum GameMode {
    SinglePlayer(Game),
    Versus(Game, Game),
}

// logic implementation for GameMode
impl GameMode {

    // creates a GameMode instance for singleplayer
    pub fn singleplayer() -> Self {
        GameMode::SinglePlayer(Game::singleplayer())
    }

    // creates a GameMode instance for versus mode with two Game instances
    pub fn versus() -> Self {
        GameMode::Versus(Game::versus(1), Game::versus(2))
    }
    
    // matches GameMode to a unique id for serialization
    pub fn id (&self) -> u8 {
        match self {
            GameMode::SinglePlayer(_) => 0,
            GameMode::Versus(_, _) => 1,
        }
    }

    // called when game ends to track scores
    pub fn game_over (&self) {

        // import score_tracker for this function only
        use crate::rt::serialization::score_tracker::*;

        // match GameMode
        match self {

            // singleplayer: track the score of the Game on this GameMode
            GameMode::SinglePlayer(game) => track_score(game.player as u8, self.id(), game.score),

            // versus: track the score of both instances of Game
            GameMode::Versus(game1, game2) => {
                track_score(game1.player as u8, self.id(), game1.score);
                track_score(game2.player as u8, self.id(), game2.score);
            }
        }
    }

    // redirects the init method to the Game of the GameMode
    pub fn init (&mut self) {
        match self {
            GameMode::SinglePlayer(game) => game.init(),
            GameMode::Versus(game1, game2) => { game1.init(); game2.init() },
        }
    }

    // redirects the update method to the Game of the GameMode
    pub fn update (&mut self, api: &mut dyn DoryenApi) -> (Option<GameEvent>, Option<UpdateEvent>) {
        match self {
            GameMode::SinglePlayer(game) => game.update(api),
            GameMode::Versus(game1, game2) => match ( game1.update(api).0, game2.update(api).0 ) {
                (Some(r1), Some(_)) => (Some(r1), None),
                _=> (None, None)
            }
        }
    }

    // redirects the render method to the Game of the GameMode
    pub fn render (&mut self, api: &mut dyn DoryenApi) {
        match self {
            GameMode::SinglePlayer(game) => game.render(api),
            GameMode::Versus(game1, game2) => { game1.render(api); game2.render(api) }
        }
    }
}

// wrapper for state
pub enum GameState {
    MainMenu(MainMenu),
    Game(GameMode),
    Scores(Scores),
    Settings(Settings),
}

// logic implementation for GameState
impl GameState {

    // initialization
    pub fn main_menu    () -> Self { GameState::MainMenu(MainMenu::new()) }
    pub fn singleplayer () -> Self { GameState::Game(GameMode::singleplayer()) }
    pub fn versus       () -> Self { GameState::Game(GameMode::versus()) }
    pub fn scores       () -> Self { GameState::Scores(Scores::new()) }
    pub fn settings     () -> Self { GameState::Settings(Settings::new()) }
}

// defines events to be returned by the GameStates to the StateHandler 
pub enum GameEvent {
    State(GameState),
    GameOver,
    Exit,
}

// GameEvent::State initialization methods
impl GameEvent {
    pub fn main_menu        () -> Self { GameEvent::State(GameState::main_menu()) }
    pub fn new_game         () -> Self { GameEvent::State(GameState::singleplayer()) }
    pub fn new_game_versus  () -> Self { GameEvent::State(GameState::versus()) }
    pub fn scores           () -> Self { GameEvent::State(GameState::scores()) }
    pub fn settings         () -> Self { GameEvent::State(GameState::settings()) }
}

// redirect methods for GameState's state
impl GameState {
    fn init(&mut self) {
        match self {
            Self::MainMenu(state) => state.init(),
            Self::Game(state)     => state.init(),
            Self::Scores(state)     => state.init(),
            Self::Settings(state) => state.init(),
        }
    }
    fn update(&mut self, api: &mut dyn DoryenApi) -> (Option<GameEvent>, Option<UpdateEvent>) {
        match self {
            Self::MainMenu(state) => state.update(api),
            Self::Game(state)     => state.update(api),
            Self::Scores(state)     => state.update(api),
            Self::Settings(state) => state.update(api),
        }
    }
    fn render(&mut self, api: &mut dyn DoryenApi) {
        clear(api.con());
        match self {
            Self::MainMenu(state) => state.render(api),
            Self::Game(state)     => state.render(api),
            Self::Scores(state)     => state.render(api),
            Self::Settings(state) => state.render(api),
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