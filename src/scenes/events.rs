#![allow(dead_code)]
use macroquad::prelude::KeyCode;

#[derive(Debug)]
pub enum SceneEvent {
    PlayGame,
    KeyPressed(KeyCode),
    Update,
    Draw,
    EndGame,
}

impl ToString for SceneEvent {
    fn to_string(&self) -> String {
        stringify!(self).to_owned()
    }
}
