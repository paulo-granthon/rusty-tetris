use std::marker::Copy;
use std::default::Default;

// // moves the content of a 2D vec increasing it's size in the process
// pub fn translate<T:Copy + Default> (grid: &Vec<Vec<T>>, factor:[i8; 2]) -> Vec<Vec<T>>{
//     let fabsus:[usize;2] = [factor[1].abs() as usize, factor[0].abs() as usize];
//     let offset:[usize;2] = [if factor[1] > 0 {fabsus[0]} else {0}, if factor[0] > 0 {fabsus[1]} else {0}];
//     let mut result:Vec<Vec<T>> = vec![vec![Default::default(); grid[0].len() + fabsus[1]]; grid.len() + fabsus[0]];
//     for x in 0..grid.len() {
//         for y in 0..grid[0].len() {
//             result[x + offset[0]][y + offset[1]] = grid[x][y];
//         }
//     }
//     result
// }

// rotates a 2D vec by 90º or -90º if clockwise
pub fn rotate<T:Copy + Default> (grid: &Vec<Vec<T>>, clockwise: bool) -> Vec<Vec<T>> {
    let mut result:Vec<Vec<T>> = vec![vec![Default::default(); grid.len()]; grid[0].len()];
    for x in 0..grid.len() {
        for y in 0..grid[0].len() {
            result[y][if clockwise { grid.len() - 1 - x } else { x }] = grid[x][ if clockwise { y } else { grid[0].len() - 1 - y }];
        }
    }
    result
}

// // cuts a 2D vec by the amount defined by "dir". Positive values cuts the beggining of the vec and negative values cuts the ends
// pub fn trim<T:Copy + Default> (grid: &Vec<Vec<T>>, dir: [i8; 2]) -> Vec<Vec<T>> {
//     let offset:[[usize; 2]; 2] = [
//         [if dir[1] > 0 {dir[1].abs() as usize} else {0}, if dir[1] < 0 {dir[1].abs() as usize} else {0}],
//         [if dir[0] > 0 {dir[0].abs() as usize} else {0}, if dir[0] < 0 {dir[0].abs() as usize} else {0}]
//     ];
//     let mut result:Vec<Vec<T>> = vec![vec![Default::default(); grid[0].len() - offset[1][1]]; grid.len() - offset[0][1]];
//     for x in offset[0][0]..result.len() {
//         for y in offset[1][0]..result[0].len() {
//             result[x][y] = grid[x - offset[0][0]][y - offset[1][0]];
//         }
//     }
//     result
// }
