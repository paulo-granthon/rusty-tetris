use super::super::{write_binary, load_binary};

const PATH: &str = "data/scores/";


pub fn save_score (player: u8, score: i32, game_mode: u8) -> Result<(), std::io::Error> {

    let mut score_bytes = (score / 10).to_be_bytes();
    score_bytes[0] = (player << 4) + game_mode;

    // println!("{:?}", player_bytes);
    // println!("{:?}", game_mode_bytes);
    // println!("{:?}", score_bytes);

    write_binary::<4>(PATH, score_bytes)
}

pub fn load_scores (player: Option<u8>, game_mode: Option<u8>) -> Result<Vec<(u8, u8, i32)>, std::io::Error> {

    match load_binary(PATH) {
        Ok(buffer) => {
            let mut list = vec![]; 
            for i in 0..buffer.len() / 4 {
                list.push((
                    buffer[i * 4] >> 4,
                    (buffer[i * 4] << 4) >> 4,
                    i32::from_be_bytes([0, buffer[(i * 4) + 1], buffer[(i * 4) + 2], buffer[(i * 4) + 3]]) * 10
                ))
            }
            // match player { Some(p) => list = list.drain_filter(|s| *s.0 == player), _=>{} }
            // match game_mode { Some(gm) => list = list.drain_filter(|s| *s.1 == gm), _=>{} }
            Ok(list)
        },
        Err(e) => Err(e) 
    }


}