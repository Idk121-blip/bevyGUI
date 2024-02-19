use crate::components::{
    MapEvent, RobotResource, ENVIRONMENT, EVENT, PLOTUPDATE, ROBOT_COL, ROBOT_ROW, SCORE,
};
use crate::gui::system::startup::{App, DefaultPlugins, ImagePlugin, Window, WindowPlugin};
use crate::gui::GuiPlugin;
use crate::robot::Graphics;
use bevy::input::mouse::MouseScrollUnit;
use bevy::input::mouse::MouseWheel;
use bevy::prelude::*;
use bevy_utils::default;
use oxagaudiotool::sound_config::OxAgSoundConfig;
use oxagaudiotool::OxAgAudioTool;
use robotics_lib::event::events::Event as RobotEvent;
use robotics_lib::interface::{get_score, look_at_sky, robot_map, where_am_i};
use robotics_lib::runner::{Runnable, Runner};
use robotics_lib::world::environmental_conditions::WeatherType;
use robotics_lib::world::tile::Content;
use robotics_lib::world::tile::TileType::*;
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

pub fn robot_audio(graphic: &Graphics) -> OxAgAudioTool {
    return match graphic {
        Graphics::Alessio => {
            alessio_sound()
        }
        Graphics::Alessandro => {
            alessandro_sound()
        }
    }
}
fn alessandro_sound() -> OxAgAudioTool {
    OxAgSoundConfig::new_looped_with_volume("assets/default/music.ogg", 2.0);
    OxAgSoundConfig::new_looped_with_volume("assets/default/music.ogg", 2.0);

    let mut events = HashMap::new();
    events.insert(
        RobotEvent::Ready,
        OxAgSoundConfig::new("assets/default/event/event_ready.ogg"),
    );
    events.insert(
        RobotEvent::Terminated,
        OxAgSoundConfig::new("assets/default/event/event_terminated.ogg"),
    );
    // events.insert(Event::EnergyRecharged(0), OxAgSoundConfig::new_with_volume("assets/default/event/event_energy_recharged.ogg", 0.1));
    events.insert(
        RobotEvent::AddedToBackpack(Content::None, 0),
        OxAgSoundConfig::new("assets/default/event/event_add_to_backpack.ogg"),
    );
    events.insert(
        RobotEvent::RemovedFromBackpack(Content::None, 0),
        OxAgSoundConfig::new("assets/default/event/event_remove_from_backpack.ogg"),
    );

    let mut tiles = HashMap::new();
    tiles.insert(
        DeepWater,
        OxAgSoundConfig::new("assets/default/tile/tile_water.ogg"),
    );
    tiles.insert(
        ShallowWater,
        OxAgSoundConfig::new("assets/default/tile/tile_water.ogg"),
    );
    tiles.insert(
        Sand,
        OxAgSoundConfig::new("assets/default/tile/tile_sand.ogg"),
    );
    tiles.insert(
        Grass,
        OxAgSoundConfig::new("assets/default/tile/tile_grass.ogg"),
    );
    tiles.insert(
        Hill,
        OxAgSoundConfig::new("assets/default/tile/tile_grass.ogg"),
    );
    tiles.insert(
        Mountain,
        OxAgSoundConfig::new("assets/default/tile/tile_mountain.ogg"),
    );
    tiles.insert(
        Snow,
        OxAgSoundConfig::new("assets/default/tile/tile_snow.ogg"),
    );
    tiles.insert(
        Lava,
        OxAgSoundConfig::new("assets/default/tile/tile_lava.ogg"),
    );
    tiles.insert(
        Teleport(false),
        OxAgSoundConfig::new("assets/default/tile/tile_teleport.ogg"),
    );
    tiles.insert(
        Street,
        OxAgSoundConfig::new("assets/default/tile/tile_street.ogg"),
    );

    let mut weather = HashMap::new();
    weather.insert(
        WeatherType::Rainy,
        OxAgSoundConfig::new("assets/default/weather/weather_rainy.ogg"),
    );
    weather.insert(
        WeatherType::Foggy,
        OxAgSoundConfig::new("assets/default/weather/weather_foggy.ogg"),
    );
    weather.insert(
        WeatherType::Sunny,
        OxAgSoundConfig::new("assets/default/weather/weather_sunny.ogg"),
    );
    weather.insert(
        WeatherType::TrentinoSnow,
        OxAgSoundConfig::new("assets/default/weather/weather_winter.ogg"),
    );
    weather.insert(
        WeatherType::TropicalMonsoon,
        OxAgSoundConfig::new("assets/default/weather/weather_tropical.ogg"),
    );

    // Create the audio tool

    OxAgAudioTool::new(events, tiles, weather).unwrap()
}

fn alessio_sound() -> OxAgAudioTool {
    let events = HashMap::new();
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
        ShallowWater,
        OxAgSoundConfig::new("assets/sounds/water_steps.wav"),
    );
    tiles.insert(
        Grass,
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
        .add_systems(Update, zoom_handler)
        .run();
}

fn zoom_handler(
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
