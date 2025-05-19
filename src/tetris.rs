use crate::keyboard::controls;
use crate::ui::window_conf;
use crate::{board::Board, colors::*, ui::Ui};
use macroquad::material::{load_material, MaterialParams};
use macroquad::prelude::{*, ShaderSource};
use macroquad::color::WHITE;

#[macroquad::main(window_conf)]
pub async fn main() {
    let ui = Ui::new();
    let mut board = Board::new();
    board.new_random_falling_tile();

    // Load the wobble shader as a Material
    let wobble_material = load_material(
        ShaderSource::Glsl {
            vertex: include_str!("../assets/wobble.vert"),
            fragment: include_str!("../assets/wobble.frag"),
        },
        MaterialParams::default(),
    ).unwrap();

    // Create a render target for postprocessing
    let mut win_w = screen_width() as u32;
    let mut win_h = screen_height() as u32;
    let mut screen_rt = render_target(win_w, win_h);
    screen_rt.texture.set_filter(FilterMode::Nearest);

    loop {
        // Recreate render target if window size changed
        let cur_w = screen_width() as u32;
        let cur_h = screen_height() as u32;
        if cur_w != win_w || cur_h != win_h {
            win_w = cur_w;
            win_h = cur_h;
            screen_rt = render_target(win_w, win_h);
            screen_rt.texture.set_filter(FilterMode::Nearest);
        }

        set_camera(&Camera2D {
            render_target: Some(screen_rt.clone()),
            ..Default::default()
        });
        clear_background(DARK_GRAY);
        ui.draw(board.score);
        board.auto_move_tile_down();
        controls(&mut board);
        board.draw();
        board.clear_lines_if_possible();

        // Draw the render target to the screen with the wobble material
        set_camera(&Camera2D::default());
        // No need to set 'time' uniform, Macroquad provides _Time automatically
        gl_use_material(&wobble_material);
        draw_texture_ex(
            &screen_rt.texture,
            0.0,
            0.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(win_w as f32, win_h as f32)),
                ..Default::default()
            },
        );
        gl_use_default_material();
        next_frame().await;
    }
}
