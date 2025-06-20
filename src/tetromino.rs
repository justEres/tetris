use std::collections::HashSet;

use macroquad::color::Color;
use rand::Rng;

use crate::colors::random_color;

#[derive(Clone, Copy)]
pub enum TetrominoType {
    I,
    O,
    T,
    J,
    L,
    S,
    Z,
}
const VARIANTS: [TetrominoType; 7] = [
    TetrominoType::I,
    TetrominoType::O,
    TetrominoType::T,
    TetrominoType::J,
    TetrominoType::L,
    TetrominoType::S,
    TetrominoType::Z,
];

#[derive(Clone)]
pub struct Tetromino {
    pub tiles: HashSet<(i8, i8)>,
    pub grid_position: (u8, u8),
    pub color: Color,
}

const SPAWN_POSITION: (u8, u8) = (4, 1);

impl Tetromino {
    pub fn random_tetromino() -> (TetrominoType, Color) {
        let mut rng = rand::rng();
        let varinant = &VARIANTS[rng.random_range(0..VARIANTS.len())];
        return (*varinant, random_color());
    }

    pub fn new_tetromino(color: Color, tetromino_type: &TetrominoType) -> Tetromino {
        let tiles: HashSet<(i8, i8)> = match tetromino_type {
            TetrominoType::I => HashSet::from([(0, 0), (1, 0), (0, 1), (1, 1)]),
            TetrominoType::O => HashSet::from([(-1, 0), (0, 0), (1, 0), (2, 0)]),
            TetrominoType::T => HashSet::from([(-1, 0), (0, 0), (1, 0), (0, -1)]),
            TetrominoType::J => HashSet::from([(0, -1), (0, 0), (0, 1), (-1, 1)]),
            TetrominoType::L => HashSet::from([(0, -1), (0, 0), (0, 1), (1, 1)]),
            TetrominoType::S => HashSet::from([(-1, 1), (0, 1), (0, 0), (1, 0)]),
            TetrominoType::Z => HashSet::from([(-1, 0), (0, 0), (0, 1), (1, 1)]),
        };
        Tetromino {
            tiles,
            grid_position: SPAWN_POSITION,
            color,
        }
    }

    pub fn rotate_right(&mut self) {
        let mut new_tiles: HashSet<(i8, i8)> = HashSet::new();
        for tile in self.tiles.iter() {
            new_tiles.insert((tile.1, -tile.0));
        }
        self.tiles = new_tiles;
    }

    pub fn rotate_left(&mut self) {
        let mut new_tiles: HashSet<(i8, i8)> = HashSet::new();
        for tile in self.tiles.iter() {
            new_tiles.insert((-tile.1, tile.0));
        }
        self.tiles = new_tiles;
    }
}
