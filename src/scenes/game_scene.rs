use std::{collections::HashMap, rc::Rc};

use macroquad::{
    prelude::{Color, IVec2, Rect, Vec2, BLUE, GREEN, WHITE},
    shapes::draw_rectangle,
    text::{draw_text, Font},
    time::get_fps,
    window::{screen_height, screen_width},
};
use zorder::{coord_of, index_of};

use crate::{
    actions::{Action, ActionResult},
    engine::{
        core::{
            camera::{self, Camera},
            entity::{Entity, EntityFeatures},
            world::{self, World},
        },
        fov::compute_fov,
        map::{
            builder::{BasicMapBuilder, MapBuilder},
            cell::Cell,
            renderer::MapRenderer,
            Map,
        },
        texture_manager::TextureManager,
        viewport::{self, Viewport},
    },
    player,
    random_walk_builder::RandomWalkBuilder,
    room_builder::RoomBuilder,
    ui::buttons::Button,
};

use super::{SceneContext, UpdatableScene};

// pub struct GameSceneContext<'a> {
//     pub world: &'a mut World,
// }

pub struct GameScene {
    log: Vec<String>,
    texture_manager: Option<Rc<TextureManager>>,
    map: Option<Map>,
    // viewport: Option<Viewport>,
    map_renderer: Option<MapRenderer>,
    pub camera: Option<Camera>,
    world: World,
    button: Option<Button>,
    font: Option<Font>,
}

impl GameScene {
    pub fn new() -> Self {
        Self {
            log: vec![],
            texture_manager: None,
            map: None,
            // viewport: None,
            // entities: Entities::new(),
            world: World::new(),
            map_renderer: None,
            camera: None,
            button: None,
            font: None,
        }
    }
}

pub fn update_fov(map: &mut Map, world: &mut World) {
    let fov_distance: i32 = 5;

    map.set_all_tiles_visibility(false);

    // let binding = world;
    let entities = world.iter().collect::<Vec<&Entity>>();

    let position = entities.get(0).unwrap().position().unwrap();

    compute_fov(map, IVec2::new(position.0, position.1), fov_distance);
}

impl UpdatableScene for GameScene {
    fn setup(&mut self, context: Rc<SceneContext>) {
        // info!("GameScene setup");

        let map = MapBuilder::new(100, 100, HashMap::new())
            // .add_step(&BasicMapBuilder::default())
            .add_step(&RandomWalkBuilder::default())
            .add_step(&RoomBuilder::default())
            .build();

        let x = context.texture_manager.as_ref().unwrap().clone();

        self.texture_manager = Some(x);

        self.map = Some(map);
        // let viewport = Viewport::new(0.0, 0.0, 40.0, 30.0, Vec2::new(17.5, 18.7));
        // self.viewport = Some(viewport);

        self.map_renderer = Some(MapRenderer::default());
        self.camera = Some(Camera::new(
            Vec2 { x: 0., y: 0. },
            2.,
            Rect::new(100., 100., 800., 600.),
        ));

        let mut player = Entity::Player(EntityFeatures::new("player".to_string()));
        player.move_to(10, 10);

        self.world.create_entity(player);

        let mut npc01 = Entity::NPC(EntityFeatures::new("npc01".to_string()));
        npc01.move_to(20, 20);
        self.world.create_entity(npc01);

        let font = *context.font.as_ref().unwrap().clone();

        let mut button = Button::new(
            "Button",
            Vec2::new(100., 100.),
            font,
            72,
            Vec2::new(10., 10.),
        );
        button.normal_color = WHITE;
        button.normal_bg_color = GREEN;
        button.hovered_color = Some(WHITE);
        button.hovered_bg_color = Some(BLUE);
        self.button = Some(button);
    }

