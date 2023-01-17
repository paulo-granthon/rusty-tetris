pub enum InputID {
    Pause,
    Left,
    Right,
    Down,
    Up,
    RotateL,
    RotateR,
    Skip,
}

impl InputID {

    // returns a &str correspondingg to this InputID
    pub fn as_str(&self) -> &str {
        match self {
            InputID::Pause => "Pause",
            InputID::Left => "Left",
            InputID::Right => "Right",
            InputID::Down => "Down",
            InputID::Up => "Up",
            InputID::RotateL => "RotateL",
            InputID::RotateR => "RotateR",
            InputID::Skip => "Skip",
        }
    }

    // returns a matching InputID for the given index
    pub fn from_index (index: usize) -> Self {
        match index {
            0 => Self::Pause,
            1 => Self::Left,
            2 => Self::Right,
            3 => Self::Down,
            4 => Self::Up,
            5 => Self::RotateL,
            6 => Self::RotateR,
            7 => Self::Skip,
            _ => panic!("InputID::from_index({}) -- index out of range", index)
        }
    }

}

#[derive(Debug)]
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

    pub fn from_vec (list: Vec<String>) -> Option<Self> {
        if list.len() != 8 { 
            println!("Controller::from_vec() -- Error: expected vec of length == 8 but got {} instead: {:?}", list.len(), list);
            return None
        }
        Some(Self {
            action:   list[0].to_owned(),
            left:     list[1].to_owned(),
            right:    list[2].to_owned(),
            down:     list[3].to_owned(),
            up:       list[4].to_owned(),
            rotate_l: list[5].to_owned(),
            rotate_r: list[6].to_owned(),
            skip:     list[7].to_owned(),
        })
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

    pub fn default_versus1 () -> Self { Self::default_versus(0) }
    pub fn default_versus2 () -> Self { Self::default_versus(1) }

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
            InputID::Pause => self.action.as_str(),
            InputID::Left => self.left.as_str(),
            InputID::Right => self.right.as_str(),
            InputID::Down => self.down.as_str(),
            InputID::Up => self.up.as_str(),
            InputID::RotateL => self.rotate_l.as_str(),
            InputID::RotateR => self.rotate_r.as_str(),
            InputID::Skip => self.skip.as_str(),
        }
    }

    
    // matches InputId with associated key
    pub fn get_at (&self, index: usize) -> &str {
        match index {
            0 => self.action.as_str(),
            1 => self.left.as_str(),
            2 => self.right.as_str(),
            3 => self.down.as_str(),
            4 => self.up.as_str(),
            5 => self.rotate_l.as_str(),
            6 => self.rotate_r.as_str(),
            7 => self.skip.as_str(),
            _ => panic!("Controller::get_at({}) -- index out of range", index)
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

    // sets the key of the InputID at the given index
    pub fn set_at (&mut self, index: usize, key: &str) {
        match index {
            0 => self.action = key.to_string(),
            1 => self.left = key.to_string(),
            2 => self.right = key.to_string(),
            3 => self.down = key.to_string(),
            4 => self.up = key.to_string(),
            5 => self.rotate_l = key.to_string(),
            6 => self.rotate_r = key.to_string(),
            7 => self.skip = key.to_string(),
            _ => panic!("Controller.set_at({}, {}) -- Trying to set key of InputID at invalid index", index, key) 

        }
    }

}