#![allow(dead_code)]
use std::{collections::HashMap, hash::Hash};

use macroquad::{prelude::Rect, texture::Texture2D};

#[derive(Debug, Clone)]
pub struct AddSpriteOptions {
    pub gap: (u32, u32),
    pub size: (u32, u32),
}

#[derive(Debug, Copy, Clone)]
pub struct Sprite {
    pub pos: Rect,
    pub spritesheet: u8,
}

#[derive(Debug, Clone)]
pub struct SpriteSheet {
    pub textures: Vec<Texture2D>,
    pub sprites: HashMap<String, Sprite>,
}

impl SpriteSheet {
    pub fn new(texture: Texture2D) -> Self {
        Self {
            textures: vec![texture],
            sprites: HashMap::new(),
        }
    }

    pub fn add_texture(&mut self, texture: Texture2D) -> usize {
        self.textures.push(texture);
        self.textures.len() - 1
    }

    pub fn add_sprite(
        &mut self,
        name: &str,
        sprite_rect: Rect,
        add_sprite_options: Option<AddSpriteOptions>,
    ) {
        let mut pos = sprite_rect;
        if let Some(add_sprite_optione) = add_sprite_options {
            pos.x = sprite_rect.x * (add_sprite_optione.size.0 + add_sprite_optione.gap.0) as f32;
            pos.y = sprite_rect.y * (add_sprite_optione.size.1 + add_sprite_optione.gap.1) as f32;
            pos.w = add_sprite_optione.size.0 as f32;
            pos.h = add_sprite_optione.size.1 as f32;
        }

        self.sprites.insert(
            name.to_owned(),
            Sprite {
                pos,
                spritesheet: 0,
            },
        );
    }

    pub fn get_sprite(&self, name: &str) -> (&Rect, &Texture2D) {
        let sprite = self.sprites.get(name).unwrap();
        (&sprite.pos, &self.textures[sprite.spritesheet as usize])
    }
}
