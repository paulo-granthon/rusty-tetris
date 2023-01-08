use doryen_rs::{Engine, DoryenApi, UpdateEvent};
use crate::states:: {MainMenu, Profiles, GameMode, Scores, Settings};
use crate::{RustyEngine, RTColor, profile_tracker, clear};

// wrapper for state
pub enum GameState {
    MainMenu(MainMenu),
    Profiles(Profiles),
    Game(GameMode),
    Scores(Scores),
    Settings(Settings),
}

// logic implementation for GameState
impl GameState {

    // initialization
    pub fn main_menu    () -> Self { GameState::MainMenu(MainMenu::new()) }
    pub fn profiles     () -> Self { GameState::Profiles(Profiles::new()) }
    pub fn singleplayer () -> Self { GameState::Game(GameMode::singleplayer()) }
    pub fn versus       () -> Self { GameState::Game(GameMode::versus()) }
    pub fn scores       () -> Self { GameState::Scores(Scores::new()) }
    pub fn settings     () -> Self { GameState::Settings(Settings::new()) }
}

// defines events to be returned by the GameStates to the StateHandler 
pub enum GameEvent {
    SetProfile(usize),
    State(GameState),
    PreviousState,
    GameOver,
    Exit,
}

// GameEvent::State initialization methods
impl GameEvent {
    pub fn main_menu        () -> Self { GameEvent::State(GameState::main_menu()) }
    pub fn profiles         () -> Self { GameEvent::State(GameState::profiles()) }
    pub fn new_game         () -> Self { GameEvent::State(GameState::singleplayer()) }
    pub fn new_game_versus  () -> Self { GameEvent::State(GameState::versus()) }
    pub fn scores           () -> Self { GameEvent::State(GameState::scores()) }
    pub fn settings         () -> Self { GameEvent::State(GameState::settings()) }
}

// redirect methods for GameState's state
impl GameState {
    fn init(&mut self) {
        match self {
            Self::MainMenu(state)  => state.init(),
            Self::Profiles(state)  => state.init(),
            Self::Game(state)      => state.init(),
            Self::Scores(state)      => state.init(),
            Self::Settings(state)  => state.init(),
        }
    }
    fn update(&mut self, api: &mut dyn DoryenApi) -> (Option<GameEvent>, Option<UpdateEvent>) {
        match self {
            Self::MainMenu(state)  => state.update(api),
            Self::Profiles(state)  => state.update(api),
            Self::Game(state)      => state.update(api),
            Self::Scores(state)      => state.update(api),
            Self::Settings(state)  => state.update(api),
        }
    }
    fn render(&mut self, api: &mut dyn DoryenApi) {
        clear(api.con());
        match self {
            Self::MainMenu(state)  => state.render(api),
            Self::Profiles(state)  => state.render(api),
            Self::Game(state)      => state.render(api),
            Self::Scores(state)      => state.render(api),
            Self::Settings(state)  => state.render(api),
        }
    }
}

// defines the StateHandler, responsible for storing the current state of the game and redirecting the engine's methods 
pub struct StateHandler {
    pub state: GameState,
    pub previous_state: Option<GameState>,
    pub profile: u8,
}

// logic implementation for StateHandler
impl StateHandler {

    // creates the StateHandler
    pub fn new () -> Self {
        Self {
            state: GameState::main_menu(), 
            previous_state: None, 
            profile: match profile_tracker::load_profile() { 
                Some(profile) => profile as u8, 
                None => 0 
            }
        }
    }

    // Sets the state of the StateHandler
    pub fn set_state(&mut self, state: GameState) {
        self.state = state;
        self.state.init();
    }

    fn previous_state(&mut self) {
        let state = match &self.previous_state {
            Some(state) => match state {
                GameState::Game(gamemode) => match gamemode {
                    GameMode::SinglePlayer(_) => GameState::singleplayer(), 
                    GameMode::Versus(_,_) => GameState::versus()
                },
                GameState::MainMenu(_) => GameState::main_menu(),
                GameState::Profiles(_) => GameState::profiles(),
                GameState::Scores(_) => GameState::scores(),
                GameState::Settings(_) => GameState::settings(),
            },
            None => GameState::main_menu()
        
        };
        self.set_state(state);
    } 

    // Sets the current profile 
    fn set_profile (&mut self, profile: usize) {
        self.profile = profile as u8;
        profile_tracker::set_profile(profile)
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

                // state requests the profile set
                GameEvent::SetProfile(profile) => self.set_profile(profile),
                
                // state returns a redirect to another state
                GameEvent::State(state) => self.set_state(state),

                // state requests to move to the previous state
                GameEvent::PreviousState => self.previous_state(),

                // state alerts that the player lost the game
                GameEvent::GameOver => match &self.state {
                    GameState::Game(game) => { game.game_over(self.profile); self.set_state(GameState::main_menu()) },
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