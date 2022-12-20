use super::{RustyTetris, RTColor};
use super::routine_handler::*;
use super::input_handler::*;
use super::render::*;

extern crate doryen_rs; use doryen_rs::{DoryenApi, Engine, UpdateEvent};

// Doryen engine implementation for RustyTetris
impl Engine for RustyTetris {

    // initialize the engine
    fn init(&mut self, api: &mut dyn DoryenApi) {
        self.register_inputs();
        self.register_routines();

        // register colors 
        for color in RTColor::iter() {
            api.con().register_color(color.value().0, color.value().1);
        }

        // get the first Tetromino for the match
        self.next();
    }

    // Called every frame
    fn update(&mut self, api: &mut dyn DoryenApi) -> Option<UpdateEvent> {

        // get the current input
        let input = api.input();

        self.mouse_pos = input.mouse_pos();

        self.handle_input(input, "game");

        if self.paused { return None }

        self.handle_routines("not_paused");

        // self.handle_input(input, "game");

        // capture the screen
        // if input.key("ControlLeft") && input.key_pressed("KeyS") {
        //     self.screenshot_idx += 1;
        //     return Some(UpdateEvent::Capture(format!(
        //         "screenshot_{:03}.png",
        //         self.screenshot_idx
        //     )));
        // }
        None
    }

    // master render method
    fn render(&mut self, api: &mut dyn DoryenApi) {
        
        // initialize the console
        let con = api.con();
        con.clear(Some(RTColor::Black.value().1), Some(RTColor::Black.value().1), Some(' ' as u16));

        self.rt_render(con);

        // match &self.cur_con {
        //     Some(x) => x.blit(self.cur_pos.0.into(), self.cur_pos.1.into(), con, 1.0, 1.0, None),
        //     _=>{}
        // }
        // con.area(
        //     10,
        //     10,
        //     5,
        //     5,
        //     Some((255, 64, 64, 255)),
        //     Some((128, 32, 32, 255)),
        //     Some('&' as u16),
        // );
        // con.ascii(0, 1, '@' as u16);
        // // con.fore(self.player_pos.0, self.player_pos.1, (255, 255, 255, 255));
        // con.print_color(
        //     (CONSOLE_WIDTH / 2) as i32,
        //     (CONSOLE_HEIGHT - 1) as i32,
        //     "#[red]arrows#[white] : move - #[red]CTRL-S#[white] : save screenshot",
        //     TextAlign::Center,
        //     None,
        // );
        // con.print_color(
        //     (CONSOLE_WIDTH / 2) as i32,
        //     (CONSOLE_HEIGHT - 3) as i32,
        //     &format!(
        //         "#[white]Mouse coordinates: #[red]{}, {}",
        //         self.mouse_pos.0, self.mouse_pos.1
        //     ),
        //     TextAlign::Center,
        //     None,
        // );
        // con.print_color(
        //     5,
        //     5,
        //     "#[blue]This blue text contains a #[red]red#[] word",
        //     TextAlign::Left,
        //     None,
        // );
        // con.back(
        //     self.mouse_pos.0 as i32,
        //     self.mouse_pos.1 as i32,
        //     (255, 255, 255, 255),
        // );
        // render_block(
        //     con, 
        //     self.mouse_pos.0 as i32 / BLOCK_SCALE as i32, 
        //     self.mouse_pos.1 as i32 / BLOCK_SCALE as i32, 
        //     RTColor::White.value().1, 
        //     BLOCK_SCALE as i32, 
        //     0, 0
        // );
    }
}

