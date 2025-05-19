use crate::keyboard::controls;
use crate::ui::window_conf;
use crate::{board::Board, colors::*, ui::Ui};
use macroquad::prelude::*;

#[macroquad::main(window_conf)]
pub async fn main() {
    let ui = Ui::new();
    let mut board = Board::new();
    board.new_random_falling_tile();

    loop {
        clear_background(DARK_GRAY);
        ui.draw();

        board.move_tile_down();
        controls(&mut board);
        board.draw();
        next_frame().await
    }
}
