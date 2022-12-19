
use super::database::{COUNT, random_enum, TetrominoID};
use super::tetromino::Tetromino;
use super::RustyTetris;

pub struct Bag {
    sequence :[TetrominoID; COUNT],
    index: usize,
}

impl Bag {
    pub fn new () -> Self {
        let mut sequence: Vec<TetrominoID> = vec![];
        for _ in 0..COUNT {
            let mut tid = random_enum();
            while contains(&sequence, &tid) {
                tid = random_enum();
            }
            sequence.push(tid);
        }
        println!("New bag: {:?}", sequence);
        Self {
            sequence: sequence.try_into().unwrap_or_else(|_| panic!("src\\rusty_tetris\\rt_random.Bag::new -- Error")),
            index: 0 
        }
    }
    pub fn next(&mut self) -> Option<Tetromino> {
        if self.index >= self.sequence.len() { return None }
        let tetromino: Tetromino = self.sequence[self.index].get();
        self.index += 1;
        Some(tetromino)
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
}

impl HasBag for RustyTetris {
    fn bag_next(&mut self) -> Tetromino {
        loop {
            match &mut self.some_bag {
                Some(bag) => {
                    match &mut bag.next() {
                        Some(tetromino) => return tetromino.to_owned(),
                        None => self.some_bag = Some(Bag::new())
                    }
                }
                None => self.some_bag = Some(Bag::new())
            }
        }
    }
}