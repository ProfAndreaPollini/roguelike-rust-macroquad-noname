use std::{collections::HashMap, rc::Rc};

use macroquad::{
    prelude::{info, IVec2, Vec2, WHITE},
    text::draw_text,
    window::{screen_height, screen_width},
};

use crate::{
    actions::{Action, ActionResult},
    engine::{
        core::{
            entity::{Entity, EntityFeatures},
            world::World,
        },
        fov::compute_fov,
        map::{
            builder::{BasicMapBuilder, MapBuilder},
            Map,
        },
        texture_manager::TextureManager,
        viewport::Viewport,
    },
    random_walk_builder::RandomWalkBuilder,
};

use super::{SceneContext, UpdatableScene};

// pub struct GameSceneContext<'a> {
//     pub world: &'a mut World,
// }

pub struct GameScene {
    log: Vec<String>,
    texture_manager: Option<Rc<TextureManager>>,
    map: Option<Map>,
    viewport: Option<Viewport>,

    world: World,
}

impl GameScene {
    pub fn new() -> Self {
        Self {
            log: vec![],
            texture_manager: None,
            map: None,
            viewport: None,
            // entities: Entities::new(),
            world: World::new(),
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
            .add_step(&BasicMapBuilder::default())
            .add_step(&RandomWalkBuilder::default())
            .build();

        let x = context.texture_manager.as_ref().unwrap().clone();

        self.texture_manager = Some(x);

        self.map = Some(map);
        let viewport = Viewport::new(0.0, 0.0, 40.0, 30.0, Vec2::new(17.5, 18.7));
        self.viewport = Some(viewport);

        let mut player = Entity::Player(EntityFeatures::new("player".to_string()));
        player.move_to(15, 15);

        self.world.create_entity(player);

        let mut npc01 = Entity::NPC(EntityFeatures::new("npc01".to_string()));
        npc01.move_to(20, 20);
        self.world.create_entity(npc01);
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

        update_fov(self.map.as_mut().unwrap(), &mut self.world);
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
        let viewport = self.viewport.as_ref().unwrap();
        let map = self.map.as_ref().unwrap();

        map.draw(texture_manager, viewport);

        self.world.iter().for_each(|e| {
            e.draw(texture_manager, viewport);
        });
    }
}
