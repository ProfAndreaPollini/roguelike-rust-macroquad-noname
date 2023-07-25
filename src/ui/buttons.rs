#![allow(dead_code)]

use macroquad::{
    prelude::{is_mouse_button_down, mouse_position, Color, MouseButton, Rect, Vec2, BLACK, WHITE},
    shapes::{draw_rectangle, draw_rectangle_lines},
    text::{draw_text_ex, measure_text, Font, TextParams},
};

pub struct Button {
    id: u32,
    text: String,
    rect: Rect,
    pub normal_color: Color,
    pub hovered_color: Option<Color>,
    pub normal_bg_color: Color,
    pub hovered_bg_color: Option<Color>,
    offset_y: f32,
    hovered: bool,
    pressed: bool,
    clicked: bool,
    font: Font,
    font_size: u16,
    padding_rect: Vec2,
    pub on_click: Option<Box<dyn Fn(u32)>>,
}

impl Button {
    pub fn new(text: &str, position: Vec2, font: Font, font_size: u16, padding: Vec2) -> Self {
        let measures = measure_text(text, Some(font), font_size, 1.);
        let rect = Rect::new(position.x, position.y, measures.width, measures.height);

        Self {
            id: 0,
            text: text.to_owned(),
            rect,
            normal_color: WHITE,
            hovered_color: None,
            normal_bg_color: BLACK,
            hovered_bg_color: None,
            hovered: false,
            pressed: false,
            clicked: false,
            offset_y: measures.offset_y,
            font,
            font_size,
            padding_rect: padding,
            on_click: None,
        }
    }

    pub fn on_click<F>(&mut self, f: F)
    where
        F: 'static + Fn(u32),
    {
        self.on_click = Some(Box::new(f));
    }

    pub fn update(&mut self) {
        let mouse_position = mouse_position();

        self.hovered = mouse_position.0 >= self.rect.x
            && mouse_position.0 <= self.rect.x + self.rect.w
            && mouse_position.1 >= self.rect.y
            && mouse_position.1 <= self.rect.y + self.rect.h;

        self.pressed = is_mouse_button_down(MouseButton::Left);

        self.clicked = self.hovered && self.pressed;
    }

    // pub fn process_events(&self) {
    // TODO: fix this
    //     if self.clicked {
    //         let on_click = self.on_click(1);
    //         if let Some(on_click) = self.on_click {
    //             if self.id() == 0 {
    //                 panic!("Button id is not set");
    //             }
    //             on_click(self.id());
    //         }
    //     }
    // }

    pub fn draw(&self) {
        let mut color = self.normal_color;
        let mut bg_color = self.normal_bg_color;

        if self.hovered {
            color = self.hovered_color.unwrap_or(color);
            bg_color = self.hovered_bg_color.unwrap_or(bg_color);
        }

        draw_rectangle(
            self.rect.x - self.padding_rect.x,
            self.rect.y - self.padding_rect.y,
            self.rect.w + self.padding_rect.x * 2.,
            self.rect.h + self.padding_rect.y * 2.,
            bg_color,
        );

        draw_rectangle_lines(
            self.rect.x - self.padding_rect.x,
            self.rect.y - self.padding_rect.y,
            self.rect.w + self.padding_rect.x * 2.,
            self.rect.h + self.padding_rect.y * 2.,
            1.,
            color,
        );

        draw_text_ex(
            &self.text,
            self.rect.x,
            self.rect.y + self.offset_y,
            TextParams {
                font: self.font,
                font_size: self.font_size,
                font_scale: 1.,
                color: color,
                ..Default::default()
            },
        );
    }

    pub(crate) fn clicked(&self) -> bool {
        self.clicked
    }

    pub fn id(&self) -> u32 {
        self.id
    }
}

impl Default for Button {
    fn default() -> Self {
        let measures = measure_text("test", None, 20, 1.);
        let rect = Rect::new(0., 0. - measures.offset_y, measures.width, measures.height);

        Self {
            id: 0,
            text: "test".to_owned(),
            rect,
            normal_color: WHITE,
            hovered_color: None,
            normal_bg_color: BLACK,
            hovered_bg_color: None,
            hovered: false,
            pressed: false,
            clicked: false,
            offset_y: measures.offset_y,
            font: Font::default(),
            font_size: 20,
            padding_rect: Vec2::new(5., 5.),
            on_click: None,
        }
    }
}

// #[derive(Debug)]
// struct Menu {
//     buttons: Vec<Button>,
//     font: Font,
//     font_size: u16,
//     padding_rect: Vec2,
// }
