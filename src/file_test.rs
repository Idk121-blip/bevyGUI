use bevy::core_pipeline::clear_color::ClearColorConfig;
use bevy::input::mouse::{MouseMotion, MouseScrollUnit, MouseWheel};
use bevy::prelude::*;
use bevy::render::render_resource::Texture;
use bevy::time::Timer;
use rand::Rng;
use robotics_lib;
use robotics_lib::world::tile::TileType::{
    DeepWater, Grass, Hill, Lava, Mountain, Sand, ShallowWater, Snow, Street,
};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
const TILE_DIMENSION: f32 = 32.0;
const PROCESS_TICK_TIME: f32 = 1.0;
use robotics_lib::energy::Energy;
use robotics_lib::event::events::Event;
use robotics_lib::interface::{robot_map, where_am_i};
use robotics_lib::runner::backpack::BackPack;
use robotics_lib::runner::{Robot, Runnable};
use robotics_lib::utils::LibError::*;
use robotics_lib::world::coordinates::Coordinate;
use robotics_lib::world::environmental_conditions::EnvironmentalConditions;
use robotics_lib::world::environmental_conditions::WeatherType::{Rainy, Sunny};
use robotics_lib::world::tile::Content::{
    Bank, Bin, Coin, Crate, Fire, Fish, Garbage, Market, Rock, Tree,
};
use robotics_lib::world::tile::TileType;
use robotics_lib::world::tile::TileType::*;
use robotics_lib::world::tile::{Content, Tile};
use robotics_lib::world::world_generator::Generator;
use robotics_lib::world::World;
use std::collections::HashMap;

fn main() {
    let my_banana = MyRobot(Robot::new());

    let mut generator = WorldGenerator::init(2);
    let run = robotics_lib::runner::Runner::new(Box::new(my_banana), &mut generator);

    match run {
        Ok(mut x) => {
            let _ = x.game_tick();
        }
        Err(x) => {
            println!("Oh shit");
            println!("{:?}", x);
        }
    }

    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Pinguino".into(),
                        resolution: (672.0, 672.0).into(),
                        resizable: true,
                        ..default()
                    }),
                    ..default()
                })
                .build(),
        )
        .add_systems(Startup, (setup, setup_tileex))
        .add_systems(
            Update,
            (
                characters_movement,
                spawn_coin,
                function,
                zoom_handler,
                camera_movement,
                get_mouse_position,
            ),
        )
        .run();
}

#[derive(Component)]
pub struct Player {
    pub energy: usize,
    pub money: usize,
}
pub fn zoom_handler(
    mut query: Query<&mut OrthographicProjection, With<Camera>>,
    mut scroll_evr: EventReader<MouseWheel>,
) {
    for ev in scroll_evr.iter() {
        match ev.unit {
            MouseScrollUnit::Line => {
                for mut projection in query.iter_mut() {
                    let mut log_scale = projection.scale.ln();
                    log_scale -= ev.y * 0.05;
                    projection.scale = log_scale.exp();
                }
            }
            _ => {
                println!("No zoom available");
            }
        }
    }

    // println!("{:?}", scroll_evr.iter().next());
}

//write a function that changes the first tile of the map with the icon inside tiles.pmg

pub fn update_map_tile(
    mut commands: Commands,
    mut atlases: ResMut<Assets<TextureAtlas>>,
    asset_server: Res<AssetServer>,
    query: Query<&Handle<TextureAtlas>>,
    mut texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
) {
    for (index, handle) in query.iter().enumerate() {
        if index != 100000 {
            if let Some(atlas) = atlases.get_mut(handle) {
                let texture_handle = asset_server.load("tiles.png");
                let texture_atlas = TextureAtlas::from_grid(
                    texture_handle,
                    Vec2::new(32.0, 32.0),
                    3,
                    3,
                    None,
                    None,
                );

                atlases.insert(handle, texture_atlas)
            }
        }
    }
}

