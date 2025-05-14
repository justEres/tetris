use crate::ui::window_conf;
use crate::{board::Board, colors::*, ui::Ui};
use macroquad::prelude::*;

#[macroquad::main(window_conf)]
pub async fn main() {
    let ui = Ui::new();
    let mut board = Board::new();
    board.test_tiles();

    let mut fall_counter = 0.;
    loop {
        fall_counter += get_frame_time();
        if fall_counter > 0.5 {
            fall_counter = 0.;
            if !board.check_for_collision() {
                if let Some(falling_tile) = &mut board.falling_tile {
                    falling_tile.grid_position.1 += 1;
                }
            }
        }

        clear_background(DARK_GRAY);
        ui.draw();

        if is_key_pressed(KeyCode::Space) {
            board.test_tiles();
        }

        board.draw();
        next_frame().await
    }
}
