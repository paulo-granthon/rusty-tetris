use super::rt_color::RTColor;
use super::tetromino::Tetromino;


// returns true if the given popsition is outside the range (0..size.x, 0..size.y)
pub fn out_of_bounds (pos: (i8, i8), size: (i8, i8)) -> bool { pos.0 < 0 || pos.0 >= size.0 || pos.1 < 0 || pos.1 >= size.1 }

// returns how much the given value it outside the axis range of 0..size
pub fn out_of_bounds_axis (v: i8, size: i8) -> i8 { (size - 1 - v).min(0).min(v) }

pub fn clamp_boundaries (pos:(i8, i8), start:(i8, i8), end:(i8, i8)) -> (i8, i8) {
    ((pos.0).max(start.0).min(end.0), (pos.1).max(start.1).min(end.1))
}

// simulates the movement of given tetromino at the given position towards given direction inside given playfield
pub fn simulate_move_y<const H: usize, const W: usize> (
    tetromino: &Tetromino, pos: (i8, i8), dir: (i8, i8), playfield: &[[Option<RTColor>; H]; W]

// returns (new_pos.x, new_pos.y, correction.x, correction.y)
) -> (i8, i8, i8, i8) {
                
    let width = tetromino.grid[0].len();
    let height = tetromino.grid.len();
    
    // calculate the new position of the Tetromino by clamping it a bit over the palyfield
    // since collision is defined by the Tetromino's grid instead of bounding box
    let new_pos = clamp_boundaries(
        (pos.0 + dir.0, pos.1 + dir.1),
        (-(width as i8), -(height as i8)),
        (W as i8, H as i8),
    );

    // calculate the correcttion value to further clamp the Tetromino inside the playfield
    let correction: (i8, i8) = get_correction(
        &tetromino.grid, 
        new_pos, 
        dir, 
        (W as i8, H as i8)
    );

    // calculate the correction value in regards to collision with other Tetrominos on the playfield
    let collision: (i8, i8) = get_collision(
        &tetromino.grid, 
        pos, 
        dir, 
        playfield
    );

    // // debugging :D
    // if correction.0 != 0 || correction.1 != 0 || collision.0 != 0 || collision.1 != 0{
    //     println!("{}, {}", &self.playfield.len(), &self.playfield[1].len());
    //     println!("correction: ({}, {})\tcollision: ({}, {})", correction.0, correction.1, collision.0, collision.1);
    // }
    
    // pick the biggest correction as the actual correction that should be applied to the position of the Tetromino
    (
        new_pos.0, new_pos.1,
        if collision.0.abs() > correction.0.abs() { collision.0 } else { correction.0 },
        if collision.1.abs() > correction.1.abs() { collision.1 } else { correction.1 },
    )

    // // mode debugging :D
    // if correction.0 != 0 || correction.1 != 0 { println!("correction result: {}, {}", correction.0, correction.1)}
}

pub fn get_correction (grid:&Vec<Vec<bool>>, pos:(i8, i8), dir: (i8, i8), max_pos:(i8, i8)) -> (i8, i8) {
    
    // initialize the correction variable to be calculated
    let mut correction:(i8, i8) = (0, 0);
    
    // get target grid dimensions
    let grid_size:(i8, i8) = (grid.len() as i8, grid[0].len() as i8);

    // create variables to store the current y correction values in the loop 
    let mut cur_cor_y:Vec<i8> = vec![0; grid_size.1 as usize];

    // loop through y
    for y in 0..grid_size.1 {
        
        // initialize a variable to store the x correction of this y
        let mut cur_cor_x: i8 = 0;

        // initialize a bool to define if collision is detected on this y
        let mut detected_y_at_x = false;

        // loop through x
        for x in 0..grid_size.0 {

            // skip if no block on grid at x y
            if !grid[x as usize][y as usize] { continue; }

            // if x + dir.x is outside boundaries, detect the collision
            if (pos.0 + x as i8) < 0 || pos.0 + x >= max_pos.0 {
                cur_cor_x -= dir.0.signum() 
            }

            // skip if y collision is already detected for this (y,x)
            if detected_y_at_x { continue; }

            // if y + dir.y is outside boundaries, detect the collision
            if (pos.1 + y as i8) < 0 || pos.1 + y >= max_pos.1 {
                cur_cor_y[y as usize] -= dir.1.signum();
                detected_y_at_x = true;
            }
        }

        // if the biggest x correction value is smaller than the correction value of this x, replace it
        if cur_cor_x.abs() > correction.0.abs() {
            correction.0 = cur_cor_x
        }
    }

    // loop through the correction values of each y
    for i in 0..cur_cor_y.len() {

        // if the biggest y correction value is smaller than the correction value of this y, replace it
        if cur_cor_y[i].abs() > correction.1.abs() { correction.1 = cur_cor_y[i] }
    }

    // return the calculated value
    correction
}

