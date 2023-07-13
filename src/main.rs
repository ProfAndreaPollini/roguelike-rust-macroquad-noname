mod actions;
mod engine;
mod player;
mod random_walk_builder;

use std::collections::HashMap;

use engine::core::Engine;

use engine::map::builder::{BasicMapBuilder, MapBuilder};

use engine::texture_manager::TextureManager;
use macroquad::prelude::*;
use random_walk_builder::RandomWalkBuilder;
fn window_conf() -> Conf {
    Conf {
        window_title: "Window Conf".to_owned(),
        window_width: 1600,
        window_height: 900,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut texture_manager =
        TextureManager::new("assets/urizen_onebit_tileset__v1d0.png", 12., 3., 1.).await;

    texture_manager.load_from_json("assets/config.json");

    //let map = Map::generate(100, 100);

    let map = MapBuilder::new(100, 100, HashMap::new())
        .add_step(&BasicMapBuilder::default())
        .add_step(&RandomWalkBuilder::default())
        .build();

    let mut engine = Engine::new(texture_manager, map);

    loop {
        clear_background(BLACK);

        engine.update();

        draw_text("IT WORKS1!", 20.0, 20.0, 30.0, DARKGRAY);
        // map.draw(&texture_manager);

        engine.render();
        engine.update_fov();

        next_frame().await
    }
}
