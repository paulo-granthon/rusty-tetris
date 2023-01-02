use super::super::super::RustyEngine;


pub struct Scores {

}

impl Scores {
    pub fn new () -> Self {
        Self {}
    }
}

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