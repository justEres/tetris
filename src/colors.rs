use macroquad::color::Color;
use rand::Rng;

// Main palette colors

pub const BLACK: Color = Color::from_hex(0x1a1c2c);
pub const PURPLE: Color = Color::from_hex(0x5d275d);
pub const RED: Color = Color::from_hex(0xb13e53);
pub const ORANGE: Color = Color::from_hex(0xef7d57);
pub const YELLOW: Color = Color::from_hex(0xffcd75);
pub const LIME: Color = Color::from_hex(0xa7f070);
pub const GREEN: Color = Color::from_hex(0x38b764);
pub const TEAL: Color = Color::from_hex(0x257179);
pub const NAVY: Color = Color::from_hex(0x29366f);
pub const BLUE: Color = Color::from_hex(0x3b5dc9);
pub const SKY: Color = Color::from_hex(0x41a6f6);
pub const CYAN: Color = Color::from_hex(0x73eff7);
pub const WHITE: Color = Color::from_hex(0xf4f4f4);
pub const LIGHT_GRAY: Color = Color::from_hex(0x94b0c2);
pub const GRAY: Color = Color::from_hex(0x566c86);
pub const DARK_GRAY: Color = Color::from_hex(0x333c57);



const POSSIBLE_COLORS: [Color; 9] = [PURPLE,RED,ORANGE,YELLOW,LIME,GREEN,TEAL,NAVY,BLUE];
pub fn random_color() -> Color{
    let mut rng = rand::rng();
    POSSIBLE_COLORS[rng.random_range(0..POSSIBLE_COLORS.len())]
}