use crate::{colors::*, board::Board, ui::Ui};
use macroquad::{prelude::*, rand::gen_range};
use crate::ui::window_conf;

#[macroquad::main(window_conf)]
pub async fn main() {
    let ui = Ui::new();
    let mut board = Board::new();
    board.test_tiles();
    
    loop {
        clear_background(DARK_GRAY);
        ui.draw();

        if is_key_pressed(KeyCode::Space){
            board.test_tiles();
        }

        board.draw();
        next_frame().await
    }
}
