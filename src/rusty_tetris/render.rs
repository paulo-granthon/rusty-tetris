extern crate doryen_rs; use doryen_rs::Console;

use crate::data::tetromino::Tetromino;
use crate::data::rt_color::RTColor;

use crate::DEBUG_RENDER;

use crate::BLOCK_SCALE;

use crate::CONSOLE_WIDTH;
use crate::CONSOLE_HEIGHT;

use crate::PLAYFIELD_WIDTH;
use crate::PLAYFIELD_HEIGHT;
use crate::rusty_tetris::RunState;

// render position of the playfield
const R_PLAYFIELD_X: i32 = CONSOLE_WIDTH as i32 / 2 - (PLAYFIELD_WIDTH * BLOCK_SCALE) as i32 / 2 - 1;
const R_PLAYFIELD_Y: i32 = CONSOLE_HEIGHT as i32 / 2 - (PLAYFIELD_HEIGHT * BLOCK_SCALE) as i32 / 2 - 1;

// render sizes of the playfield
const R_PLAYFIELD_SIZE_X: u32 = (PLAYFIELD_WIDTH * BLOCK_SCALE) as u32 + 2;
const R_PLAYFIELD_SIZE_Y: u32 = (PLAYFIELD_HEIGHT * BLOCK_SCALE) as u32 + 2;

const NEXT_CON_WIDTH : u32 = 6;
const NEXT_CON_HEIGHT : u32 = 6;

pub trait RenderEngine {
    fn rt_render (&mut self, con: &mut Console);
}

impl RenderEngine for super::RustyTetris {

