use crate::{colors::*, ui::Ui};
use macroquad::prelude::*;

#[macroquad::main("Tetris - by Erik & Justus")]
pub async fn main() {
    let ui = Ui::new();
    loop {
        clear_background(SKY);
        ui.draw();

        next_frame().await
    }
}
