use std::collections::HashSet;

use macroquad::{
    prelude::*,
    ui::{hash, root_ui, widgets},
};
use noise::{Fbm, Perlin};
use rust_nonamerl_core::{
    property::{HealthData, Property},
    world::{EntityKey, World},
    Action, ActionQueue, AddSpriteOptions, BuilderAlgoWithNoise, Camera, Camera2D, Dimension2,
    Dimension2D, FovOccluder, IntExtent2D, IntVector2, Map, MapBuilder, MapCommand, MapCommands,
    MoveAction, RandomWalkBuilder, RenderOp, Renderer, RoomBuilder, SpriteSheet, Tile,
    TileSpriteInfo, Vec2, Viewport, VisibilityOcclusion, Visible, Visited, Walkable,
};

fn window_conf() -> Conf {
    Conf {
        window_title: "Window Conf".to_owned(),
        fullscreen: false,
        window_width: 1500,
        window_height: 800,
        ..Default::default()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TileKind {
    Grass,
    Floor,
    Wall,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TestTile {
    pub kind: TileKind,
    pub visited: bool,
    pub visible: bool,
}

impl TestTile {
    pub fn new(kind: TileKind) -> Self {
        Self {
            kind,
            visited: false,
            visible: false,
        }
    }
}

impl Default for TestTile {
    fn default() -> Self {
        Self {
            kind: TileKind::Grass,
            visited: false,
            visible: false,
        }
    }
}

impl Tile for TestTile {
    fn sprite_info(&self) -> TileSpriteInfo {
        match self.kind {
            TileKind::Grass => TileSpriteInfo::SpriteSheet("grass"),
            TileKind::Floor => TileSpriteInfo::SpriteSheet("floor"),
            TileKind::Wall => TileSpriteInfo::SpriteSheet("wall"),
        }
    }
}
impl Visible for TestTile {
    fn is_visible(&self) -> bool {
        self.visible
    }

    fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }
}
impl Visited for TestTile {
    fn is_visited(&self) -> bool {
        self.visited
    }

    fn set_visited(&mut self, visited: bool) {
        self.visited = visited;
    }
}
impl FovOccluder for TestTile {}
impl Walkable for TestTile {
    fn is_walkable(&self) -> bool {
        self.kind != TileKind::Wall
    }
}

fn create_player<T: Tile>(world: &mut World<T>, pos: IntVector2) -> EntityKey {
    let mut entities = world.entities.borrow_mut();

    let mut player_key = entities.add("Player", |player| {
        player.add_property(Property::Xp(5));
        player.add_property(Property::Health(HealthData { health: 100 }));
        player.add_property(Property::Position(pos));
    });
    // entities.get(player_key).unwrap()
    player_key
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut fov_cells = HashSet::<IntVector2>::new();
    let mut current_fov_cells = HashSet::<IntVector2>::new();
    let mut world = World::new();
    let mut action_queue = ActionQueue::new();
    let mut map_builder =
        MapBuilder::<TestTile>::new(IntExtent2D::new(0, 0, 100, 100), Dimension2D::new(24, 24));
    let mut world_x = 240.;
    let mut world_y = 240.;
    map_builder.add_tile("grass", TestTile::default());
    map_builder.add_tile("water", TestTile::default());
    map_builder.add_tile("floor", TestTile::new(TileKind::Floor));
    map_builder.add_tile("wall", TestTile::new(TileKind::Wall));

    // let mut map_commands = MapCommands::default();
    let noise = Fbm::<Perlin>::default();
    let f = |x: i32, y: i32, value: f64| {
        // println!("x: {}, y: {}, value: {}", x, y, value);
        if value > 0.1 {
            Some(TestTile::default())
        } else {
            None
        }
    };

    map_builder.add_step(&RandomWalkBuilder::new(IntVector2::new(10, 10)));
    map_builder.add_step(&BuilderAlgoWithNoise::new(noise, f));
    map_builder.add_step(&RoomBuilder::new());

    let map_size = map_builder.map.size();

    let mut map = map_builder.map;

    let start_point = IntVector2::new(5, 5);
    let player = create_player(&mut world, start_point);

    let mut draw_ops: Vec<RenderOp<TestTile>> = Vec::new();
    for i in 0..10 {
        for j in 0..10 {
            let tile = map.get(i, j);

            if let Some(tile) = tile {
                draw_ops.push(RenderOp::HighlightCell(i, j));
            }
        }
    }

    let mut texture = load_texture("assets/urizen_onebit_tileset__v1d0.png")
        .await
        .unwrap();
    texture.set_filter(FilterMode::Nearest);
    let mut sprites = SpriteSheet::new(texture);
    sprites.add_sprite(
        "test",
        Rect {
            x: 0.,
            y: 10.,
            w: 12.,
            h: 12.,
        },
        Some(AddSpriteOptions {
            gap: (1, 1),
            size: (12, 12),
        }),
    );
    sprites.add_sprite(
        "grass",
        Rect {
            x: 0.,
            y: 9.,
            w: 12.,
            h: 12.,
        },
        Some(AddSpriteOptions {
            gap: (1, 1),
            size: (12, 12),
        }),
    );
    sprites.add_sprite(
        "floor",
        Rect {
            x: 0.,
            y: 2.,
            w: 12.,
            h: 12.,
        },
        Some(AddSpriteOptions {
            gap: (1, 1),
            size: (12, 12),
        }),
    );

    sprites.add_sprite(
        "wall",
        Rect {
            x: 0.,
            y: 3.,
            w: 12.,
            h: 12.,
        },
        Some(AddSpriteOptions {
            gap: (1, 1),
            size: (12, 12),
        }),
    );
    clear_background(LIGHTGRAY);

    let (rect, texture) = sprites.get_sprite("test");

    // Creazione del viewport con posizione (0, 0) e dimensioni (800, 600)
    let viewport = Viewport {
        x: 0.0,
        y: 0.0,
        width: 800.0,
        height: 600.0,
    };

    let renderer = Renderer::from_map_cell_size(map.cell_size());

    // Creazione della camera inizializzata dal viewport e con scala di zoom 1.0
    let mut camera = Camera2D::from_viewport(0.0, 0.0, &viewport, 1.0);
    // Proiettare il punto del mondo (200, 300) nel viewport (come prima)
    camera.center_on_world_point(world_x, world_y, &viewport);

    loop {
        if map.commands_available() {
            map.process_commands();
        }
        if is_key_down(KeyCode::Right) {
            world_x += 1.0 * 12.;
            let move_action = MoveAction::new(IntVector2::new(1, 0), player);
            // move_action.perform(&world);
            action_queue.add(Box::new(move_action));
        }

        if is_key_down(KeyCode::Left) {
            world_x -= 1.0 * 12.;
            // camera.center_on_fixed_world_point(world_x, world_y, &viewport)
            let move_action = MoveAction::new(IntVector2::new(-1, 0), player);
            // move_action.perform(&world);
            action_queue.add(Box::new(move_action));
        }

        if is_key_down(KeyCode::Up) {
            world_y -= 1.0 * 12.;
            // camera.center_on_fixed_world_point(world_x, world_y, &viewport)
            let move_action = MoveAction::new(IntVector2::new(0, -1), player);
            // move_action.perform(&world);
            action_queue.add(Box::new(move_action));
        }

        if is_key_down(KeyCode::Down) {
            world_y += 1.0 * 12.;
            let move_action = MoveAction::new(IntVector2::new(0, 1), player);
            // move_action.perform(&world);
            // camera.center_on_fixed_world_point(world_x, world_y, &viewport)
            action_queue.add(Box::new(move_action));
        }

        action_queue.process_actions(&mut world, &mut map);

        let mouse_pos = mouse_position();

        // mouse wheel zoom
        let zoom = mouse_wheel();
        if zoom.1 != 0.0 {
            // println!("zoom: {:?}", zoom);
            let zoom_scale = 1.0 + zoom.1 / (10.0 * screen_height());
            camera.zoom_scale *= zoom_scale;
            // camera.center_on_fixed_world_point(world_x, world_y, &viewport)
        }

        if let Some(Property::Position(pos)) = world
            .entities
            .borrow()
            .get(player)
            .unwrap()
            .get_property(Property::POSITION)
        {
            let player_world_pos = (
                pos.x() as f32 * map.cell_size().width() as f32,
                pos.y() as f32 * map.cell_size().height() as f32,
            );
            // camera.center_on_world_point(player_world_pos.0, player_world_pos.1, &viewport);
            world_x = player_world_pos.0;
            world_y = player_world_pos.1;
        }
        camera.center_on_world_point(world_x, world_y, &viewport);
        let (viewport_x, viewport_y) = camera.world_to_viewport(world_x, world_y, &viewport);
        let (x_p, y_p) =
            camera.world_to_viewport(viewport.width / 2., viewport.height / 2., &viewport);
        let (viewport_cell_x, viewport_cell_y) = camera.world_to_viewport(24.0, 24., &viewport);
        let (viewport_camera_pos_x, viewport_camera_pos_y) =
            camera.world_to_viewport(camera.position_x, camera.position_y, &viewport);

        draw_circle(
            viewport_camera_pos_x,
            viewport_camera_pos_y,
            5.,
            Color {
                r: 1.0,
                g: 0.,
                b: 0.,
                a: 1.,
            },
        );

        if viewport.contains_screen_point(mouse_pos.0, mouse_pos.1) {
            draw_circle(
                mouse_pos.0,
                mouse_pos.1,
                5.,
                Color {
                    r: 0.0,
                    g: 1.,
                    b: 0.,
                    a: 1.,
                },
            );
            let world_mouse_pos = camera.viewport_to_world(mouse_pos.0, mouse_pos.1, &viewport);

            let visibile_cells = camera.get_visibile_extent(
                &viewport,
                map.cell_size().width(),
                map.cell_size().height(),
            );

            // println!("min_cell: {:?}, max_cell: {:?}", min_cell, max_cell);

            // for i in min_cell.0..max_cell.0 {
            //     for j in min_cell.1..max_cell.1 {
            // println!("cells: {:?} ", map.len());
            let mut map_batch = Vec::<RenderOp<TestTile>>::new();
            for i in visibile_cells.left()..visibile_cells.right() {
                for j in visibile_cells.top()..visibile_cells.bottom() {
                    let coords = map.coords_of_cell(i, j);
                    if coords.is_none() {
                        continue;
                    }
                    let coords = coords.unwrap();
                    // let (x, y) =
                    //     camera.world_to_viewport(coords.x() as f32, coords.y() as f32, &viewport);
                    // // println!("x: {}, y: {}", x, y);
                    // draw_rectangle(
                    //     x,
                    //     y,
                    //     24. * camera.zoom_scale,
                    //     24. * camera.zoom_scale,
                    //     Color {
                    //         r: (i as f32 + 1.) / (visibile_cells.width()) as f32,
                    //         g: (j as f32 + 1.) / (visibile_cells.height()) as f32,
                    //         b: 0.,
                    //         a: 0.6,
                    //     },
                    // );
                    if let Some(tile) = map.get(i, j) {
                        map_batch.push(RenderOp::DrawTile(i, j, tile));
                    }
                    // map_batch.push(RenderOp::DrawTile(i, j, map.get(i, j).unwrap()));
                }
            }

            renderer.batch_render(&camera, &viewport, &sprites, &map_batch);

            map_batch.clear();
            map_batch.push(RenderOp::FillCell(
                (world_mouse_pos.0 / map.cell_size().width() as f32) as i32,
                (world_mouse_pos.1 / map.cell_size().height() as f32) as i32,
                Color {
                    r: 1.,
                    g: 1.,
                    b: 1.,
                    a: 0.5,
                },
            ));

            // map.fov_iter(start,fov_size)

            let fov_size = 4;
            let mut coords = (
                (world_mouse_pos.0 / map.cell_size().width() as f32) as i32,
                (world_mouse_pos.1 / map.cell_size().height() as f32) as i32,
            );
            if let Some(Property::Position(pos)) = world
                .entities
                .borrow()
                .get(player)
                .unwrap()
                .get_property(Property::POSITION)
            {
                coords = (pos.x(), pos.y());
            }

            // let candidate_fov_cells = Tree::<(IntVector2, TestTile)>::new();

            // loop over the border of a 5x5 grid centerd in the mouse position
            for i in -fov_size..=fov_size {
                for j in -fov_size..=fov_size {
                    if i == fov_size || i == -fov_size || j == fov_size || j == -fov_size {
                        let target = (coords.0 + i, coords.1 + j);
                        let path = map.line(
                            IntVector2::new(coords.0, coords.1),
                            IntVector2::new(target.0, target.1),
                        );
                        // println!("target: {:?}", target);
                        'outer: for tile in path.iter().map(|v| (v, map.get(v.x(), v.y())))
                        //.take_while(|x| x.1.is_some())
                        {
                            let (p, tile) = tile;
                            // print!("p: {:?}, ", p);
                            if current_fov_cells.contains(p) {
                                // println!("cell already in fov");
                                continue;
                            }

                            let t = map.get(p.x(), p.y());
                            if let Some(tile) = t {
                                // let tile = tile.unwrap();

                                if !(tile.block_visibility() == TestTile::BLOCKED) {
                                    map_batch.push(RenderOp::FillCell(
                                        p.x(),
                                        p.y(),
                                        Color {
                                            r: 1.,
                                            g: 1.,
                                            b: 1.,
                                            a: 0.5,
                                        },
                                    ));
                                    //   BUG: implement map commands
                                    // (&mut map).set_visited(p.x(), p.y(), true);
                                    // map.set_visited(p.x(), p.y(), true);
                                    map.add_command(MapCommand::SetVisited(*p, true));
                                    current_fov_cells.insert(*p);
                                } else {
                                    // println!("tile blocked");
                                    break 'outer;
                                }
                            } else {
                                // println!("tile not found");
                                break 'outer;
                            }
                        }
                    }

                    // map_batch.push(RenderOp::HighlightCell(coords.x(), coords.y()));
                }
            }
            // println!("fov_cells: {:?}", current_fov_cells);
            let fov_cells_to_remove = fov_cells.difference(&current_fov_cells);
            let fov_cells_to_add = current_fov_cells.difference(&fov_cells);

            map.add_commands(
                fov_cells_to_remove
                    .map(|v| MapCommand::SetVisible(*v, false))
                    .collect(),
            );

            map.add_commands(
                fov_cells_to_add
                    .map(|v| MapCommand::SetVisible(*v, true))
                    .collect(),
            );

            fov_cells = current_fov_cells.clone();
            current_fov_cells.clear();

            if let Some(Property::Position(pos)) = world
                .entities
                .borrow()
                .get(player)
                .unwrap()
                .get_property(Property::POSITION)
            {
                println!("player pos: {:?}", pos);
                map_batch.push(RenderOp::FillCell(
                    pos.x(),
                    pos.y(),
                    Color {
                        r: 1.,
                        g: 1.,
                        b: 1.,
                        a: 0.5,
                    },
                ));
            }

            renderer.batch_render(&camera, &viewport, &sprites, &map_batch);

            draw_text_ex(
                &format!(
                    "world_mouse_pos: {:?}",
                    (
                        (world_mouse_pos.0 / map.cell_size().width() as f32) as i32,
                        (world_mouse_pos.1 / map.cell_size().height() as f32) as i32
                    )
                ),
                mouse_pos.0 + 10.,
                mouse_pos.1 + 10.,
                TextParams::default(),
            );
        } else {
            draw_circle(
                mouse_pos.0,
                mouse_pos.1,
                5.,
                Color {
                    r: 0.0,
                    g: 0.,
                    b: 1.,
                    a: 1.,
                },
            );
        }

        // draw_rectangle(
        //     viewport.x,
        //     viewport.y,
        //     viewport.width,
        //     viewport.height,
        //     Color {
        //         r: 1.0,
        //         g: 0.,
        //         b: 0.,
        //         a: 0.3,
        //     },
        // );

        // for i in 0..10 {
        //     for j in 0..10 {
        //         let coords = map.coords_of_cell(i, j).unwrap();
        //         let (x, y) =
        //             camera.world_to_viewport(coords.x() as f32, coords.y() as f32, &viewport);

        //         draw_rectangle(
        //             x,
        //             y,
        //             24. / camera.zoom_scale,
        //             24. / camera.zoom_scale,
        //             Color {
        //                 r: (i as f32 + 1.) / 10.,
        //                 g: (j as f32 + 1.) / 10.,
        //                 b: 0.,
        //                 a: 0.6,
        //             },
        //         );
        //     }
        // }

        // renderer.batch_render(&camera, &viewport, &sprites, &draw_ops);

        draw_texture_ex(
            texture,
            viewport_x,
            viewport_y,
            WHITE,
            DrawTextureParams {
                source: Some(*rect),
                dest_size: Some(macroquad::math::Vec2::new(
                    24. / camera.zoom_scale,
                    24. / camera.zoom_scale,
                )),
                ..Default::default()
            },
        );

        draw_circle(
            x_p,
            y_p,
            5.,
            Color {
                r: 1.0,
                g: 0.,
                b: 0.,
                a: 1.,
            },
        );

        widgets::Window::new(
            hash!(),
            vec2(viewport.x + viewport.width + 100., viewport.y),
            vec2(320., 400.),
        )
        .label("Camera")
        .titlebar(true)
        .movable(false)
        .ui(&mut *root_ui(), |ui| {
            ui.label(None, "Camera");
            ui.separator();
            ui.label(
                None,
                &format!("pos: ({}, {})", camera.position_x, camera.position_y),
            );
            ui.label(None, &format!("zoom: {}", camera.zoom_scale));
            ui.label(
                None,
                &format!("mouse pos: ({}, {})", mouse_pos.0, mouse_pos.1),
            );

            ui.separator();
            if let Some(Property::Position(pos)) = world
                .entities
                .borrow()
                .get(player)
                .unwrap()
                .get_property(Property::POSITION)
            {
                ui.label(None, &format!("player pos: {:?}", pos));
            };
        });

        next_frame().await
    }
}
