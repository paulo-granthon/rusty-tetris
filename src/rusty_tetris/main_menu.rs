
pub struct MainMenu {
    pub cursor_pos: usize,
    pub inputmap: Vec::<super::KeyMap>,
}

impl MainMenu {
    pub fn new() -> Self { Self {cursor_pos: 0, inputmap: vec![] }}

    fn move_cursor(&mut self, dir: i32) {
        self.cursor_pos = (self.cursor_pos as i32 + dir).max(0).min(3) as usize
    }

    fn action (&self) {
        match self.cursor_pos {
            0 => println!("play"),
            // 0 => println!("versus"),
            1 => println!("scores"),
            2 => println!("settings"),
            3 => println!("exit"),
            _ => {} 
        }
    }
}

impl super::RustyEngine for MainMenu {
    fn init(&mut self) {}

    fn update(&mut self, api: &mut dyn doryen_rs::DoryenApi) -> Option<doryen_rs::UpdateEvent> {
        todo!()
    }

    fn render(&mut self, api: &mut dyn doryen_rs::DoryenApi) {
        todo!()
    }
}

impl super::InputHandler for MainMenu {
    fn register_inputs (&mut self) {
        self.inputmap = vec![
            super::KeyMap::new("Enter",         "", None ),
            super::KeyMap::new("ArrowUp",       "", Some(12) ),
            super::KeyMap::new("ArrowDown",     "", Some(12) ),
        ];
    }

    fn handle_input(&mut self, input: &mut dyn doryen_rs::InputApi, _category: &str) {

        // loop through all registered inputs
        for index in 0..self.inputmap.len() {

            // if trigger returns true, match the key to call the function
            if self.inputmap[index].trigger(input).to_owned() { match self.inputmap[index].key_text.as_str() {

                // before paused check game inputs
                "Enter"         => self.action(),
                "ArrowUp"       => self.move_cursor(-1),
                "ArrowDown"     => self.move_cursor(1),

                // no key ? probably a overlook
                _=> println!("{}.handle_input: Key '{}' is registered but not mapped!", std::any::type_name::<Self>(), self.inputmap[index].key_text)
            }}
        }
    }
}