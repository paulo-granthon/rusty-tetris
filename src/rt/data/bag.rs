
use crate::database::{COUNT, random_enum, TetrominoID};
use crate::Tetromino;

pub struct Bag {
    sequence :[TetrominoID; COUNT],
    index: usize,
}

impl Bag {
    fn fill_sequence (sequence: &mut Vec<TetrominoID>) -> Vec<TetrominoID> {
        while sequence.len() < COUNT {
            let mut tid = random_enum();
            while contains(&sequence, &tid) {
                tid = random_enum();
            }
            sequence.push(tid);
        }
        sequence.to_owned()
    }
    fn sequence_to_array<const L: usize>(sequence: Vec<TetrominoID>) -> [TetrominoID; L] {
        sequence.try_into().unwrap_or_else(|_| panic!("src\\rusty_tetris\\rt_random.Bag::new -- Error"))
    }
    pub fn from (sequence: Vec<TetrominoID>) -> Self {
        Self {
            sequence: Self::sequence_to_array(Self::fill_sequence(&mut sequence.to_owned())),
            index: 0 
        }
    }
    pub fn new () -> Self {
        Self::from(vec![])
    }
    pub fn next(&mut self) -> Tetromino {
        let seq_len = self.sequence.len();
        if self.index >= seq_len - 1 {
            self.sequence = Self::sequence_to_array(Self::fill_sequence(&mut vec![self.sequence[seq_len - 1]]));
            self.index = 0;
        }
        let tetromino: Tetromino = self.sequence[self.index].get();
        self.index += 1;
        tetromino
    }
    pub fn peek (&self) -> Option<TetrominoID> {
        if self.index >= self.sequence.len() { return None }
        Some(self.sequence[self.index])
    }

}

fn contains(sequence: &Vec<TetrominoID>, tid: &TetrominoID) -> bool {
    for i in 0..sequence.len() {
        if &sequence[i] == tid { return true }
    }
    false
}
pub trait HasBag {
    fn bag_next(&mut self) -> Tetromino;
    fn bag_peek_next(&self) -> Option<TetrominoID>;
}
