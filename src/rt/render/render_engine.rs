use doryen_rs::Console;

use crate::{Game, RunState, render::*, RTColor};

use crate::DEBUG_RENDER;

use crate::BLOCK_SCALE;
use crate::CONSOLE_WIDTH;   use crate::CONSOLE_HEIGHT;
use crate::PLAYFIELD_WIDTH; use crate::PLAYFIELD_HEIGHT;

pub trait RenderEngine {
    fn rt_render (&mut self, con: &mut Console);
}

impl RenderEngine for Game {

    fn rt_render (&mut self, con: &mut Console) {

        let paused = match self.run_state { crate::RunState::Paused(_) => true, _=> false };

        let side =  if self.player == 0 {0} else { ((self.player as i32 - 1) * 2) - 1 };

        let player_x_offset = ((R_PLAYFIELD_SIZE_X as i32 / 2) + 1) * side;

        let block_scale = BLOCK_SCALE as i32;
        let _half_pf_width = PLAYFIELD_WIDTH as i32 / 2;
        let half_pf_height = PLAYFIELD_HEIGHT as i32 / 2;

        let half_con_width = CONSOLE_WIDTH as i32 / 2;
        let half_con_height = CONSOLE_HEIGHT as i32 / 2;

        // con.back( half_con_width + player_x_offset, 0, RTColor::Orange.u8());

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

        if match &self.run_state {
            RunState::Paused(menu) => {
                render_paused_popup(con, half_con_width + player_x_offset, half_con_height, 32, 7, menu);
                true
            },
            RunState::Over => {
                render_game_over_popup(con, half_con_width + player_x_offset, half_con_height, 24, 7);
                con.print(half_con_width + player_x_offset, half_con_height + 1, format!("Scored {} points!", &self.score).as_str(), doryen_rs::TextAlign::Center, Some(RTColor::Red.u8()), None);
                true
            }
            _=> false
        } { return; }

        let white = Some(RTColor::White.u8());

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
                nt_con.clear(Some(RTColor::Black.u8()), Some(RTColor::Black.u8()), None);
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

        use crate::HasBag;
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

        // con.back(28, 10, (127, 127, 0, 127));
        // con.back(50, 10, (127, 127, 0, 127));

        
    }

}

