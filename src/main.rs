use std::sync::Arc;

use macroquad::color;
use macroquad::math::Rect;
use macroquad::prelude::Vec2;
use macroquad::shapes;
use macroquad::texture::{draw_texture_ex, DrawTextureParams};
use macroquad::window;

mod asset;

use asset::Assets;

const GRID_WIDTH: i16 = 11;
const GRID_HEIGHT: i16 = 4;
const PADDING: f32 = 10.;

// Game mechanics
// - Player has 20 hp
// - Troop has 3 stats: attack, defense, hp
//   attack controls how much damages it will deal to the enemy
//   defense is replenished on every turn and controls how many attack damage
//     a troop can take before losing live
//   hp controls how many non-blocked damage left it can take
//     when hp goes to 0 the troop will be gone

#[macroquad::main("Snake")]
async fn main() {
    env_logger::init();

    let assets = Arc::new(asset::load_assets().await);

    loop {
        let screenw = window::screen_width();
        let screenh = window::screen_height();

        draw_board(screenw, screenh, assets.clone());

        draw_overlay(screenw, screenh);

        window::next_frame().await
    }
}

fn draw_board(screenw: f32, screenh: f32, textures: Arc<Assets>) {
    let gamew = screenw - 2. * PADDING;
    let gameh = screenh - 2. * PADDING;

    let sq_size_x = gamew / GRID_WIDTH as f32;
    let sq_size_y = gameh / GRID_HEIGHT as f32;
    let sq_size = sq_size_x.min(sq_size_y);
    let offset_x = (screenw - sq_size * GRID_WIDTH as f32) / 2.;
    let offset_y = (screenh - sq_size * GRID_HEIGHT as f32) / 2.;

    window::clear_background(color::LIGHTGRAY);

    // Draw player board
    shapes::draw_rectangle(
        offset_x + 2. * sq_size,
        offset_y,
        3. * sq_size,
        4. * sq_size,
        color::WHITE,
    );
    for row in 2..5 {
        for col in 0..4 {
            shapes::draw_rectangle_lines(
                offset_x + row as f32 * sq_size, 
                offset_y + col as f32 * sq_size, 
                sq_size, 
                sq_size, 
                1.0, 
                color::GRAY,
            );
        }
    }

    // Draw opponent board
    shapes::draw_rectangle(
        offset_x + 6. * sq_size,
        offset_y,
        3. * sq_size,
        4. * sq_size,
        color::WHITE,
    );
    for row in 6..9 {
        for col in 0..4 {
            shapes::draw_rectangle_lines(
                offset_x + row as f32 * sq_size, 
                offset_y + col as f32 * sq_size, 
                sq_size, 
                sq_size, 
                1.0, 
                color::GRAY,
            );
        }
    }

    // Try drawing texture
    let texture_params = DrawTextureParams {
        dest_size: Some(Vec2::new(sq_size, sq_size)),
        source: Some(Rect::new(50., 0., 420., 420.)),
        rotation: 0.0,
        flip_x: false,
        flip_y: false,
        pivot: None,
    };
    draw_texture_ex(
        textures.wraith1, 
        offset_x + 2. * sq_size, 
        offset_y, 
        color::WHITE,
        texture_params,
    );
}

fn draw_overlay(screenw: f32, screenh: f32) {
}

