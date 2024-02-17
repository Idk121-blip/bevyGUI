use bevy::core_pipeline::clear_color::ClearColorConfig;
use bevy::input::mouse::{MouseMotion, MouseScrollUnit, MouseWheel};
use bevy::prelude::*;
use bevy::render::render_resource::Texture;
use bevy::time::Timer;
use rand::Rng;
use robotics_lib;
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
use robotics_lib::world::tile::TileType::{
    DeepWater, Grass, Hill, Lava, Mountain, Sand, ShallowWater, Snow, Street,
};
use robotics_lib::world::tile::{Content, Tile};
use robotics_lib::world::world_generator::Generator;
use robotics_lib::world::World;
use std::collections::HashMap;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

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
