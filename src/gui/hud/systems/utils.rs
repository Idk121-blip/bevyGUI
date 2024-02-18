use crate::components::{CLOCK_ASSET_FILE, WEATHER_ASSET_FILE};
use crate::gui::components::TILE_DIMENSION;
use bevy::prelude::*;
use robotics_lib::world::environmental_conditions::WeatherType;

pub(crate) fn get_weather_asset(
    weather: &WeatherType,
    asset_server: &Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
) -> (Handle<TextureAtlas>, usize) {
    const COLUMNS: usize = 3;
    const ROWS: usize = 2;
    let texture_handle = asset_server.load(WEATHER_ASSET_FILE);
    let texture_atlas = TextureAtlas::from_grid(
        texture_handle,
        Vec2::new(TILE_DIMENSION, TILE_DIMENSION),
        COLUMNS,
        ROWS,
        None,
        None,
    );
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    return (texture_atlas_handle, get_weather_asset_number(&weather));
}

pub(crate) fn get_weather_asset_number(weather: &WeatherType) -> usize {
    match weather {
        WeatherType::Sunny => 0,
        WeatherType::Rainy => 1,
        WeatherType::Foggy => 2,
        WeatherType::TrentinoSnow => 3,
        WeatherType::TropicalMonsoon => 4,
    }
}

pub(crate) fn get_clock_asset(
    time: &str,
    asset_server: &Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
) -> (Handle<TextureAtlas>, usize) {
    const COLUMNS: usize = 4;
    const ROWS: usize = 12;
    let texture_handle = asset_server.load(CLOCK_ASSET_FILE);
    let texture_atlas = TextureAtlas::from_grid(
        texture_handle,
        Vec2::new(128., 128.),
        COLUMNS,
        ROWS,
        None,
        None,
    );
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    return (texture_atlas_handle, get_time_asset_number(time));
}

pub(crate) fn get_time_asset_number(time: &str) -> usize {
    let time_string = time.to_string();
    let hour = time_string.split(":").collect::<Vec<&str>>()[0]
        .parse::<usize>()
        .unwrap();
    let minute = time_string.split(":").collect::<Vec<&str>>()[1]
        .parse::<usize>()
        .unwrap();
    return ((hour % 12) * 4) + (minute / 15);
}
