use super::rtcolor::RTColor;


pub fn clamp_boundaries (pos:(i8, i8), start:(i8, i8), end:(i8, i8)) -> (i8, i8) {
    ((pos.0).max(start.0).min(end.0), (pos.1).max(start.1).min(end.1))
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

            let block_pos = (pos.0 + x, pos.1 + y);

            // if outside bounds 
            if {

                let mut oob = false;

                // check outside bounds x
                if block_pos.0 + dir.0 < 0 || block_pos.0 + dir.0 >= field_size.0 {
                    // cur_coll_x -= dir.0.signum();
                    oob = true;
                }

                // check outside bounds y
                if block_pos.1 + dir.1 < 0 || block_pos.1 + dir.1 >= field_size.1 {
                    // if !detected_y_at_x {
                    //     cur_coll_y[y as usize] -= dir.1.signum();
                    //     detected_y_at_x = true;    
                    // }
                    oob = true;
                }

                // not outside bounds
                oob

            // if outside bounds detected, skip 
            } { continue; }
            
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
    