    fn rt_render (&mut self, con: &mut Console) {

        let paused = match self.run_state { crate::rusty_tetris::RunState::Paused => true, _=> false };

        let side = match self.player { Some(p) => ((p as i32 - 1) * 2) - 1, _=> 0 };

        let player_x_offset = ((R_PLAYFIELD_SIZE_X as i32 / 2) + 1) * side;

        let block_scale = BLOCK_SCALE as i32;
        let _half_pf_width = PLAYFIELD_WIDTH as i32 / 2;
        let half_pf_height = PLAYFIELD_HEIGHT as i32 / 2;

        let half_con_width = CONSOLE_WIDTH as i32 / 2;
        let half_con_height = CONSOLE_HEIGHT as i32 / 2;

        con.back( half_con_width + player_x_offset, 0, RTColor::Orange.value().1);

        match render_playfield(self.playfield_con.as_mut(), &self.playfield, BLOCK_SCALE as i32, !paused) {
            Some(pfcon) => {
                pfcon.blit(
                    R_PLAYFIELD_X + player_x_offset,
                    R_PLAYFIELD_Y,
                    con,
                    1.0,
                    1.0,
                    None
                )
            },
            None => { println!("render playfield error") }
        }

        // get a reference to the current position of the Tetromino
        let cur_pos = (self.cur_pos.0 + player_x_offset as i8, self.cur_pos.1 );

        // render the score
        render_score(con, half_con_width + player_x_offset, half_pf_height, self.score);

        if match self.run_state {
            RunState::Paused => {
                render_paused_popup(con, half_con_width + player_x_offset, half_con_height, 24, 7);
                true
            },
            RunState::Over => {
                render_game_over_popup(con, half_con_width + player_x_offset, half_con_height, 24, 7);
                con.print(half_con_width + player_x_offset, half_con_height + 1, format!("Scored {} points!", &self.score).as_str(), doryen_rs::TextAlign::Center, Some(RTColor::Red.value().1), None);
                true
            }
            _=> false
        } { return; }

        let white = Some(RTColor::White.value().1);

        // render the current Tetromino
        let s = self.get_skip_steps(&self.cur_tetromino.to_owned().unwrap());
        let t_con = self.cur_con.as_mut();

        match render_tetromino(t_con, &self.cur_tetromino, (0, 0), block_scale, white) {
            Some(cur_con) => {
                cur_con.blit(
                    R_PLAYFIELD_X - player_x_offset + (1 + cur_pos.0 as i32 * block_scale) ,
                    half_con_height + (cur_pos.1 as i32 - half_pf_height) * block_scale,
                    con, 
                    1.0,
                    1.0, 
                    if DEBUG_RENDER {None} else {white}
                );
                cur_con.blit(
                    R_PLAYFIELD_X - player_x_offset + (1 + cur_pos.0 as i32 * block_scale) ,
                    half_con_height + ((cur_pos.1 + s) as i32 - half_pf_height) * block_scale,
                    con, 
                    0.3,
                    0.3, 
                    if DEBUG_RENDER {None} else {white}
                );

            },
            None => {}
        }

        match self.next_con.as_mut() {
            Some (nt_con) => {
                nt_con.clear(Some(RTColor::Black.value().1), Some(RTColor::Black.value().1), None);
                nt_con.rectangle(
                    0,
                    0,
                    NEXT_CON_WIDTH * block_scale as u32,
                    NEXT_CON_HEIGHT * block_scale as u32,
                    Some((128, 128, 128, 255)),
                    Some((80, 80, 80, 255)),
                    None,
                );                    
            }
            None => {}
        }

        use super::HasBag;
        match self.bag_peek_next() {
            Some(next_tetromino) => { 
                let nt = next_tetromino.get();
                let nt_width = nt.grid.len();
                let nt_heigth = nt.grid[0].len();

                let nt_con = self.next_con.as_mut();
                match render_tetromino(nt_con, &Some(nt), (
                    (NEXT_CON_WIDTH as i8 - nt_width  as i8) / 2, 
                    (NEXT_CON_WIDTH as i8 - nt_heigth as i8) - 1,
                ), block_scale, white) {
                    Some(nt_con) => {
                        const R_HALF_PF_SIZE_X_I32: i32 = R_PLAYFIELD_SIZE_X as i32 / 2;
                        nt_con.blit(
                            half_con_width + player_x_offset + (R_HALF_PF_SIZE_X_I32 * (1 - side.abs())) - (R_HALF_PF_SIZE_X_I32 * -side) + ((NEXT_CON_WIDTH as i32 * 2) * side.min(0)),
                            half_con_height + (R_PLAYFIELD_SIZE_Y as i32 / 2) - (NEXT_CON_HEIGHT as i32 * block_scale),
                            con, 
                            1.0,
                            1.0, 
                            if DEBUG_RENDER {None} else { white }
                        );
                    },
                    None => { println!("render -- peek_bag_next ")}
                }
            },
            None => { println!("render -- bag_peek_next returned None")}
        }

        con.back(28, 10, (127, 127, 0, 127));
        con.back(50, 10, (127, 127, 0, 127));

        
    }

}

// renders a playfield
pub fn render_playfield<'a, const W: usize, const H: usize> (playfield_con: Option<&'a mut Console>, playfield: &[[Option<RTColor>; H]; W], scale: i32, render_blocks: bool) -> Option<&'a mut Console> {

    match playfield_con {
        Some(pfcon) => {

            pfcon.clear(None, Some(RTColor::White.value().1), Some(' ' as u16));

            // render the playfield
            pfcon.rectangle(
                0,
                0,
                W as u32 * BLOCK_SCALE as u32 + 2,
                H as u32 * BLOCK_SCALE as u32 + 2,
                Some((128, 128, 128, 255)),
                Some((0, 0, 0, 255)),
                Some(' ' as u16),
            );

            if render_blocks {
                for x in 0..playfield.len() {
                    for y in 0..playfield[x].len() {
                        match playfield[x][y] {
                            Some(color) => render_block (
                                pfcon,
                                x as i32,
                                y as i32,
                                color.value().1, 
                                scale,
                                1, 1,
                                Some(RTColor::Grey.value().1)
                            ),
                            None => continue
                        };
                    }
                }
            }
            Some(pfcon)

        }
        None => None
    }


}

