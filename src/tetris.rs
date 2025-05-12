use crate::ui::draw_ui;
use crate::colors::*;
use macroquad::prelude::*;


#[macroquad::main("Tetris - by Erik & Justus")]
pub async fn main(){

    loop{
        clear_background(SKY);
        draw_ui();



        next_frame().await
    }
    
}