use super::super::super::RustyEngine;

// defines the State "Scores"
pub struct Scores {
    history: Vec<(u8, u8, i32)>,
    best: Vec<(u8, u8, i32)>,
    cursor_pos: usize,
    inputmap: Vec::<crate::KeyMap>,
}

// implements initialization for the state
impl Scores {
    pub fn new () -> Self {
        use crate::rt::serialization::score_tracker::*;
        Self {
            history: load_history(None, None).unwrap(), 
            best: load_best(None, None).unwrap(),
            cursor_pos: 0, inputmap: vec![],
        }
    }
}

// implements the doryen-rs engine for the state
impl RustyEngine for Scores {
    fn init(&mut self) {
        todo!()
    }

    fn update(&mut self, api: &mut dyn doryen_rs::DoryenApi) -> (Option<super::super::GameEvent>, Option<doryen_rs::UpdateEvent>) {
        todo!()
    }

    fn render(&mut self, api: &mut dyn doryen_rs::DoryenApi) {
        todo!()
    }
}