pub fn camera_movement(
    mut query: Query<&mut Transform, (With<Camera>, Without<Player>)>,
    time: Res<Time>,
    input: Res<Input<KeyCode>>,
    player: Query<&Transform, With<Player>>,
) {
    if input.pressed(KeyCode::Right) {
        for mut projection in query.iter_mut() {
            projection.translation.x += 5.0;
        }
    }
    if input.pressed(KeyCode::Left) {
        for mut projection in query.iter_mut() {
            projection.translation.x -= 5.0;
        }
    }
    if input.pressed(KeyCode::Up) {
        for mut projection in query.iter_mut() {
            projection.translation.y += 5.0;
        }
    }
    if input.pressed(KeyCode::Down) {
        for mut projection in query.iter_mut() {
            projection.translation.y -= 5.0;
        }
    }
    if input.pressed(KeyCode::R) {
        for mut projection in query.iter_mut() {
            let tranform_player = player.single();
            let player_x = tranform_player.translation.x;
            let player_y = tranform_player.translation.y;
            projection.translation.y = player_y;
            projection.translation.x = player_x;
        }
    }
}

/*write a function to get mouse position based on the camera*/
fn get_mouse_position(
    mut query: Query<&mut Transform, (With<Camera>, Without<RobotUI>)>,
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut mouse_click_events: Res<Input<MouseButton>>,
) {
    if mouse_click_events.pressed(MouseButton::Left) {
        for ev in mouse_motion_events.iter() {
            for mut projection in query.iter_mut() {
                projection.translation.x -= ev.delta.x;
                projection.translation.y += ev.delta.y;
            }
        }
    }
}
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle {
        camera_2d: Camera2d {
            clear_color: ClearColorConfig::Custom(Color::DARK_GRAY),
        },
        ..default()
    });
    let texture = asset_server.load("penguin.png");
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(30.0, 30.0)),
                ..default()
            },
            texture,
            ..default()
        },
        Player {
            energy: 100,
            money: 100,
        },
    ));

    commands.spawn(TickUpdate(Timer::from_seconds(
        PROCESS_TICK_TIME,
        TimerMode::Repeating,
    )));
}

fn characters_movement(
    mut characters: Query<(&mut Transform, &mut Player)>,
    input: Res<Input<KeyCode>>,
) {
    for (mut transform, mut player) in &mut characters {
        if input.just_pressed(KeyCode::W) {
            transform.translation.y += TILE_DIMENSION;
            // player.energy-=10;

            println!("{:?}, {}", player.energy, transform.translation.y)
        }
        if input.just_pressed(KeyCode::S) {
            transform.translation.y -= TILE_DIMENSION;
            // player.energy-=10;
            println!("{:?}, {}", player.energy, transform.translation.y)
        }
        if input.just_pressed(KeyCode::D) {
            transform.translation.x += TILE_DIMENSION;
            // player.energy-=10;

            println!("{:?}, {}", player.energy, transform.translation.y)
        }
        if input.just_pressed(KeyCode::A) {
            transform.translation.x -= TILE_DIMENSION;
            println!("{:?}, {}", player.energy, transform.translation.y)
        }
    }
}

fn spawn_coin(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    input: Res<Input<KeyCode>>,
    mut player: Query<&Transform, With<Player>>,
) {
    if !input.just_pressed(KeyCode::Space) {
        return;
    }

    if input.just_pressed(KeyCode::Space) {
        let tranform_player = player.single_mut();
        let texture = asset_server.load("coin.png");
        let player_x = tranform_player.translation.x;
        let player_y = tranform_player.translation.y;
        let player_z = tranform_player.translation.z;
        commands.spawn((SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(15.0, 15.0)),
                ..default()
            },
            texture,
            transform: Transform::from_xyz(player_x, player_y, player_z - 1.0),
            ..default()
        },));
    }
}

fn setup_tileex(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let map = create_map();

    for i in (0..100) {
        for j in (-100..100) {
            let (texture_atlas_handle, index) = get_asset_of_tile(
                map[(j + 100) as usize][(i + 100) as usize],
                &asset_server,
                &mut texture_atlases,
            );

            let test = Transform::from_xyz((i * 32) as f32, -(j * 32) as f32, -5.0)
                .with_scale(Vec3::new(1.0, 1.0, 0.0));
            commands.spawn(
                (SpriteSheetBundle {
                    texture_atlas: texture_atlas_handle.clone(),
                    sprite: TextureAtlasSprite::new(index),
                    transform: test,
                    ..default()
                }),
            );
        }
    }
}
