mod actions;
mod engine;
mod npc;
mod player;
mod random_walk_builder;
mod scenes;

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use engine::core::Engine;

use engine::map::builder::{BasicMapBuilder, MapBuilder};

use engine::texture_manager::TextureManager;
use macroquad::prelude::*;
use random_walk_builder::RandomWalkBuilder;
use scenes::events::SceneEvent;
use scenes::fsm::GlobalStateTransitionHandler;
use scenes::{Scene, SceneContext};

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

    let engine = Engine::new(texture_manager, map);

    let mut scene_sm = nefsm::sync::StateMachine::<Scene, SceneContext, SceneEvent>::new(
        SceneContext {
            game: engine.clone(),
        },
        Some(Box::new(GlobalStateTransitionHandler {})),
    );

    let _ret = scene_sm.init(Scene::Intro);

    loop {
        clear_background(BLACK);

        engine.update();

        // map.draw(&texture_manager);

        engine.render();
        engine.update_fov();

        egui_macroquad::ui(|egui_ctx| {
            egui::Window::new("egui ❤ macroquad").show(egui_ctx, |ui| {
                //display fps
                ui.label(format!("FPS: {}", get_fps()));

                ui.label("Test");
            });
        });

        //get all key pressed this frame
        let keys_pressed_this_frame = get_last_key_pressed();

        println!(" key pressed{:?}", keys_pressed_this_frame);
        if keys_pressed_this_frame.is_some() {
            let event = SceneEvent::KeyPressed(keys_pressed_this_frame.unwrap());
            println!("firing event {:?}", event);
            let _ = scene_sm.process_event(&event);
        }

        let _ = scene_sm.process_event(&SceneEvent::Update);
        let _ = scene_sm.process_event(&SceneEvent::Draw);

        egui_macroquad::draw();

        next_frame().await
    }
}
