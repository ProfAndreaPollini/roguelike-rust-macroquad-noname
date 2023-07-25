#![allow(dead_code)]

use macroquad::{
    prelude::{Color, Rect, Vec2},
    shapes::draw_rectangle,
    text::{draw_text_ex, measure_text, Font, TextParams},
};

pub struct Label {
    id: u32,
    text: String,
    rect: Rect,
    pub color: Color,
    // pub hovered_color: Option<Color>,
    pub bg_color: Color,
    // pub hovered_bg_color: Option<Color>,
    offset_y: f32,
    // hovered: bool,
    // pressed: bool,
    // clicked: bool,
    font: Font,
    font_size: u16,
    padding: Vec2,
    // pub on_click: Option<Box<dyn Fn(u32)>>,
}

impl Label {
    pub fn new(text: &str, font: Font, font_size: u16, padding: Vec2) -> Self {
        let measures = measure_text(text, Some(font), font_size, 1.);
        // let rect = Rect::new(position.x, position.y, measures.width, measures.height);
        Self {
            id: 1,
            text: text.to_owned(),
            // rect,
            color: Color::new(1.0, 1.0, 1.0, 1.0),
            bg_color: Color::new(0.0, 0.0, 0.0, 0.0),
            font,
            font_size,
            padding,
            offset_y: measures.offset_y,
            ..Default::default()
        }
    }

    pub fn update_label(&mut self, text: &str) {
        self.text = text.to_owned();
        let measures = measure_text(text, Some(self.font), self.font_size, 1.);
        self.rect = Rect::new(self.rect.x, self.rect.y, measures.width, measures.height);
    }

    pub fn draw(&self, pos: Vec2) {
        draw_rectangle(
            pos.x - self.padding.x,
            pos.y - self.padding.y,
            self.rect.w + self.padding.x * 2.,
            self.rect.h + self.padding.y * 2.,
            self.bg_color,
        );

        // draw_rectangle_lines(
        //     self.rect.x - self.padding_rect.x,
        //     self.rect.y - self.padding_rect.y,
        //     self.rect.w + self.padding_rect.x * 2.,
        //     self.rect.h + self.padding_rect.y * 2.,
        //     1.,
        //     color,
        // );

        draw_text_ex(
            &self.text,
            pos.x,
            pos.y + self.offset_y,
            TextParams {
                font: self.font,
                font_size: self.font_size,
                font_scale: 1.,
                color: self.color,
                ..Default::default()
            },
        );
    }
}

impl Default for Label {
    fn default() -> Self {
        Self {
            id: 1,
            text: String::from(""),
            rect: Rect::new(0., 0., 0., 0.),
            color: Color::new(1.0, 1.0, 1.0, 1.0),
            bg_color: Color::new(0.0, 0.0, 0.0, 0.0),
            font: Font::default(),
            font_size: 0,
            padding: Vec2::new(0., 0.),
            offset_y: 0.,
        }
    }
}
