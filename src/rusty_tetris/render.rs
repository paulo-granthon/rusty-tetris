extern crate doryen_rs; use doryen_rs::Console;

use crate::data::tetromino::Tetromino;
use crate::data::rt_color::RTColor;

use crate::DEBUG_RENDER;

use crate::BLOCK_SCALE;

use crate::CONSOLE_WIDTH;
use crate::CONSOLE_HEIGHT;

use crate::PLAYFIELD_WIDTH;
use crate::PLAYFIELD_HEIGHT;

// render position of the playfield
const R_PLAYFIELD_X: i32 = CONSOLE_WIDTH as i32 / 2 - (PLAYFIELD_WIDTH * BLOCK_SCALE) as i32 / 2 - 1;
const R_PLAYFIELD_Y: i32 = CONSOLE_HEIGHT as i32 / 2 - (PLAYFIELD_HEIGHT * BLOCK_SCALE) as i32 / 2 - 1;

// render sizes of the playfield
const R_PLAYFIELD_SIZE_X: u32 = (PLAYFIELD_WIDTH * BLOCK_SCALE) as u32 + 2;
const R_PLAYFIELD_SIZE_Y: u32 = (PLAYFIELD_HEIGHT * BLOCK_SCALE) as u32 + 2;

const NEXT_CON_WIDTH : u32 = 6;
const NEXT_CON_HEIGHT : u32 = 8;

pub trait RenderEngine {
    fn rt_render (&mut self, con: &mut Console);
}

impl RenderEngine for super::RustyTetris {

    fn rt_render (&mut self, con: &mut Console) {
        match render_playfield(self.playfield_con.as_mut(), &self.playfield, (R_PLAYFIELD_SIZE_X, R_PLAYFIELD_SIZE_Y), BLOCK_SCALE as i32) {
            Some(pfcon) => pfcon.blit(
                R_PLAYFIELD_X,
                R_PLAYFIELD_Y,
                con,
                1.0,
                1.0,
                None
            ),
            None => {}
        }

        // get a reference to the current position of the Tetromino
        let cur_pos = self.cur_pos;

        let block_scale = BLOCK_SCALE as i32;
        let half_pf_width = PLAYFIELD_WIDTH as i32 / 2;
        let half_pf_height = PLAYFIELD_HEIGHT as i32 / 2;

        let half_con_width = CONSOLE_WIDTH as i32 / 2;
        let half_con_height = CONSOLE_HEIGHT as i32 / 2;
        
        con.print_color(
            half_con_width,
            half_pf_height,
            format!("{}", self.score).as_str(),
            doryen_rs::TextAlign::Center,
            Some(RTColor::Black.value().1)
        );

        // render the current Tetromino
        let s = self.get_skip_steps(&self.cur_tetromino.to_owned().unwrap());
        let t_con = self.cur_con.as_mut();

        match render_tetromino(t_con, &self.cur_tetromino, (0, 0), block_scale) {
            Some(cur_con) => {
                cur_con.blit(
                    half_con_width + (cur_pos.0 as i32 - half_pf_width) * block_scale,
                    half_con_height + (cur_pos.1 as i32 - half_pf_height) * block_scale,
                    con, 
                    1.0,
                    1.0, 
                    if DEBUG_RENDER {None} else {Some(RTColor::White.value().1)}
                );
                cur_con.blit(
                    half_con_width + (cur_pos.0 as i32 - half_pf_width) * block_scale,
                    half_con_height + ((cur_pos.1 + s) as i32 - half_pf_height) * block_scale,
                    con, 
                    0.3,
                    0.3, 
                    if DEBUG_RENDER {None} else {Some(RTColor::White.value().1)}
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
                ), block_scale) {
                    Some(nt_con) => {
                        nt_con.blit(
                            half_con_width + (R_PLAYFIELD_SIZE_X as i32 / 2),
                            half_con_height + (R_PLAYFIELD_SIZE_Y as i32 / 2) - (NEXT_CON_HEIGHT as i32 * block_scale),
                            con, 
                            1.0,
                            1.0, 
                            if DEBUG_RENDER {None} else {Some(RTColor::White.value().1)}
                        );
                    },
                    None => { println!("render -- peek_bag_next ")}
                }
            },
            None => { println!("render -- bag_peek_next returned None")}
        }

        
    }

}

// renders a playfield
pub fn render_playfield<'a, const W: usize, const H: usize> (playfield_con: Option<&'a mut Console>, playfield: &[[Option<RTColor>; H]; W], size:(u32, u32), scale: i32) -> Option<&'a mut Console> {

    match playfield_con {
        Some(pfcon) => {

            pfcon.clear(None, Some(RTColor::White.value().1), Some(' ' as u16));

            // render the playfield
            pfcon.rectangle(
                0,
                0,
                size.0,
                size.1,
                Some((128, 128, 128, 255)),
                Some((0, 0, 0, 255)),
                Some(' ' as u16),
            );

            for x in 0..playfield.len() {
                for y in 0..playfield[x].len() {
                    match playfield[x][y] {
                        Some(color) => render_block (
                            pfcon,
                            x as i32,
                            y as i32,
                            color.value().1, 
                            scale,
                            1, 1
                        ),
                        None => continue
                    };
                }
            }
            Some(pfcon)

        }
        None => None
    }


}

// renders a Tetromino
pub fn render_tetromino<'a>(t_con: Option<&'a mut Console>, tetromino: &Option<Tetromino>, pos: (i8, i8), scale:i32) -> Option<&'a mut Console> {

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
                            render_block(con, pos.0 as i32 + x as i32, pos.1 as i32 + y as i32, color, scale, 0, 0);
                            
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

const CHAR_GRID: [[u16; 2]; 2] = [
    // [187, 188],
    // [300, 301]
    [201, 187],
    [200, 188]
];

// renders a single block of a Tetromino
pub fn render_block (con: &mut Console, x: i32, y: i32, color: (u8, u8, u8, u8), scale: i32, offs_x: i32, offs_y : i32) {

    for bx in 0..scale {
        for by in 0..scale {

            // remove foreground ascii from target location
            con.ascii(x * scale + bx + offs_x, y * scale + by + offs_y, CHAR_GRID[by as usize][bx as usize]);

            // render this position if true, render blank if false
            con.back(x * scale + bx + offs_x, y * scale + by + offs_y, color);
        }
    }

}