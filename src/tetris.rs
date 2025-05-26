use crate::keyboard::controls;
use crate::ui::window_conf;
use crate::{board::Board, colors::*, ui::Ui};
use macroquad::prelude::*;

#[macroquad::main(window_conf)]
pub async fn main() {
    let ui = Ui::new();
    let mut board = Board::new();
    board.next_tetromino();
    loop {
        clear_background(DARK_GRAY);
        ui.draw(board.score);

        board.auto_move_tile_down();
        controls(&mut board);
        board.draw();
        Ui::draw_next_tile(&board);
        board.clear_lines_if_possible();
        next_frame().await
    }
}
