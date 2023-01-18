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
    Gray,
    DarkGray,
    DarkerGray,
}

use std::slice::Iter;
use RTColor::*;
impl RTColor {
    pub fn iter() -> Iter<'static, RTColor> {
        static RTCOLORS: [RTColor; COUNT] = [Black, White, Red, Green, Blue, Yellow, Magenta, Cyan, Orange];
        RTCOLORS.iter()
    }
    pub fn text (&self) -> &str {
        match self {
            Black       => "black",
            White       => "white",
            Red         => "red",
            Green       => "green",
            Blue        => "blue",
            Yellow      => "yellow",
            Magenta     => "magenta",
            Cyan        => "cyan",
            Orange      => "orange",
            Gray        => "gray",
            DarkGray    => "dark_gray",
            DarkerGray  => "darker_gray",
        }
    }
    pub fn u8 (&self) -> (u8, u8, u8, u8) {
        match self {
            Black       => (0,     0,      0,      255),
            White       => (255,   255,    255,    255),
            Red         => (255,   92,     92,     255),
            Green       => (92,    255,    92,     255),
            Blue        => (92,    92,     255,    255),
            Yellow      => (255,   255,    92,     255),
            Magenta     => (255,   92,     255,    255),
            Cyan        => (92,    255,    255,    255),
            Orange      => (255,   92,     0,      255),
            Gray        => (127,   127,    127,    255),
            DarkGray    => (92,    92,     92,     255),
            DarkerGray  => (46,    46,     46,     255),
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