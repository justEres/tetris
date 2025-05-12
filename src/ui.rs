use crate::colors::*;
use macroquad::text::{Font, TextParams,draw_text_ex, load_ttf_font_from_bytes};

pub struct Ui {
    font: Font,
}

const FONT_FILE: &[u8] = include_bytes!("../assets/font.ttf");

impl Ui {
    pub fn draw(&self) {
        draw_text_ex(
            "HELLO",
            20.0,
            20.0,
            TextParams {
                font: Some(&self.font),
                color: BLACK,
                ..Default::default()
            }
        );
    }
    pub fn new() -> Ui {
        Ui {
            font: load_ttf_font_from_bytes(FONT_FILE).expect("couldnt load font"),
        }
    }
}
