mod actions;
mod engine;
mod map;
mod player;

use actions::ActionHandler;
use engine::texture_manager::TextureManager;
use macroquad::prelude::*;
use map::{Map, Tile};
use player::Player;

const TILE_SIZE: f32 = 32.0;

#[macroquad::main("BasicShapes")]
async fn main() {
    let texture_manager =
        TextureManager::new("assets/urizen_onebit_tileset__v1d0.png", 12., 3., 1.).await;

    let mut player = Player::new(&texture_manager);
    let mut action_manager = ActionHandler::new();

    let mut map = Map::new(1000, 1000);
    map.add_tile(10, 10, Tile::new());

    if let Some(tile) = map.tile_at(11, 10) {
        println!("tile: {:?}", tile);
    } else {
        println!("no tile");
    }

    println!("map: {:?}", map);

    loop {
        clear_background(RED);

        draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
        draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);
        draw_circle(screen_width() - 30.0, screen_height() - 30.0, 15.0, YELLOW);

        draw_text("IT WORKS1!", 20.0, 20.0, 30.0, DARKGRAY);

        player.handle_input(&mut action_manager);
        action_manager.handle_actions(&mut player);
        player.draw();
        next_frame().await
    }
}
