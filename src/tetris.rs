use crate::{colors::*, tetris_constants::{BOARD_WIDTH, TILE_SIZE}, ui::Ui};
use macroquad::{miniquad::window::set_window_size, prelude::*};
use crate::ui::window_conf;

#[macroquad::main(window_conf)]
pub async fn main() {
    let ui = Ui::new();
    
    loop {
        clear_background(SKY);
        ui.draw();

        next_frame().await
    }
}
