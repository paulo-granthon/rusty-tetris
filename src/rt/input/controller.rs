pub enum InputID {
    Action,
    Left,
    Right,
    Down,
    Up,
    RotateL,
    RotateR,
    Skip,
}

impl InputID {
    pub fn as_str(&self) -> &str {
        match self {
            InputID::Action => "Action",
            InputID::Left => "Left",
            InputID::Right => "Right",
            InputID::Down => "Down",
            InputID::Up => "Up",
            InputID::RotateL => "RotateL",
            InputID::RotateR => "RotateR",
            InputID::Skip => "Skip",
        }
    }
    pub fn from_index (index: u8) -> Option<Self> {
        match index {
            0 => Some(Self::Action),
            1 => Some(Self::Left),
            2 => Some(Self::Right),
            3 => Some(Self::Down),
            4 => Some(Self::Up),
            5 => Some(Self::RotateL),
            6 => Some(Self::RotateR),
            7 => Some(Self::Skip),
            _=> None
        }
    }
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

    pub fn from_vec (list: Vec<String>) -> Self {
        Self {
            action:   list[0].to_owned(),
            left:     list[1].to_owned(),
            right:    list[2].to_owned(),
            down:     list[3].to_owned(),
            up:       list[4].to_owned(),
            rotate_l: list[5].to_owned(),
            rotate_r: list[6].to_owned(),
            skip:     list[7].to_owned(),
        }
    }

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

    // matches InputId with associated key
    pub fn get (&self, input: InputID) -> &str {
        match input {
            InputID::Action => self.action.as_str(),
            InputID::Left => self.left.as_str(),
            InputID::Right => self.right.as_str(),
            InputID::Down => self.down.as_str(),
            InputID::Up => self.up.as_str(),
            InputID::RotateL => self.rotate_l.as_str(),
            InputID::RotateR => self.rotate_r.as_str(),
            InputID::Skip => self.skip.as_str(),
        }
    }

    // returns an array with all String keys of the Controller
    pub fn get_all (&self) -> [&str; 8] {[
        self.action.as_str(),
        self.left.as_str(),
        self.right.as_str(),
        self.down.as_str(),
        self.up.as_str(),
        self.rotate_l.as_str(),
        self.rotate_r.as_str(),
        self.skip.as_str(),
    ]}

}