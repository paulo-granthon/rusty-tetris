enum InputID {
    Action,
    Left,
    Right,
    Down,
    Up,
    RotateL,
    RotateR,
    Skip,
}

pub struct Controller {
    pub action: String,
    pub left: String,
    pub right: String,
    pub down: String,
    pub up: String,
    pub rotate_l: String,
    pub rotate_r: String,
    pub skip: String,
}

impl Controller {

    // create a new controller
    pub fn new (
        action: String,
        left: String,
        right: String,
        down: String,
        up: String,
        rotate_l: String,
        rotate_r: String,
        skip: String
    ) -> Self { Self { action, left, right, down, up, rotate_l, rotate_r, skip, }}

    // default controller
    pub fn default () -> Self {
        Self::new(
            "Enter".to_string(),
            "ArrowLeft".to_string(),
            "ArrowRight".to_string(),
            "ArrowDown".to_string(),
            "ArrowUp".to_string(),
            "KeyW".to_string(),
            "KeyE".to_string(),
            "KeyQ".to_string(),
        )
    }

    // default multiplayer controller for versus mode
    pub fn default_versus (player: usize) -> Self {
        if player != 0 && player != 1 { panic!("invalid player index for default_versus controller") }
        Self::new(
            "Enter".to_string(),
            ["ArrowLeft", "KeyJ"][player].to_string(),
            ["ArrowRight", "KeyL"][player].to_string(),
            ["ArrowDown", "KeyK"][player].to_string(),
            ["ArrowUp", "KeyI"][player].to_string(),
            ["KeyN", "KeyW"][player].to_string(),
            ["KeyV", "KeyE"][player].to_string(),
            ["KeyB", "KeyQ"][player].to_string(),
        )
    }

}