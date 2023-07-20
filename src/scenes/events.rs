#![allow(dead_code)]
use macroquad::prelude::KeyCode;

use super::Scene;

#[derive(Debug)]
pub enum MouseEvent {
    Pressed(macroquad::prelude::MouseButton),
    Released(macroquad::prelude::MouseButton),
}

#[derive(Debug)]
pub enum SceneCommands {
    ChangeScene(Scene),
    Exit,
}

#[derive(Debug)]
pub enum SceneEvent {
    PlayGame,
    KeyPressed(KeyCode),
    Mouse(Vec<MouseEvent>),
}

impl ToString for SceneEvent {
    fn to_string(&self) -> String {
        stringify!(self).to_owned()
    }
}
