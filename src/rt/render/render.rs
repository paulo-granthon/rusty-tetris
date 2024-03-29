extern crate doryen_rs; use doryen_rs::Console;
use crate::Align;

use crate::data::tetromino::Tetromino;
use crate::RTColor;

use crate::BLOCK_SCALE;
use crate::CONSOLE_WIDTH;   use crate::CONSOLE_HEIGHT;
use crate::PLAYFIELD_WIDTH; use crate::PLAYFIELD_HEIGHT;

// render position of the playfield
pub const R_PLAYFIELD_X: i32 = CONSOLE_WIDTH as i32 / 2 - (PLAYFIELD_WIDTH * BLOCK_SCALE) as i32 / 2 - 1;
pub const R_PLAYFIELD_Y: i32 = CONSOLE_HEIGHT as i32 / 2 - (PLAYFIELD_HEIGHT * BLOCK_SCALE) as i32 / 2 - 1;

// render sizes of the playfield
pub const R_PLAYFIELD_SIZE_X: u32 = (PLAYFIELD_WIDTH * BLOCK_SCALE) as u32 + 2;
pub const R_PLAYFIELD_SIZE_Y: u32 = (PLAYFIELD_HEIGHT * BLOCK_SCALE) as u32 + 2;

pub const NEXT_CON_WIDTH : u32 = 6;
pub const NEXT_CON_HEIGHT : u32 = 6;

pub fn clear (con: &mut Console) {
    con.clear(Some(RTColor::Black.u8()), Some(RTColor::Black.u8()), Some(' ' as u16));
}

// renders a playfield
pub fn render_playfield<'a, const W: usize, const H: usize> (playfield_con: Option<&'a mut Console>, playfield: &[[Option<RTColor>; H]; W], scale: i32, render_blocks: bool) -> Option<&'a mut Console> {

    match playfield_con {
        Some(pfcon) => {

            pfcon.clear(None, Some(RTColor::White.u8()), Some(' ' as u16));

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
                                color.u8(), 
                                scale,
                                1, 1,
                                Some(RTColor::Gray.u8())
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
            // con.clear(None, Some(RTColor::White.u8()), Some(' ' as u16));

            match tetromino {
                Some(t) => {

                    // for each position on the Tetromino's grid
                    for x in 0..t.grid.len() {
                        for y in 0..t.grid[0].len() {

                            let color = if t.grid[x][y] { t.color.u8() } else { RTColor::White.u8() };
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
pub fn render_score (con: &mut Console, x: i32, y: i32, score: i32) {  
    con.print_color(
        x,
        y,
        format!("{}", score).as_str(),
        doryen_rs::TextAlign::Center,
        Some(RTColor::Black.u8())
    );
}

// renders a popup stating that the game is paused
pub fn render_popup_window (
    con: &mut Console,
    x: i32, y: i32, w: u32, h: u32,
    anchor: (Align, Align),
    fore: Option<(u8, u8, u8, u8)>,
    back: Option<(u8, u8, u8, u8)>, 
    bg_char: Option<u16>
) {
    con.rectangle(
        x + anchor.0.value(w as i32),
        y + anchor.1.value(h as i32),
        w,
        h,
        fore,
        back,
        bg_char
    );

}

pub fn render_paused_popup (con: &mut Console, x: i32, y: i32, w: u32, h: u32, menu: &crate::PauseMenu) {
    render_popup_window(con, x, y, w, h, Align::center2(), Some(RTColor::DarkerGray.u8()), Some(RTColor::Black.u8()), Some('/' as u16));
    con.print(x, y-3, "Paused", doryen_rs::TextAlign::Center, Some(RTColor::White.u8()), None);
    let (exit_fore, exit_back, cont_fore, cont_back) = match menu.cursor {
        0 => (Some(RTColor::Red.u8()), Some(RTColor::White.u8()), Some(RTColor::DarkGray.u8()), None),
        _ => (Some(RTColor::DarkGray.u8()), None, Some(RTColor::Blue.u8()), Some(RTColor::White.u8()))
    };
    con.print(x - 8, y, "Exit", doryen_rs::TextAlign::Center, exit_fore, exit_back);
    con.print(x + 8, y, "Continue", doryen_rs::TextAlign::Center, cont_fore, cont_back);
}

pub fn render_game_over_popup (con: &mut Console, x: i32, y: i32, w: u32, h: u32) {
    render_popup_window(con, x, y, w, h, Align::center2(), Some(RTColor::Gray.u8()), Some(RTColor::Black.u8()), Some(' ' as u16));
    con.print(x, y, "Game Over", doryen_rs::TextAlign::Center, Some(RTColor::White.u8()), None);
}