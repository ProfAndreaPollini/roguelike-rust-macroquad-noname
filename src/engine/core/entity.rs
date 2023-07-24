#![allow(dead_code)]

use crate::{
    actions::Action,
    engine::{map::Map, texture_manager::TextureManager, viewport::Viewport},
    npc::NPC,
    player::Player,
};

use delegate::delegate;

use macroquad::{prelude::Vec2, prelude::WHITE, texture::draw_texture_ex};

use super::{
    camera::Camera,
    direction::Direction,
    world::{EntityKey, World},
};

pub trait Drawable {
    fn draw(&self, _texture_manager: &TextureManager, _viewport: &Camera) {}
}

pub trait Updatable {
    fn update(&self, _map: &mut Map, _world: &World, _key: EntityKey) -> Vec<Action> {
        vec![]
    }

    fn next_action(&self, _map: &Map, _world: &World, _key: EntityKey) -> Option<Action> {
        None
    }
    fn position(&self) -> Option<(i32, i32)> {
        None
    }

    fn move_by(&mut self, _dx: i32, _dy: i32) {}
    fn move_to(&mut self, _x: i32, _y: i32) {}
}

/// Trait representing an entity in the game world.
#[derive(Debug, Clone)]
pub struct EntityFeatures<T: Updatable + Drawable> {
    id: Option<EntityKey>,
    pub name: String,
    pub breed: T,
}

impl<T> EntityFeatures<T>
where
    T: Updatable + Drawable + Default,
{
    pub fn new(name: String) -> Self {
        Self {
            name,
            breed: T::default(),
            id: None,
        }
    }

    pub fn render(&self, _texture_manager: &TextureManager, _viewport: &Viewport) {}
}

impl<T> EntityFeatures<T>
where
    T: Updatable + Drawable,
{
    // fn draw(&self, texture_manager: &TextureManager, viewport: &Viewport) {}
    // fn update(&mut self) {}

    fn is_player(&self) -> bool {
        false
    }

    fn next_action(&mut self) -> Option<Action> {
        None
    }
    pub fn position(&self) -> Option<(i32, i32)> {
        None
    }

    pub fn move_by(&mut self, dx: i32, dy: i32) {
        if let Some((x, y)) = self.position() {
            self.breed.move_by(x + dx, y + dy);
        }
    }

    pub fn move_to(&mut self, x: i32, y: i32) {
        if let Some((_x, _y)) = self.position() {
            self.breed.move_to(x, y);
        }
    }
}

#[derive(Debug, Clone)]
pub enum Entity {
    Player(EntityFeatures<Player>),
    NPC(EntityFeatures<NPC>),
}

impl Entity {
    pub fn id(&self) -> Option<EntityKey> {
        match self {
            Entity::Player(player) => player.id,
            Entity::NPC(npc) => npc.id,
        }
    }

    pub(crate) fn set_id(&mut self, id: EntityKey) {
        match self {
            Entity::Player(player) => player.id = Some(id),
            Entity::NPC(npc) => npc.id = Some(id),
        }
    }

    pub fn is_player(&self) -> bool {
        match self {
            Entity::Player(_) => true,
            Entity::NPC(_) => false,
        }
    }

    pub fn position(&self) -> Option<(i32, i32)> {
        match self {
            Entity::Player(player) => player.breed.position(),
            Entity::NPC(npc) => npc.breed.position(),
        }
    }

    pub fn update(&self, world: &World, map: &mut Map) -> Vec<Action> {
        match self {
            Entity::Player(features) => {
                if let Some(key) = features.id {
                    features.breed.update(map, world, key)
                } else {
                    vec![]
                }
            }
            Entity::NPC(features) => {
                if let Some(key) = features.id {
                    features.breed.update(map, world, key)
                } else {
                    vec![]
                }
            }
        }
    }

    pub fn next_action(&mut self, world: &mut World, map: &Map) -> Option<Action> {
        match self {
            Entity::Player(features) => {
                features.breed.next_action(map, world, features.id.unwrap())
            }
            Entity::NPC(features) => features.breed.next_action(map, world, features.id.unwrap()),
        };

        None
    }

    pub fn draw(&self, texture_manager: &TextureManager, camera: &Camera) {
        match self {
            Entity::Player(player) => player.breed.draw(texture_manager, camera),
            Entity::NPC(npc) => npc.breed.draw(texture_manager, camera),
        }
    }

    pub fn direction(&self) -> Direction {
        match self {
            Entity::Player(player) => player.breed.direction,
            Entity::NPC(npc) => npc.breed.direction,
        }
    }

    pub fn set_direction(&mut self, direction: Direction) {
        match self {
            Entity::Player(player) => player.breed.direction = direction,
            Entity::NPC(npc) => npc.breed.direction = direction,
        }
    }

    delegate! {
        to match self {
            Entity::Player(player) => player.breed,
            Entity::NPC(npc) => npc.breed,
        } {
            pub fn move_by(&mut self, dx: i32, dy: i32);
            pub fn move_to(&mut self, x: i32, y: i32);
        }
    }
}

pub fn draw_sprite(
    texture: &macroquad::texture::Texture2D,
    x: i32,
    y: i32,
    viewport: &Viewport,
    cell_output_size: Vec2,
    sprite_rect: macroquad::prelude::Rect,
) {
    let center = viewport.center();

    draw_texture_ex(
        *texture,
        (x as f32 + center.x) * cell_output_size.x,
        (y as f32 + center.y) * cell_output_size.y,
        WHITE,
        macroquad::prelude::DrawTextureParams {
            source: Some(sprite_rect),
            dest_size: Some(cell_output_size),
            ..Default::default()
        },
    );
}
