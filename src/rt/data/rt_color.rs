const COUNT: usize = 9;

#[derive(Copy, Clone)]
#[allow(dead_code)]
pub enum RTColor {
    Black,
    White,
    Red,
    Green,
    Blue,
    Yellow,
    Magenta,
    Cyan,
    Orange,
    Grey,
    DarkGrey,
    DarkerGrey,
}

use std::slice::Iter;
use RTColor::*;
impl RTColor {
    pub fn iter() -> Iter<'static, RTColor> {
        static RTCOLORS: [RTColor; COUNT] = [Black, White, Red, Green, Blue, Yellow, Magenta, Cyan, Orange];
        RTCOLORS.iter()
    }
    pub fn value(&self) -> (&str, (u8, u8, u8, u8)) {
        match *self {
            Black       => ("black",        (0,     0,      0,      255)),
            White       => ("white",        (255,   255,    255,    255)),
            Red         => ("red",          (255,   92,     92,     255)),
            Green       => ("green",        (92,    255,    92,     255)),
            Blue        => ("blue",         (92,    92,     255,    255)),
            Yellow      => ("yellow",       (255,   255,    92,     255)),
            Magenta     => ("magenta",      (255,   92,     255,    255)),
            Cyan        => ("cyan",         (92,    255,    255,    255)),
            Orange      => ("orange",       (255,   92,     0,      255)),
            Grey        => ("grey",         (127,   127,    127,    255)),
            DarkGrey    => ("dark_grey",    (92,    92,     92,     255)),
            DarkerGrey  => ("darker_grey",  (46,    46,     46,     255)),
        }
    }
}

pub trait Alpha {
    fn no_alpha(&self) -> Self;
    fn alpha(&self, a: u8) -> Self;
    fn dim (&self, d: u8) -> Self;
}

impl Alpha for (u8, u8, u8, u8) {
    fn no_alpha(&self) -> Self {
        (self.0, self.1, self.2, 0)
    }
    fn alpha(&self, a: u8) -> Self {
        (self.0, self.1, self.2, a)
    }
    fn dim (&self, d: u8) -> Self {
        (self.0 / d, self.1 / d, self.2 / d, self.3)        
    }
}