pub fn get_collision <const W: usize, const H: usize> (grid: &Vec<Vec<bool>>, pos:(i8, i8), dir: (i8, i8), field: &[[Option<RTColor>; W]; H]) -> (i8, i8) {
    
    // get target grid and field dimensions
    let grid_size : (i8, i8) = ( grid.len() as i8,  grid[0].len() as i8);
    let field_size: (i8, i8) = (field.len() as i8, field[0].len() as i8);

    // loop through y
    for y in 0..grid_size.1 {

        // loop through x
        for x in 0..grid_size.0 {

            // skip if no block on grid at x y
            if !grid[x as usize][y as usize] { continue; }

            // calculate the position of this block inside of Tetromino
            let block_pos = (pos.0 + x, pos.1 + y);

            // skip if outside bounds
            if out_of_bounds((block_pos.0 + dir.0, block_pos.1 + dir.1), field_size) { continue; }
            
            match field[(block_pos.0 + dir.0) as usize][(block_pos.1 + dir.1) as usize] {
                Some (_color) => {
                    // println!("X: Some at ({}, {})", (block_pos.0 + dir.0) as usize, (block_pos.1 + dir.1) as usize);
                    return (-dir.0, -dir.1);
                },
                None => {
                    // println!("X: None at ({}, {})", (block_pos.0 + dir.0) as usize, (block_pos.1 + dir.1) as usize)
                }
            }


        }

    }

    // return the calculated value
    (0, 0)
}


pub fn get_rot_correction <const W: usize, const H: usize> (grid: &Vec<Vec<bool>>, pos:(i8, i8), field: &[[Option<RTColor>; W]; H]) -> i8 {
    
    // get target grid and field dimensions
    let grid_size : (i8, i8) = ( grid.len() as i8,  grid[0].len() as i8);
    let field_size: (i8, i8) = (field.len() as i8, field[0].len() as i8);
    
    // initialize the correction value
    let mut correction: i8 = 0;

    // loop through y
    for y in 0..grid_size.1 {

        // loop through x
        for x in 0..grid_size.0 {

            // skip if no block on grid at x y
            if !grid[x as usize][y as usize] { continue; }

            // calculate the position of this block inside of Tetromino
            let block_pos = (pos.0 + x, pos.1 + y);

            // skip if outside bounds vertically
            if out_of_bounds_axis(block_pos.1, field_size.1) != 0 { continue; }

            // get how much this block is outside bounds horizontaly (0 if inside)
            let oob_correction = out_of_bounds_axis(block_pos.0, field_size.0);

            // if not 0
            if oob_correction != 0 {
                println!("oob_x: {}", oob_correction);

                // compare with current correction
                if oob_correction.abs() > correction.abs() { correction = oob_correction; }

                // don't check collision in this case
                continue;
            }
            
            // match this position on the palyfield
            match field[block_pos.0 as usize][block_pos.1 as usize] {

                // if contains block
                Some (_color) => {

                    // calculate the correction 
                    let corr_at_x = x - (grid_size.0 / 2);

                    // compare with current correction
                    if corr_at_x.abs() > correction.abs() { correction = corr_at_x }
                },

                // no block: nothing to compare
                None => {}
            }

        }
    }

    // return the calculated value 
    correction
}