// renders a Tetromino
pub fn render_tetromino<'a>(t_con: Option<&'a mut Console>, tetromino: &Option<Tetromino>, pos: (i8, i8), scale:i32, fore: Option<(u8, u8, u8, u8)>) -> Option<&'a mut Console> {

    // match console Some / None
    match t_con {

        // if Some
        Some(con) => {

            // clear the Tetromino's console 
            // con.clear(None, Some(RTColor::White.value().1), Some(' ' as u16));

            match tetromino {
                Some(t) => {

                    // for each position on the Tetromino's grid
                    for x in 0..t.grid.len() {
                        for y in 0..t.grid[0].len() {

                            let color = if t.grid[x][y] { t.color.value().1 } else { RTColor::White.value().1 };
                            render_block(con, pos.0 as i32 + x as i32, pos.1 as i32 + y as i32, color, scale, 0, 0, fore);
                            
                        }
                    };
                }
                None => return None,
            }

            // return the console reference
            Some(con)
        },

        // no current Tetromino, return nothing
        None => None
    }

}

// renders a single block of a Tetromino
pub fn render_block (con: &mut Console, x: i32, y: i32, color: (u8, u8, u8, u8), scale: i32, offs_x: i32, offs_y : i32, fore: Option<(u8, u8, u8, u8)>) {

    // chars used to render a Tetromino's block
    const CHAR_GRID: [[u16; 2]; 2] = [[201, 187], [200, 188]];

    // loop through the block
    for bx in 0..scale {
        for by in 0..scale {

            // remove foreground ascii from target location
            con.ascii(x * scale + bx + offs_x, y * scale + by + offs_y, CHAR_GRID[by as usize][bx as usize]);

            // render this position if true, render blank if false
            con.back(x * scale + bx + offs_x, y * scale + by + offs_y, color);
            if let Some(fore_col) = fore { con.fore(x * scale + bx + offs_x, y * scale + by + offs_y, fore_col); }
        }
    }

}

// renders the player's score
fn render_score (con: &mut Console, x: i32, y: i32, score: i32) {  
    con.print_color(
        x,
        y,
        format!("{}", score).as_str(),
        doryen_rs::TextAlign::Center,
        Some(RTColor::Black.value().1)
    );
}

#[allow(dead_code)]
enum Align { 
    Start,
    Fraction(i32),
    End,
    Custom(i32)
}

impl Align {

    // short for Fraction(2)
    fn center () -> Self { Align::Fraction(2) }

    // short for (Fraction(2), Fraction(2))
    fn center2 () -> (Self, Self) { (Align::center(), Align::center()) } 

    // returns the i32 value of the Align for the given range 
    fn value (&self, size: i32) -> i32 {
        match &self {
            Align::Start => 0,
            Align::Fraction(x) => if x.to_owned() == 0 {0} else {size / x},
            Align::End => size,
            Align::Custom(x) => x.to_owned(),
        }
    }
}

// renders a popup stating that the game is paused
fn render_popup_window (
    con: &mut Console,
    x: i32, y: i32, w: u32, h: u32,
    anchor: (Align, Align),
    fore: Option<(u8, u8, u8, u8)>,
    back: Option<(u8, u8, u8, u8)>, 
    bg_char: Option<u16>
) {
    con.rectangle(
        x - anchor.0.value(w as i32),
        y - anchor.1.value(h as i32),
        w,
        h,
        fore,
        back,
        bg_char
    );

}

fn render_paused_popup (con: &mut Console, x: i32, y: i32, w: u32, h: u32) {
    render_popup_window(con, x, y, w, h, Align::center2(), Some(RTColor::Grey.value().1), Some(RTColor::Black.value().1), Some('/' as u16));
    con.print(x, y, "Paused", doryen_rs::TextAlign::Center, Some(RTColor::White.value().1), None);
}

fn render_game_over_popup (con: &mut Console, x: i32, y: i32, w: u32, h: u32) {
    render_popup_window(con, x, y, w, h, Align::center2(), Some(RTColor::Grey.value().1), Some(RTColor::Black.value().1), Some(' ' as u16));
    con.print(x, y, "Game Over", doryen_rs::TextAlign::Center, Some(RTColor::White.value().1), None);
    // con.print(x, y, "Game Over", doryen_rs::TextAlign::Center, Some(RTColor::White.value().1), None);
}