use std::collections::HashMap;

use macroquad::{
    color::Color,
    shapes::{draw_rectangle, draw_rectangle_lines},
    time::get_frame_time,
};

use crate::{
    colors::{BLACK, SKY},
    tetris_constants::{BOARD_HEIGHT, BOARD_WIDTH, TILE_SIZE},
    tetromino::Tetromino,
};

pub struct Board {
    draw_offset: (u32, u32),
    ground_tiles: HashMap<(u8, u8), Color>,
    pub falling_tile: Option<Tetromino>,
    fall_counter: f32,
    fall_speed: u32,
}

impl Board {
    pub fn new() -> Board {
        let draw_offset = (TILE_SIZE as u32 * 2, TILE_SIZE as u32 * 2);
        Board {
            draw_offset,
            ground_tiles: HashMap::new(),
            falling_tile: None,
            fall_counter: 0.,
            fall_speed: 1,
        }
    }

    pub fn draw(&self) {
        draw_rectangle(
            self.draw_offset.0 as f32 - 1.,
            self.draw_offset.1 as f32 - 1.,
            BOARD_WIDTH as f32 + 2.,
            BOARD_HEIGHT as f32 + 2.,
            SKY,
        );
        for tile in self.ground_tiles.iter() {
            self.draw_single_tile(tile.0, tile.1);
        }
        if let Some(tetromino) = &self.falling_tile {
            for tile in &tetromino.tiles {
                self.draw_single_tile(
                    &(
                        &tetromino.grid_position.0 + tile.0 as u8,
                        &tetromino.grid_position.1 + tile.1 as u8,
                    ),
                    &tetromino.color,
                );
            }
        }
    }

    fn draw_single_tile(&self, pos: &(u8, u8), color: &Color) {
        const LINE_THICKNESS: f32 = 2.;
        draw_rectangle(
            (self.draw_offset.0 + pos.0 as u32 * TILE_SIZE as u32) as f32,
            (self.draw_offset.1 + pos.1 as u32 * TILE_SIZE as u32) as f32,
            TILE_SIZE as f32,
            TILE_SIZE as f32,
            *color,
        );
        draw_rectangle_lines(
            (self.draw_offset.0 + pos.0 as u32 * TILE_SIZE as u32) as f32,
            (self.draw_offset.1 + pos.1 as u32 * TILE_SIZE as u32) as f32,
            TILE_SIZE as f32,
            TILE_SIZE as f32,
            LINE_THICKNESS,
            BLACK,
        );
    }

    pub fn check_for_collision(&self) -> bool {
        if let Some(falling_tile) = &self.falling_tile {
            for tile in &falling_tile.tiles {
                if falling_tile.grid_position.0 + (tile.0 as u8) >= 10 {
                    return true;
                }
                if falling_tile.grid_position.1 + (tile.1 as u8) >= 20 {
                    return true;
                }
                if self.ground_tiles.contains_key(&(
                    &falling_tile.grid_position.0 + tile.0 as u8,
                    &falling_tile.grid_position.1 + tile.1 as u8,
                )) {
                    return true;
                }
            }
        }
        return false;
    }

    pub fn new_random_falling_tile(&mut self) {
        self.falling_tile = Some(Tetromino::random_tetromino());
        if self.check_for_collision() {
            panic!()
        }
    }

    pub fn auto_move_tile_down(&mut self) {
        self.fall_counter += get_frame_time();
        if self.fall_counter > 0.8 / self.fall_speed as f32 {
            self.fall_counter = 0.;
            if let Some(falling_tile) = &mut self.falling_tile {
                falling_tile.grid_position.1 += 1;
            }
            if self.check_for_collision() {
                if let Some(falling_tile) = &mut self.falling_tile {
                    falling_tile.grid_position.1 -= 1;
                    self.dissolve_falling_tile();
                }
            }
        }
    }

    pub fn move_tile_down(&mut self) {
        if let Some(falling_tile) = &mut self.falling_tile {
            falling_tile.grid_position.1 += 1;
        }
        if self.check_for_collision() {
            if let Some(falling_tile) = &mut self.falling_tile {
                falling_tile.grid_position.1 -= 1;
                self.dissolve_falling_tile();
            }
        }
    }

    pub fn try_move_tile_right(&mut self) {
        if let Some(falling_tile) = &mut self.falling_tile {
            falling_tile.grid_position.0 += 1;
        }
        if self.check_for_collision() {
            if let Some(falling_tile) = &mut self.falling_tile {
                falling_tile.grid_position.0 -= 1;
            }
        }
    }

    pub fn try_move_tile_left(&mut self) {
        if let Some(falling_tile) = &mut self.falling_tile {
            falling_tile.grid_position.0 -= 1;
        }
        if self.check_for_collision() {
            if let Some(falling_tile) = &mut self.falling_tile {
                falling_tile.grid_position.0 += 1;
            }
        }
    }

    pub fn try_rotate_tile_right(&mut self) {
        if let Some(falling_tile) = &mut self.falling_tile {
            falling_tile.rotate_right();
        }
        if self.check_for_collision() {
            if let Some(falling_tile) = &mut self.falling_tile {
                falling_tile.rotate_left();
            }
        }
    }
    pub fn try_rotate_tile_left(&mut self) {
        if let Some(falling_tile) = &mut self.falling_tile {
            falling_tile.rotate_left();
        }
        if self.check_for_collision() {
            if let Some(falling_tile) = &mut self.falling_tile {
                falling_tile.rotate_right();
            }
        }
    }

    pub fn dissolve_falling_tile(&mut self) {
        if let Some(falling_tile) = &self.falling_tile {
            for tile in falling_tile.tiles.iter() {
                self.ground_tiles.insert(
                    (
                        (tile.0 + falling_tile.grid_position.0 as i8) as u8,
                        (tile.1 + falling_tile.grid_position.1 as i8) as u8,
                    ),
                    falling_tile.color,
                );
            }
            self.new_random_falling_tile();
        }
    }

    pub fn clear_lines(&mut self){
        for x in 19..0{
            for y in 0..9{
                todo!();
            }
        }
    }
}
