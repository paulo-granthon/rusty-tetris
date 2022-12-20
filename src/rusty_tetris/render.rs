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

        // render the current Tetromino
        let s = self.get_skip_steps(&self.cur_tetromino.to_owned().unwrap());
        let t_con = self.cur_con.as_mut();

        match render_tetromino(t_con, &self.cur_tetromino, BLOCK_SCALE as i32) {
            Some(cur_con) => {
                cur_con.blit(
                    (CONSOLE_WIDTH as i32 / 2) + (cur_pos.0 as i32 - (PLAYFIELD_WIDTH as i32 / 2)) * BLOCK_SCALE as i32,
                    (CONSOLE_HEIGHT as i32 / 2) + (cur_pos.1 as i32 - (PLAYFIELD_HEIGHT as i32 / 2)) * BLOCK_SCALE as i32,
                    con, 
                    1.0,
                    1.0, 
                    if DEBUG_RENDER {None} else {Some(RTColor::White.value().1)}
                );
                cur_con.blit(
                    (CONSOLE_WIDTH as i32 / 2) + (cur_pos.0 as i32 - (PLAYFIELD_WIDTH as i32 / 2)) * BLOCK_SCALE as i32,
                    (CONSOLE_HEIGHT as i32 / 2) + ((cur_pos.1 + s) as i32 - (PLAYFIELD_HEIGHT as i32 / 2)) * BLOCK_SCALE as i32,
                    con, 
                    0.5,
                    0.5, 
                    if DEBUG_RENDER {None} else {Some(RTColor::White.value().1)}
                );

            },
            None => {}
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
                Some('.' as u16),
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
pub fn render_tetromino<'a>(t_con: Option<&'a mut Console>, tetromino: &Option<Tetromino>, scale:i32) -> Option<&'a mut Console> {

    // match Some / None
    match t_con {

        // if Some
        Some(con) => {

            // clear the Tetromino's console 
            con.clear(None, Some(RTColor::White.value().1), Some(' ' as u16));

            match tetromino {
                Some(t) => {

                    // for each position on the Tetromino's grid
                    for x in 0..t.grid.len() {
                        for y in 0..t.grid[0].len() {

                            let color = if t.grid[x][y] { t.color.value().1 } else { RTColor::White.value().1 };
                            render_block(con, x as i32, y as i32, color, scale, 0, 0);
                            
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
pub fn render_block (con: &mut Console, x: i32, y: i32, color: (u8, u8, u8, u8), scale: i32, offs_x: i32, offs_y : i32) {

    for bx in 0..scale {
        for by in 0..scale {

            // remove foreground ascii from target location
            con.ascii(x * scale + bx + offs_x, y * scale + by + offs_y, 0);

            // render this position if true, render blank if false
            con.back(x * scale + bx + offs_x, y * scale + by + offs_y, color);
        }
    }

}