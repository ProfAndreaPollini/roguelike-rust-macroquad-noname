use std::rc::Rc;

use macroquad::{
    prelude::{Vec2, DARKGREEN, WHITE},
    text::{draw_text_ex, TextParams},
    window::{screen_height, screen_width},
};

use crate::{engine::viewport::Viewport, ui::buttons::Button};

use super::{SceneContext, UpdatableScene};

pub struct IntroScene {
    viewport: Option<Viewport>,
    btn_play: Option<Button>,
    btn_exit: Option<Button>,
}
impl IntroScene {
    pub fn new() -> Self {
        Self {
            viewport: None,
            btn_play: None,
            btn_exit: None,
        }
    }
}

impl UpdatableScene for IntroScene {
    fn process_input(
        &mut self,
        event: super::events::SceneEvent,
    ) -> Option<super::events::SceneCommands> {
        if self.btn_play.as_ref().unwrap().clicked() {
            Some(super::events::SceneCommands::ChangeScene(
                super::Scene::Game,
            ))
        } else if self.btn_exit.as_ref().unwrap().clicked() {
            Some(super::events::SceneCommands::Exit)
        } else {
            None
        }

        // match event {
        //     super::events::SceneEvent::KeyPressed(key) => {
        //         if key == macroquad::input::KeyCode::Space {
        //             Some(super::events::SceneCommands::ChangeScene(
        //                 super::Scene::Game,
        //             ))
        //         } else {
        //             None
        //         }
        //     }
        //     _ => None,
        // }
    }

    fn setup(&mut self, context: Rc<SceneContext>) {
        // info!("IntroScene setup");
        let viewport = Viewport::new(0.0, 0.0, 40.0, 30.0, Vec2::new(17.5, 18.7));
        self.viewport = Some(viewport);

        let font = *context.font.as_ref().unwrap().clone();
        let mut btn_play = Button::new(
            "Play",
            Vec2::new(screen_width() / 2., screen_height() / 2.),
            font,
            72,
            Vec2::new(10., 10.),
        );
        btn_play.hovered_bg_color = Some(DARKGREEN);
        btn_play.on_click = Some(Box::new(|button| {
            println!("Button clicked!!");
        }));
        self.btn_play = Some(btn_play);

        let mut btn_exit = Button::new(
            "Exit",
            Vec2::new(screen_width() / 2., screen_height() / 2. + 100.),
            font,
            72,
            Vec2::new(10., 10.),
        );
        btn_exit.hovered_bg_color = Some(DARKGREEN);
        btn_exit.on_click = Some(Box::new(|button| {
            println!("Exit!!");
        }));

        self.btn_exit = Some(btn_exit);
    }

    fn update(&mut self) {
        // info!("IntroScene update");
        self.btn_play.as_mut().unwrap().update();
        self.btn_exit.as_mut().unwrap().update();
    }

    fn draw(&self) {
        // info!("IntroScene draw");
        // let center = self.viewport.as_ref().unwrap().center();
        // //let cell_output_size = self.texture_manager.cell_output_size();
        // for x in 0..40 {
        //     for y in 0..30 {
        //         macroquad::shapes::draw_rectangle(
        //             (x as f32 + center.x) * 36.,
        //             (y as f32 + center.y) * 36.,
        //             36.,
        //             36.,
        //             macroquad::color::Color::new(x as f32 / 40., y as f32 / 30., 0.0, 1.0),
        //         );
        //     }
        // }

        // draw_text_ex(
        //     "Rust Roguelike",
        //     screen_width() / 2.,
        //     screen_height() / 2.,
        //     {
        //         TextParams {
        //             font_size: 120,
        //             font_scale: 1.0,
        //             color: WHITE,
        //             ..TextParams::default()
        //         }
        //     },
        // );
        self.btn_play.as_ref().unwrap().draw();
        self.btn_exit.as_ref().unwrap().draw();
    }
}
