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
        ui.draw(board.score);

        board.auto_move_tile_down();
        controls(&mut board);
        board.draw();
        board.clear_lines_if_possible();
        next_frame().await
    }
}