    fn update(&mut self) {
        // println!("GameScene update");
        let map = self.map.as_mut().unwrap();

        let entities = self.world.iter().collect::<Vec<&Entity>>();
        let mut actions: Vec<Action> = Vec::new();

        for entity in entities {
            // println!("update entity : {:?}", entity);

            let a = entity.update(&self.world, map);

            actions.extend(a);
        }

        while let Some(action) = actions.pop() {
            let action_reponse = action.perform(map, &mut self.world);

            match action_reponse {
                ActionResult::Succeeded => {}
                ActionResult::Failure => {}
                ActionResult::AlternativeAction(action) => {
                    actions.push(action);
                }
            }
        }

        let player = self.world.player_mut().unwrap();
        let mut camera = self.camera.as_mut().unwrap();

        // let viewport_size = self.viewport.as_ref().unwrap().get().size() * 0.5;
        let viewport_center = camera.viewport_center();
        let player_pos = player.position().unwrap();
        let tile_size = self.texture_manager.as_ref().unwrap().cell_size;
        let p0 = Vec2::new(player_pos.0 as f32, player_pos.1 as f32);
        let p1 = p0 * tile_size - viewport_center;
        println!(
            "camera_pos = {:?} tile_size = {:?} center = {:?}",
            camera.position, tile_size, viewport_center
        );
        println!("p0 = {:?}, p1 = {:?}", p0, p1);
        // self.camera.as_mut().unwrap().position = p0 * tile_size - viewport_size;
        self.camera.as_mut().unwrap().position = p0 * tile_size - viewport_center;
        // println!("camera_pos = {:?}", camera_pos);
        update_fov(self.map.as_mut().unwrap(), &mut self.world);
        self.button.as_mut().unwrap().update();

        if self.button.as_ref().unwrap().clicked() {
            println!("Button clicked");
        }
    }

    fn draw(&self) {
        // debug!("GameScene draw");
        draw_text(
            "GAME!!!!!",
            screen_width() / 2.,
            screen_height() / 2.,
            120.,
            WHITE,
        );

        let mut y = 50.;

        for log in self.log.iter() {
            draw_text(log, screen_width() - 200., y, 20., WHITE);
            y += 20.;
        }

        let texture_manager = self.texture_manager.as_ref().unwrap();
        // let viewport = self.viewport.as_ref().unwrap();
        let map = self.map.as_ref().unwrap();
        let world = &self.world;

        // /map.draw(texture_manager, viewport);
        self.map_renderer.as_ref().unwrap().render(
            map,
            self.camera.as_ref().unwrap(),
            texture_manager,
        );

        self.world.iter().for_each(|e| {
            // print!("draw entity : {:?} - ", e);
            e.draw(texture_manager, self.camera.as_ref().unwrap());
        });

        let player_pos = world.player().unwrap().position().unwrap();

        let pos = (player_pos.0 as u16, player_pos.1 as u16);
        let c = Cell::new(pos.0, pos.1);

        let path = c.line_to(&Cell::new(10, 10));

        for p in path {
            let (x, y) = coord_of(p);
            // print!("{} {} -> ", x, y);
            self.map_renderer.as_ref().unwrap().highlight_tile(
                Vec2::new(x as f32, y as f32),
                texture_manager,
                self.camera.as_ref().unwrap(),
                Color::new(0., 1., 0., 0.5),
            );
        }

        // if let Some(camera) = self.camera.as_ref() {
        //     // let (x, y) = (camera.position.x as u16, camera.position.y as u16);
        //     // print!("{} {} -> ", x, y);
        //     self.map_renderer.as_ref().unwrap().highlight_tile(
        //         // Vec2::new(x as f32, y as f32),
        //         texture_manager,
        //         self.camera.as_ref().unwrap(),
        //         Color::new(1., 0., 0., 0.5),
        //     );
        // }

        // if let Some(ref button) = self.button {
        //     button.draw();
        // }
    }

    fn draw_ui(&mut self) {
        let camera = self.camera.as_mut().unwrap();

        egui_macroquad::ui(|egui_ctx: &egui::Context| {
            egui::Window::new("egui ❤ macroquad").show(egui_ctx, |ui| {
                //display fps
                ui.label(format!("FPS: {}", get_fps()));

                ui.label("Test");
                ui.label("ViewPort: ");

                // ui.label(format!("{:?}", camera.viewport));

                ui.label("Camera Viewport Pos: ");
                ui.add(egui::DragValue::new(&mut camera.viewport.x).speed(10.));
                ui.add(egui::DragValue::new(&mut camera.viewport.y).speed(10.));

                ui.label("Camera Position: ");
                ui.add(egui::DragValue::new(&mut camera.position.x).speed(10.));
                ui.add(egui::DragValue::new(&mut camera.position.y).speed(10.));
            });
        });
    }
}
