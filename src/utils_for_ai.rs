use crate::components::{
    MapEvent, RobotResource, ENVIRONMENT, EVENT, PLOTUPDATE, ROBOT_COL, ROBOT_ROW, SCORE,
};
use crate::gui::system::startup::{App, DefaultPlugins, ImagePlugin, Window, WindowPlugin};
use crate::gui::GuiPlugin;
use bevy::prelude::*;
use bevy_utils::default;
use oxagaudiotool::sound_config::OxAgSoundConfig;
use oxagaudiotool::OxAgAudioTool;
use robotics_lib::interface::{get_score, look_at_sky, robot_map, where_am_i};
use robotics_lib::runner::{Runnable, Runner};
use robotics_lib::world::environmental_conditions::WeatherType;
use robotics_lib::world::tile::TileType;
use robotics_lib::world::World;
use std::collections::HashMap;

pub fn ui_variable_update(robot: &mut impl Runnable, world: &mut World) {
    let (_robot_view, robot_position) = where_am_i(robot, world);
    {
        let mut env = ENVIRONMENT.lock().unwrap();
        *env = Some(look_at_sky(&world));
    }
    let robot_map = robot_map(world);

    let mut plot_update = PLOTUPDATE.lock().unwrap();
    let mut events = EVENT.lock().unwrap();
    let mut robot_col = ROBOT_COL.lock().unwrap();
    let mut robot_row = ROBOT_ROW.lock().unwrap();
    if robot_position.0 != *robot_col || *robot_row != robot_position.1 {
        events.push(MapEvent::UpdateRobot);
        *robot_col = robot_position.0;
        *robot_row = robot_position.1;
    }
    let mut update_map = false;
    match robot_map {
        None => {}
        Some(v) => {
            for r in 0..v.len() {
                for c in 0..v[0].len() {
                    if !v[r][c].is_none() {
                        if plot_update[r][c].is_none() {
                            update_map = true;
                            plot_update[r][c] = v[r][c].clone();
                        } else if plot_update[r][c].as_ref().unwrap().tile_type
                            != v[r][c].as_ref().unwrap().tile_type
                        {
                            update_map = true;
                            plot_update[r][c] = v[r][c].clone();
                        } else if plot_update[r][c].as_ref().unwrap().content
                            != v[r][c].as_ref().unwrap().content
                        {
                            update_map = true;
                            plot_update[r][c].as_mut().unwrap().content =
                                v[r][c].clone().unwrap().content;
                        }
                    }
                }
            }
        }
    }
    let mut score = SCORE.lock().unwrap();
    *score = get_score(&world);
    if update_map {
        events.push(MapEvent::UpdateMap);
    }
}

pub fn robot_audio() -> OxAgAudioTool {
    let mut events = HashMap::new();
    // events.insert(
    //     Event::Ready,
    //     OxAgSoundConfig::new("assets/default/event/event_ready.ogg"),
    // );
    // events.insert(Event::Terminated, OxAgSoundConfig::new("assets/default/event/event_terminated.ogg"));
    // // events.insert(Event::EnergyRecharged(0), OxAgSoundConfig::new_with_volume("assets/default/event/event_energy_recharged.ogg", 0.1));
    // events.insert(Event::AddedToBackpack(Content::None, 0), OxAgSoundConfig::new("assets/default/event/event_add_to_backpack.ogg"));
    // events.insert(Event::RemovedFromBackpack(Content::None, 0), OxAgSoundConfig::new("assets/default/event/event_remove_from_backpack.ogg"));

    let mut tiles = HashMap::new();
    tiles.insert(
        TileType::ShallowWater,
        OxAgSoundConfig::new("assets/sounds/water_steps.wav"),
    );
    tiles.insert(
        TileType::Grass,
        OxAgSoundConfig::new("assets/sounds/grass_steps.wav"),
    );

    let mut weather = HashMap::new();
    weather.insert(
        WeatherType::Rainy,
        OxAgSoundConfig::new("assets/sounds/rain.wav"),
    );
    weather.insert(
        WeatherType::Foggy,
        OxAgSoundConfig::new("assets/sounds/fog.wav"),
    );
    weather.insert(
        WeatherType::Sunny,
        OxAgSoundConfig::new("assets/sounds/sunny.wav"),
    );
    weather.insert(
        WeatherType::TrentinoSnow,
        OxAgSoundConfig::new("assets/sounds/snow.wav"),
    );
    weather.insert(
        WeatherType::TropicalMonsoon,
        OxAgSoundConfig::new("assets/sounds/wind.wav"),
    );
    let audio = OxAgAudioTool::new(events, tiles, weather);
    if audio.is_err() {
        panic!();
    }
    audio.unwrap()
}

pub fn ui_runner(runner: Runner) {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Robot".into(),
                        resolution: (1200.0, 736.0).into(),
                        resizable: true,
                        ..default()
                    }),
                    ..default()
                })
                .build(),
        )
        .insert_non_send_resource(RobotResource { runner })
        .add_plugins(GuiPlugin)
        .run();
}
