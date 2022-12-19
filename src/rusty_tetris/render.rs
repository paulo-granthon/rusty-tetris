extern crate doryen_rs; use doryen_rs::Console;

use crate::data::tetromino::Tetromino;
use crate::data::rt_color::RTColor;

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