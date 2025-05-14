use crate::{colors::*, game::Board, ui::Ui};
use macroquad::prelude::*;
use crate::ui::window_conf;

#[macroquad::main(window_conf)]
pub async fn main() {
    let ui = Ui::new();
    let mut board = Board::new();
    
    
    loop {
        clear_background(DARK_GRAY);
        ui.draw();
        board.draw();
        next_frame().await
    }
}
