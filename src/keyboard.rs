use macroquad::input::{KeyCode, is_key_pressed};

use crate::board::Board;

pub fn controls(board: &mut Board) {
    if is_key_pressed(KeyCode::Left) {
        board.try_move_tile_left();
    }
    if is_key_pressed(KeyCode::Right) {
        board.try_move_tile_right();
    }
}
