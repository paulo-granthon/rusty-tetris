extern crate doryen_rs; use doryen_rs::{Console};

pub fn render_block (con: &mut Console, x: i32, y: i32, color: (u8, u8, u8, u8), scale: i32, offs_x: i32, offs_y : i32) {

    for bx in 0..scale {
        for by in 0..scale {

            // remove foreground ascii from target location
            if bx == 0 && by == 0 {con.ascii(x * scale + bx + offs_x, y * scale + by + offs_y, 1);}
            else {con.ascii(x * scale + bx + offs_x, y * scale + by + offs_y, 0);}

            // render this position if true, render blank if false
            con.back(x * scale + bx + offs_x, y * scale + by + offs_y, color);
        }
    }

}