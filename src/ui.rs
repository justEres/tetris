use crate::{
    board::Board,
    colors::*,
    tetris_constants::{self, BOARD_HEIGHT, BOARD_WIDTH, TILE_SIZE},
};
use macroquad::{
    shapes::{draw_circle, draw_rectangle},
    text::{Font, TextParams, draw_text_ex, load_ttf_font_from_bytes},
    window::Conf,
};

pub struct Ui {
    font: Font,
}

const FONT_FILE: &[u8] = include_bytes!("../assets/font.ttf");
const INNER_OFFSET: f32 = TILE_SIZE as f32;
const BOARD_POSITION: (f32, f32) = (TILE_SIZE as f32, TILE_SIZE as f32);
const NEXT_TILE_OFFSET: (f32, f32) = (TILE_SIZE as f32 * 17., TILE_SIZE as f32 * 10.);

impl Ui {
    pub fn draw(&self, score: u32) {
        draw_rounded_rect(
            BOARD_POSITION.0,
            BOARD_POSITION.1,
            tetris_constants::BOARD_WIDTH as f32 + 2. * INNER_OFFSET,
            tetris_constants::BOARD_HEIGHT as f32 + 2. * INNER_OFFSET,
            10.,
            BLUE,
        );
        draw_text_ex(
            &format!("SCORE: {}", score),
            (TILE_SIZE * 15) as f32,
            (TILE_SIZE * 2) as f32,
            TextParams {
                font: Some(&self.font),
                color: WHITE,
                ..Default::default()
            },
        );
        //draw_fps();
        draw_text_ex(
            "NEXT TETROMINO:",
            NEXT_TILE_OFFSET.0 - 2. * TILE_SIZE as f32,
            NEXT_TILE_OFFSET.1 - 3. * TILE_SIZE as f32,
            TextParams {
                font: Some(&self.font),
                color: WHITE,
                ..Default::default()
            },
        );
    }
    pub fn new() -> Ui {
        Ui {
            font: load_ttf_font_from_bytes(FONT_FILE).expect("couldnt load font"),
        }
    }
    pub fn draw_next_tile(board: &Board) {
        draw_rounded_rect(
            NEXT_TILE_OFFSET.0 - 2. * TILE_SIZE as f32,
            NEXT_TILE_OFFSET.1 - 2. * TILE_SIZE as f32,
            6. * TILE_SIZE as f32,
            5. * TILE_SIZE as f32,
            10.,
            BLUE,
        );
        board.draw_next_tile(&NEXT_TILE_OFFSET);
    }
}

pub fn window_conf() -> Conf {
    Conf {
        window_width: BOARD_WIDTH as i32 + 10 * TILE_SIZE as i32 + 2 * INNER_OFFSET as i32,
        window_height: BOARD_HEIGHT as i32 + 2 * TILE_SIZE as i32 + 2 * INNER_OFFSET as i32,
        window_resizable: false,
        ..Default::default()
    }
}

fn draw_rounded_rect(x: f32, y: f32, w: f32, h: f32, radius: f32, color: macroquad::color::Color) {
    // Draw the central rectangle
    draw_rectangle(x + radius, y, w - 2.0 * radius, h, color);
    draw_rectangle(x, y + radius, w, h - 2.0 * radius, color);

    // Draw the four corner circles
    draw_circle(x + radius, y + radius, radius, color); // Top-left
    draw_circle(x + w - radius, y + radius, radius, color); // Top-right
    draw_circle(x + radius, y + h - radius, radius, color); // Bottom-left
    draw_circle(x + w - radius, y + h - radius, radius, color); // Bottom-right
}
