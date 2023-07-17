#![allow(dead_code)]
use crate::{
    actions::Action,
    engine::{texture_manager::TextureManager, viewport::Viewport},
    npc::NPC,
    player::Player,
};

use delegate::delegate;

use macroquad::{prelude::Vec2, prelude::WHITE, texture::draw_texture_ex};

pub trait Drawable {
    fn draw(&self, _texture_manager: &TextureManager, _viewport: &Viewport) {}
}

pub trait Updatable {
    fn update(&mut self);
    fn next_action(&self) -> Option<Action> {
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
    pub name: String,
    breed: T,
}

impl<T> EntityFeatures<T>
where
    T: Updatable + Drawable + Default,
{
    pub fn new(name: String) -> Self {
        Self {
            name,
            breed: T::default(),
        }
    }
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

    fn next_action(&self) -> Option<Action> {
        None
    }
    fn position(&self) -> Option<(i32, i32)> {
        None
    }

    fn move_by(&mut self, dx: i32, dy: i32) {
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

    pub fn update(&mut self) {
        match self {
            Entity::Player(player) => player.breed.update(),
            Entity::NPC(npc) => npc.breed.update(),
        }
    }

    pub fn next_action(&self) -> Option<Action> {
        match self {
            Entity::Player(player) => player.breed.next_action(),
            Entity::NPC(npc) => npc.breed.next_action(),
        }
    }

    pub fn draw(&self, texture_manager: &TextureManager, viewport: &Viewport) {
        match self {
            Entity::Player(player) => player.breed.draw(texture_manager, viewport),
            Entity::NPC(npc) => npc.breed.draw(texture_manager, viewport),
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
