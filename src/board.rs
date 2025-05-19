use std::collections::HashMap;

use macroquad::{
    color::Color,
    input::is_key_down,
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
    pub fall_fast: bool,
    pub score: u32,
}

impl Board {
    pub fn new() -> Board {
        let draw_offset = (TILE_SIZE as u32 * 2, TILE_SIZE as u32 * 2);
        Board {
            draw_offset,
            ground_tiles: HashMap::new(),
            falling_tile: None,
            fall_counter: 0.,
            fall_fast: false,
            score: 0,
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
        if self.fall_counter > if self.fall_fast { 0.08 } else { 0.6 } {
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

    fn clear_line(&mut self, line: u8) {
        let mut new_ground_tiles: HashMap<(u8, u8), Color> = HashMap::new();
        for y in 0..line {
            for x in 0..10 {
                if let Some(color) = self.ground_tiles.get(&(x, y)) {
                    new_ground_tiles.insert((x, y + 1), *color);
                }
            }
        }
        for y in line + 1..20 {
            for x in 0..10 {
                if let Some(color) = self.ground_tiles.get(&(x, y)) {
                    new_ground_tiles.insert((x, y), *color);
                }
            }
        }
        self.ground_tiles = new_ground_tiles;
        self.score += 10;
    }

    fn get_full_lines(&mut self) -> Vec<u8> {
        if is_key_down(macroquad::input::KeyCode::Space) {
            dbg!(self.ground_tiles.iter());
        }
        let mut full_lines = Vec::new();
        'line: for y in 0..20 {
            for x in 0..10 {
                if !self.ground_tiles.contains_key(&(x, y)) {
                    continue 'line;
                }
            }
            full_lines.push(y);
        }
        return full_lines;
    }

    pub fn clear_lines_if_possible(&mut self) {
        let lines = self.get_full_lines();
        for line in lines {
            self.clear_line(line);
        }
    }
}
