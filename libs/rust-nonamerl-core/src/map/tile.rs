use std::fmt::Debug;

use macroquad::{prelude::Color, texture::Texture2D};

use crate::world::ItemKey;

#[derive(Debug, Clone, PartialEq)]
pub enum TileSpriteInfo {
    SpriteSheet(&'static str),
    SingleSprite(Texture2D),
    Fill(Color),
    None,
}

pub trait Tile:
    'static + Debug + Clone + Visible + Visited + FovOccluder + Walkable + ItemContainer
{
    fn sprite_info(&self) -> TileSpriteInfo {
        TileSpriteInfo::None
    }
}

#[repr(transparent)]
#[derive(PartialEq)]
pub struct VisibilityOcclusion(f32);

impl VisibilityOcclusion {
    pub fn new(v: f32) -> Option<Self> {
        if (0.0..=1.0).contains(&v) {
            Some(Self(v))
        } else {
            None
        }
    }

    pub unsafe fn new_unchecked(v: f32) -> Self {
        Self(v)
    }
}

impl From<VisibilityOcclusion> for f32 {
    fn from(v: VisibilityOcclusion) -> Self {
        v.0
    }
}

pub trait FovOccluder {
    const BLOCKED: VisibilityOcclusion = VisibilityOcclusion(0.);
    const VISIBLE: VisibilityOcclusion = VisibilityOcclusion(1.);
    fn block_visibility(&self) -> VisibilityOcclusion {
        Self::VISIBLE
    }
}

pub trait Visible {
    fn is_visible(&self) -> bool {
        true
    }
    fn set_visible(&mut self, visible: bool) {}
}

pub trait Visited {
    fn is_visited(&self) -> bool {
        false
    }
    fn set_visited(&mut self, visited: bool) {}
}

pub trait Walkable {
    fn is_walkable(&self) -> bool {
        true
    }
}

pub trait ItemContainer {
    fn items(&self) -> &[ItemKey] {
        &[]
    }
    fn add_item(&mut self, _item: ItemKey) {}
    fn remove_item(&mut self, _item: ItemKey) {}
}
