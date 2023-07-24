mod actions;
mod engine;
mod npc;
mod player;
mod random_walk_builder;
mod room_builder;
mod scenes;
pub mod ui;

use std::cell::RefCell;
use std::collections::HashMap;

use std::rc::Rc;

use engine::core::Engine;

use engine::map::builder::{BasicMapBuilder, MapBuilder};

use engine::texture_manager::TextureManager;
use macroquad::prelude::*;

use macroquad::ui::{hash, root_ui, widgets};
use random_walk_builder::RandomWalkBuilder;
use room_builder::RoomBuilder;
use scenes::events::SceneEvent;

use scenes::{Scene, UpdatableScene};

use crate::scenes::events::MouseEvent;

fn window_conf() -> Conf {
    Conf {
        window_title: "Window Conf".to_owned(),
        window_width: 1600,
        window_height: 900,
        ..Default::default()
    }
}

// lazy_static! {
//     static ref WORLD: RwLock<World> = RwLock::new(World::new());
// }

#[macroquad::main(window_conf)]
async fn main() {
    let mut texture_manager =
        TextureManager::new("assets/urizen_onebit_tileset__v1d0.png", 12., 3., 1.).await;

    texture_manager.load_from_json("assets/config.json");

    let map = MapBuilder::new(100, 100, HashMap::new())
        .add_step(&BasicMapBuilder::default())
        .add_step(&RandomWalkBuilder::default())
        .build();

    let engine = Engine::new(texture_manager, map);

    let mut texture_manager =
        TextureManager::new("assets/urizen_onebit_tileset__v1d0.png", 12., 3., 1.).await;

    texture_manager.load_from_json("assets/config.json");

    let font = load_ttf_font("assets/fonts/dealerplate_california.otf")
        .await
        .unwrap();

    let mut scenes = HashMap::<Scene, Rc<RefCell<dyn UpdatableScene>>>::new();

    let intro_scene = scenes::intro_scene::IntroScene::new();
    scenes.insert(Scene::Intro, Rc::new(RefCell::new(intro_scene)));

    let game_scene = scenes::game_scene::GameScene::new();
    scenes.insert(Scene::Game, Rc::new(RefCell::new(game_scene)));

    // let _ret = scene_sm.init(Scene::Intro);

    // let x = scenes.get(&Scene::Intro).unwrap().borrow_mut();
    let mut current_scene = scenes.get(&Scene::Intro).unwrap().clone();
    // let mut current_scene = current_scene_ref.borrow_mut();

    let context = Rc::new(scenes::SceneContext {
        texture_manager: Some(Rc::new(texture_manager)),
        font: Some(Rc::new(font)),
    });

    current_scene.borrow_mut().setup(context.clone());
    let mut commands = vec![];

    loop {
        clear_background(BLACK);

        current_scene.borrow_mut().update();
        current_scene.borrow_mut().draw();
        current_scene.borrow_mut().draw_ui();

        widgets::Window::new(hash!(), vec2(400., 200.), vec2(320., 400.))
            .label("Shop")
            .titlebar(true)
            // .movable(false)
            .ui(&mut root_ui(), |ui| {
                ui.label(Vec2::new(50., 50.), "Hello World!");
            });

        let pos = engine.entity_at(0).position().unwrap();

        // egui_macroquad::ui(|egui_ctx: &egui::Context| {
        //     egui::Window::new("egui ❤ macroquad").show(egui_ctx, |ui| {
        //         //display fps
        //         ui.label(format!("FPS: {}", get_fps()));

        //         ui.label("Test");
        //         ui.label("ViewPort: ");
        //         ui.label(format!("{:?}", engine.viewport()));
        //         let mut binding = engine.viewport_m();
        //         let offset = binding.offset_mut();

        //         ui.add(egui::DragValue::new(&mut offset.x).speed(0.1));
        //         ui.add(egui::DragValue::new(&mut offset.y).speed(0.1));

        //         let mut binding2 = binding.clone();
        //         let rect = binding2.rect_mut();
        //         ui.label("Rect: ");
        //         ui.add(egui::DragValue::new(&mut rect.x).speed(0.1));
        //         ui.add(egui::DragValue::new(&mut rect.y).speed(0.1));

        //         ui.add(egui::DragValue::new(&mut rect.w).speed(0.1));
        //         ui.add(egui::DragValue::new(&mut rect.h).speed(0.1));

        //         ui.label("Player: ");
        //         ui.label(format!("{:?}", pos));
        //     });
        // });

        //get all key pressed this frame
        let keys_pressed_this_frame = get_last_key_pressed();

        if keys_pressed_this_frame.is_some() {
            let event = SceneEvent::KeyPressed(keys_pressed_this_frame.unwrap());
            println!("key pressed {:?}", event);

            if let Some(cmd) = current_scene.borrow_mut().process_input(event) {
                commands.push(cmd);
            }
        }

        // process mouse events
        let mut mouse_events: Vec<MouseEvent> = vec![];

        for btn in [MouseButton::Left, MouseButton::Right] {
            if is_mouse_button_pressed(btn) {
                mouse_events.push(MouseEvent::Pressed(btn));
            }

            if is_mouse_button_released(btn) {
                mouse_events.push(MouseEvent::Released(btn));
            }
        }

        // process mouse move
        if !mouse_events.is_empty() {
            let event = SceneEvent::Mouse(mouse_events);
            println!("firing mouse event {:?}", event);

            if let Some(cmd) = current_scene.borrow_mut().process_input(event) {
                commands.push(cmd);
            }
        }

        egui_macroquad::draw();

        // process commands
        if !commands.is_empty() {
            let cmd = commands.pop().unwrap();
            match cmd {
                scenes::events::SceneCommands::ChangeScene(scene) => {
                    let new_scene = scenes.get(&scene).unwrap().clone();
                    current_scene = new_scene;
                    current_scene.borrow_mut().setup(context.clone());
                }
                scenes::events::SceneCommands::Exit => break,
            }
        }

        next_frame().await
    }
}
