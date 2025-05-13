use std::collections::HashSet;

use macroquad::color::Color;

use crate::colors::ORANGE;




pub struct Tetromino{
    pub tiles: HashSet<(i8,i8)>,
    pub grid_position: (u8,u8),
    pub color: Color,
}


impl Tetromino{

    //example: the square or 2x2 Tetromino
    pub fn new_block_tetromino() -> Tetromino{
        let tiles: HashSet<(i8,i8)> = HashSet::from([(0,0),(1,0),(0,1),(1,1)]);
        Tetromino { tiles , grid_position: (5,0) , color: ORANGE}
    }
}



