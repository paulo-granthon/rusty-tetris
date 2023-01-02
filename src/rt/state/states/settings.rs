use super::super::super::RustyEngine;


pub struct Settings {

}

impl Settings {
    pub fn new () -> Self {
        Self {}
    }
}

impl RustyEngine for Settings {
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