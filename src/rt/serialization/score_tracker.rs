use crate::{write_binary, append_binary, load_binary};

// path where scores are saved
const PATH_HISTORY: &str = "data/scores/history";
const PATH_BEST: &str = "data/scores/best";

// maximum best scores list length
const MAX_BEST_LENGTH: usize = 100;

// formats the given data to binary
fn to_bytes (player_id: u8, game_mode: u8, score: i32) -> [u8; 4] {
    assert!(player_id <= 16, "score_tracker.to_bytes() -- Error: Expected player of id <= 16 but got {} instead", player_id);
    debug_assert!(game_mode <= 16, "score_tracker.to_bytes() -- Error: Expected game_mode of id <= 16 but got {} instead", game_mode);

    // get the score as be_bytes
    let mut bytes = (score / 10).to_be_bytes();

    // replace the first u8 element with 4 bytes for the player and 4 bytes for the gamemode
    bytes[0] = (player_id << 4) + game_mode;

    // return the result
    bytes
}

// stores the given score on history and recalculates the best scores considering the new entry
pub fn track_score (profile: u8, game_mode: u8, score: i32) {

    // save score on history
    match save_score(profile, game_mode, score) {
        Err(e) => println!("track_score: save_score({}, {}, {}) -- Erro: {}", profile, game_mode, score, e),
        _=> {}
    }

    // update best scores
    match update_best(profile, game_mode, score) {
        Err(e) => println!("track_score: update_best({}, {}, {}) -- Erro: {}", profile, game_mode, score, e),
        _=> {}
    }
}


// updates the best scores of given player
pub fn update_best (player: u8, game_mode: u8, score: i32) -> Result<(), std::io::Error>{

    if score == 0 { return Ok(()) }

    // loads best scores and match result
    let scores = match load_scores(PATH_BEST) {
        
        // best scores loaded successfully
        Ok(scores) => scores,
        
        // best scores file not initialized
        _=> vec![]
    };

    // convert the new entry to bytes
    let new_score_bytes = to_bytes(player, game_mode, score);

    // initialize a list for the new best scores
    let mut new_scores = vec![];

    // initialize a bool to define if the new score was added
    let mut added: bool = false;

    // loop through the scores
    for i in 0..scores.len().min(MAX_BEST_LENGTH) {

        new_scores.push(to_bytes(scores[i].0, scores[i].1, scores[i].2));

        // if the score at the index is greater than the given score, skip
        if scores[i].2 > score || added { continue; }

        // new score is greater or equal to current score, insert the new score at this index
        new_scores.insert(i, new_score_bytes);

        added = true;
    }

    if !added && scores.len() < MAX_BEST_LENGTH { new_scores.push(new_score_bytes) }

    // write nothing to the file to overwrite
    let _ = write_binary(PATH_BEST, new_scores[0]);

    // append each score 
    for i in 1..new_scores.len() {
        let _ = append_binary(PATH_BEST, new_scores[i]);
    }

    // update successfull
    Ok(())


}

// append the score to the history
pub fn save_score (player: u8, game_mode: u8, score: i32) -> Result<(), std::io::Error> {

    // append the score to the binary at path 
    append_binary(PATH_HISTORY, to_bytes(player, game_mode, score))
}

// loads all scores on given file
fn load_scores (path: &str) -> Result<Vec<(u8, u8, i32)>, std::io::Error> {

    // loads the binary and match result
    match load_binary(path) {

        // file is loaded successfully
        Ok(buffer) => {

            // create a vec of tuple for player, gamemode and score
            let mut list = vec![]; 

            // loop through the loaded buffer with increments of 4 u8
            for i in 0..buffer.len() / 4 {

                // add the following tuple to the list
                list.push((

                    // player: first 4 bytes of first u8
                    buffer[i * 4] >> 4,

                    // game_mode: last 4 bytes of first u8
                    (buffer[i * 4] << 4) >> 4,

                    // score: the 3 remaining u8
                    i32::from_be_bytes([0, buffer[(i * 4) + 1], buffer[(i * 4) + 2], buffer[(i * 4) + 3]]) * 10
                ))
            }

            // return the result
            Ok(list)
        },

        // error loading binary file
        Err(e) => Err(e)
    }
}

// loads and filters a list of scores at given path
fn load_filter (path: &str, player: Option<u8>, game_mode: Option<u8>) -> Result<Vec<(u8, u8, i32)>, std::io::Error> {
    match load_scores(path) {
        Ok(mut scores) => {
            
            // match filter params and filters list if given
            match player { Some(p) => { scores = scores.drain(..).filter(|s| s.0 == p).collect(); }, _=>{} }
            match game_mode { Some(gm) => { scores = scores.drain(..).filter(|s| s.1 == gm).collect(); }, _=>{} }
            Ok(scores)

        },
        Err(e) => Err(e)
    }
}

// loads history and filters the list if player and/or game_mode params are given
pub fn load_history (player: Option<u8>, game_mode: Option<u8>) -> Result<Vec<(u8, u8, i32)>, std::io::Error> {
    match load_filter(PATH_HISTORY, player, game_mode) {
        Ok(scores) => Ok(scores),
        _=> Ok(vec![])
    }
}

// loads best and filters the list if player and/or game_mode params are given
pub fn load_best (player: Option<u8>, game_mode: Option<u8>) -> Result<Vec<(u8, u8, i32)>, std::io::Error> {
    match load_filter(PATH_BEST, player, game_mode) {
        Ok(scores) => Ok(scores),
        _ => Ok(vec![])
    }
}
