use std::collections::HashMap;

use macroquad::{color::Color, shapes::{draw_rectangle, draw_rectangle_lines}};

use crate::{colors::{BLACK, BLUE, ORANGE, PURPLE, RED}, tetris_constants::TILE_SIZE, tetromino::Tetromino};





pub struct Board{
    draw_offset: (u32,u32),
    ground_tiles: HashMap<(u8,u8),Color>,
    falling_tile: Option<Tetromino>,
}

impl Board{
    pub fn new() -> Board{
        let draw_offset = (TILE_SIZE as u32 * 2, TILE_SIZE as u32 * 2);
        Board { draw_offset, ground_tiles: HashMap::new(), falling_tile: None }
    }

    pub fn draw(&self) {
        for tile in self.ground_tiles.iter(){
            self.draw_single_tile(tile.0, tile.1);
        }
        if let Some(tetromino) = &self.falling_tile{
            for tile in &tetromino.tiles{
                self.draw_single_tile(&(&tetromino.grid_position.0 + tile.0 as u8,&tetromino.grid_position.1 + tile.1 as u8), &tetromino.color);
            }
        }
    }

    fn draw_single_tile(&self, pos: &(u8,u8), color: &Color){
        const LINE_THICKNESS: f32 = 2.;
        draw_rectangle((self.draw_offset.0 + pos.0 as u32 * TILE_SIZE as u32) as f32, (self.draw_offset.1 + pos.1 as u32 * TILE_SIZE as u32) as f32, TILE_SIZE as f32, TILE_SIZE as f32, *color);
        draw_rectangle_lines((self.draw_offset.0 + pos.0 as u32 * TILE_SIZE as u32) as f32, (self.draw_offset.1 + pos.1 as u32 * TILE_SIZE as u32) as f32, TILE_SIZE as f32, TILE_SIZE as f32,LINE_THICKNESS, BLACK);
    }

    pub fn test_tiles(&mut self){

        

        for x in 0..10{
            for y in 0..20{
                self.ground_tiles.insert((x,y), RED);
            }
        }
        self.falling_tile = Some(Tetromino::new_block_tetromino());
    }
}

