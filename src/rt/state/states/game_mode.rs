use doryen_rs::{DoryenApi, UpdateEvent};
use crate::{Game, GameEvent, RustyEngine};

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
    pub fn game_over (&self, profile: u8) {

        // import score_tracker for this function only
        use crate::rt::serialization::score_tracker::*;

        // match GameMode
        match self {

            // singleplayer: track the score of the Game on this GameMode
            GameMode::SinglePlayer(game) => track_score(profile, self.id(), game.score),

            // versus: track the score of both instances of Game
            GameMode::Versus(game1, game2) => {
                track_score(profile, self.id(), game1.score);
                track_score(profile, self.id(), game2.score);
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

