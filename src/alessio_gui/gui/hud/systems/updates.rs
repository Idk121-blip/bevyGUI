use crate::components::{RobotResource, TextUpdate, ENVIRONMENT, FUTUREENVIRONMENT, SCORE};
use crate::gui::hud::components::*;
use crate::gui::hud::systems::utils::{get_time_asset_number, get_weather_asset_number};
use bevy::prelude::*;
use bevy_progressbar::ProgressBar;
use robotics_lib::world::tile::Content;

pub(crate) fn text_updater(
    mut clock_time: Query<
        &mut Text,
        (
            With<ClockTime>,
            Without<ClockIcon>,
            Without<EnergyBar>,
            Without<ScoreBar>,
            Without<WeatherIcon>,
            Without<FutureWeatherIcon>,
            Without<CoinText>,
            Without<RockText>,
            Without<TreeText>,
            Without<WaterText>,
            Without<GarbageText>,
            Without<FishText>,
        ),
    >,
    mut clock_icon: Query<(
        &mut UiTextureAtlasImage,
        (
            With<ClockIcon>,
            Without<ClockTime>,
            Without<EnergyBar>,
            Without<ScoreBar>,
            Without<WeatherIcon>,
            Without<FutureWeatherIcon>,
            Without<CoinText>,
            Without<RockText>,
            Without<TreeText>,
            Without<WaterText>,
            Without<GarbageText>,
            Without<FishText>,
        ),
    )>,
    mut energy_bar: Query<
        &mut ProgressBar,
        (
            With<EnergyBar>,
            Without<ClockTime>,
            Without<ClockIcon>,
            Without<ScoreBar>,
            Without<WeatherIcon>,
            Without<FutureWeatherIcon>,
            Without<CoinText>,
            Without<RockText>,
            Without<TreeText>,
            Without<WaterText>,
            Without<GarbageText>,
            Without<FishText>,
        ),
    >,
    mut score_bar: Query<
        &mut ProgressBar,
        (
            With<ScoreBar>,
            Without<ClockTime>,
            Without<ClockIcon>,
            Without<EnergyBar>,
            Without<WeatherIcon>,
            Without<FutureWeatherIcon>,
            Without<CoinText>,
            Without<RockText>,
            Without<TreeText>,
            Without<WaterText>,
            Without<GarbageText>,
            Without<FishText>,
        ),
    >,
    mut weather_icon: Query<(
        &mut UiTextureAtlasImage,
        (
            With<WeatherIcon>,
            Without<ClockTime>,
            Without<ClockIcon>,
            Without<EnergyBar>,
            Without<ScoreBar>,
            Without<FutureWeatherIcon>,
            Without<CoinText>,
            Without<RockText>,
            Without<TreeText>,
            Without<WaterText>,
            Without<GarbageText>,
            Without<FishText>,
        ),
    )>,
    mut future_weather_icon: Query<(
        &mut UiTextureAtlasImage,
        (
            With<FutureWeatherIcon>,
            Without<ClockTime>,
            Without<ClockIcon>,
            Without<EnergyBar>,
            Without<ScoreBar>,
            Without<WeatherIcon>,
            Without<CoinText>,
            Without<RockText>,
            Without<TreeText>,
            Without<WaterText>,
            Without<GarbageText>,
            Without<FishText>,
        ),
    )>,
    mut coin_text: Query<
        &mut Text,
        (
            With<CoinText>,
            Without<ClockTime>,
            Without<ClockIcon>,
            Without<EnergyBar>,
            Without<ScoreBar>,
            Without<WeatherIcon>,
            Without<FutureWeatherIcon>,
            Without<RockText>,
            Without<TreeText>,
            Without<WaterText>,
            Without<GarbageText>,
            Without<FishText>,
        ),
    >,
    mut rock_text: Query<
        &mut Text,
        (
            With<RockText>,
            Without<ClockTime>,
            Without<ClockIcon>,
            Without<EnergyBar>,
            Without<ScoreBar>,
            Without<WeatherIcon>,
            Without<FutureWeatherIcon>,
            Without<CoinText>,
            Without<TreeText>,
            Without<WaterText>,
            Without<GarbageText>,
            Without<FishText>,
        ),
    >,
    mut tree_text: Query<
        &mut Text,
        (
            With<TreeText>,
            Without<ClockTime>,
            Without<ClockIcon>,
            Without<EnergyBar>,
            Without<ScoreBar>,
            Without<WeatherIcon>,
            Without<FutureWeatherIcon>,
            Without<CoinText>,
            Without<RockText>,
            Without<WaterText>,
            Without<GarbageText>,
            Without<FishText>,
        ),
    >,
    mut water_text: Query<
        &mut Text,
        (
            With<WaterText>,
            Without<ClockTime>,
            Without<ClockIcon>,
            Without<EnergyBar>,
            Without<ScoreBar>,
            Without<WeatherIcon>,
            Without<FutureWeatherIcon>,
            Without<CoinText>,
            Without<RockText>,
            Without<TreeText>,
            Without<GarbageText>,
            Without<FishText>,
        ),
    >,
    mut garbage_text: Query<
        &mut Text,
        (
            With<GarbageText>,
            Without<ClockTime>,
            Without<ClockIcon>,
            Without<EnergyBar>,
            Without<ScoreBar>,
            Without<WeatherIcon>,
            Without<FutureWeatherIcon>,
            Without<CoinText>,
            Without<RockText>,
            Without<TreeText>,
            Without<WaterText>,
            Without<FishText>,
        ),
    >,
    mut fish_text: Query<
        &mut Text,
        (
            With<FishText>,
            Without<ClockTime>,
            Without<ClockIcon>,
            Without<EnergyBar>,
            Without<ScoreBar>,
            Without<WeatherIcon>,
            Without<FutureWeatherIcon>,
            Without<CoinText>,
            Without<RockText>,
            Without<TreeText>,
            Without<WaterText>,
            Without<GarbageText>,
        ),
    >,
    runner: NonSend<RobotResource>,
    time: Res<Time>,
    mut query: Query<&mut TextUpdate>,
) {
    if timer_finished(&time, &mut query) {
        update_backpack(
            &mut coin_text,
            &mut rock_text,
            &mut tree_text,
            &mut water_text,
            &mut garbage_text,
            &mut fish_text,
            &runner,
        );
        update_energy(&mut energy_bar, &runner);
        update_score(&mut score_bar);
        update_weather(
            &mut clock_time,
            &mut weather_icon,
            &mut clock_icon,
            &mut future_weather_icon,
        );
    }
}

fn update_weather(
    clock_time: &mut Query<
        &mut Text,
        (
            With<ClockTime>,
            Without<ClockIcon>,
            Without<EnergyBar>,
            Without<ScoreBar>,
            Without<WeatherIcon>,
            Without<FutureWeatherIcon>,
            Without<CoinText>,
            Without<RockText>,
            Without<TreeText>,
            Without<WaterText>,
            Without<GarbageText>,
            Without<FishText>,
        ),
    >,
    weather_icon: &mut Query<(
        &mut UiTextureAtlasImage,
        (
            With<WeatherIcon>,
            Without<ClockTime>,
            Without<ClockIcon>,
            Without<EnergyBar>,
            Without<ScoreBar>,
            Without<FutureWeatherIcon>,
            Without<CoinText>,
            Without<RockText>,
            Without<TreeText>,
            Without<WaterText>,
            Without<GarbageText>,
            Without<FishText>,
        ),
    )>,
    clock_icon: &mut Query<(
        &mut UiTextureAtlasImage,
        (
            With<ClockIcon>,
            Without<ClockTime>,
            Without<EnergyBar>,
            Without<ScoreBar>,
            Without<WeatherIcon>,
            Without<FutureWeatherIcon>,
            Without<CoinText>,
            Without<RockText>,
            Without<TreeText>,
            Without<WaterText>,
            Without<GarbageText>,
            Without<FishText>,
        ),
    )>,
    future_weather_icon: &mut Query<(
        &mut UiTextureAtlasImage,
        (
            With<FutureWeatherIcon>,
            Without<ClockTime>,
            Without<ClockIcon>,
            Without<EnergyBar>,
            Without<ScoreBar>,
            Without<WeatherIcon>,
            Without<CoinText>,
            Without<RockText>,
            Without<TreeText>,
            Without<WaterText>,
            Without<GarbageText>,
            Without<FishText>,
        ),
    )>,
) {
    let time;
    let weather;
    let future_weather;
    {
        let env = ENVIRONMENT.lock().unwrap();
        time = env.as_ref().unwrap().get_time_of_day_string();

        weather = env.as_ref().unwrap().get_weather_condition();
        match (*FUTUREENVIRONMENT.lock().unwrap()).as_mut() {
            Some(x) => future_weather = x.clone(),
            None => future_weather = weather.clone(),
        }
    }

    for mut icon in weather_icon.iter_mut() {
        icon.0.index = get_weather_asset_number(&weather);
    }
    for mut icon in future_weather_icon.iter_mut() {
        icon.0.index = get_weather_asset_number(&future_weather);
    }
    for mut text in clock_time.iter_mut() {
        text.sections[0].value = time.clone();
    }
    for mut icon in clock_icon.iter_mut() {
        icon.0.index = get_time_asset_number(&time);
    }
}

fn update_score(
    score_bar: &mut Query<
        &mut ProgressBar,
        (
            With<ScoreBar>,
            Without<ClockTime>,
            Without<ClockIcon>,
            Without<EnergyBar>,
            Without<WeatherIcon>,
            Without<FutureWeatherIcon>,
            Without<CoinText>,
            Without<RockText>,
            Without<TreeText>,
            Without<WaterText>,
            Without<GarbageText>,
            Without<FishText>,
        ),
    >,
) {
    let score = SCORE.lock().unwrap();

    for mut x in score_bar.iter_mut() {
        x.set_progress(*score / 100.);
    }
}

fn update_energy(
    energy_bar: &mut Query<
        &mut ProgressBar,
        (
            With<EnergyBar>,
            Without<ClockTime>,
            Without<ClockIcon>,
            Without<ScoreBar>,
            Without<WeatherIcon>,
            Without<FutureWeatherIcon>,
            Without<CoinText>,
            Without<RockText>,
            Without<TreeText>,
            Without<WaterText>,
            Without<GarbageText>,
            Without<FishText>,
        ),
    >,
    runner: &NonSend<RobotResource>,
) {
    let energy_level = runner.runner.get_robot().get_energy().get_energy_level();
    for mut x in energy_bar.iter_mut() {
        x.set_progress(energy_level as f32 / 1000.0);
    }
}

fn update_backpack(
    mut coin_text: &mut Query<
        &mut Text,
        (
            With<CoinText>,
            Without<ClockTime>,
            Without<ClockIcon>,
            Without<EnergyBar>,
            Without<ScoreBar>,
            Without<WeatherIcon>,
            Without<FutureWeatherIcon>,
            Without<RockText>,
            Without<TreeText>,
            Without<WaterText>,
            Without<GarbageText>,
            Without<FishText>,
        ),
    >,
    mut rock_text: &mut Query<
        &mut Text,
        (
            With<RockText>,
            Without<ClockTime>,
            Without<ClockIcon>,
            Without<EnergyBar>,
            Without<ScoreBar>,
            Without<WeatherIcon>,
            Without<FutureWeatherIcon>,
            Without<CoinText>,
            Without<TreeText>,
            Without<WaterText>,
            Without<GarbageText>,
            Without<FishText>,
        ),
    >,
    mut tree_text: &mut Query<
        &mut Text,
        (
            With<TreeText>,
            Without<ClockTime>,
            Without<ClockIcon>,
            Without<EnergyBar>,
            Without<ScoreBar>,
            Without<WeatherIcon>,
            Without<FutureWeatherIcon>,
            Without<CoinText>,
            Without<RockText>,
            Without<WaterText>,
            Without<GarbageText>,
            Without<FishText>,
        ),
    >,
    mut water_text: &mut Query<
        &mut Text,
        (
            With<WaterText>,
            Without<ClockTime>,
            Without<ClockIcon>,
            Without<EnergyBar>,
            Without<ScoreBar>,
            Without<WeatherIcon>,
            Without<FutureWeatherIcon>,
            Without<CoinText>,
            Without<RockText>,
            Without<TreeText>,
            Without<GarbageText>,
            Without<FishText>,
        ),
    >,
    mut garbage_text: &mut Query<
        &mut Text,
        (
            With<GarbageText>,
            Without<ClockTime>,
            Without<ClockIcon>,
            Without<EnergyBar>,
            Without<ScoreBar>,
            Without<WeatherIcon>,
            Without<FutureWeatherIcon>,
            Without<CoinText>,
            Without<RockText>,
            Without<TreeText>,
            Without<WaterText>,
            Without<FishText>,
        ),
    >,
    mut fish_text: &mut Query<
        &mut Text,
        (
            With<FishText>,
            Without<ClockTime>,
            Without<ClockIcon>,
            Without<EnergyBar>,
            Without<ScoreBar>,
            Without<WeatherIcon>,
            Without<FutureWeatherIcon>,
            Without<CoinText>,
            Without<RockText>,
            Without<TreeText>,
            Without<WaterText>,
            Without<GarbageText>,
        ),
    >,
    runner: &NonSend<RobotResource>,
) {
    for (content, number) in runner.runner.get_robot().get_backpack().get_contents() {
        match content {
            Content::Rock(_) => {
                update_rock_text(&mut rock_text, *number);
            }
            Content::Tree(_) => {
                update_tree_text(&mut tree_text, *number);
            }
            Content::Garbage(_) => {
                update_garbage_text(&mut garbage_text, *number);
            }
            Content::Coin(_) => {
                update_coin_text(&mut coin_text, *number);
            }
            Content::Water(_) => {
                update_water_text(&mut water_text, *number);
            }
            Content::Fish(_) => {
                update_fish_text(&mut fish_text, *number);
            }
            _ => {}
        }
    }
}

fn timer_finished(time: &Res<Time>, query: &mut Query<&mut TextUpdate>) -> bool {
    for mut timer in &mut query.iter_mut() {
        timer.tick(time.delta());
        return timer.just_finished();
    }
    return false;
}

fn update_coin_text(
    coin_text: &mut Query<
        &mut Text,
        (
            With<CoinText>,
            Without<ClockTime>,
            Without<ClockIcon>,
            Without<EnergyBar>,
            Without<ScoreBar>,
            Without<WeatherIcon>,
            Without<FutureWeatherIcon>,
            Without<RockText>,
            Without<TreeText>,
            Without<WaterText>,
            Without<GarbageText>,
            Without<FishText>,
        ),
    >,
    number: usize,
) {
    for mut text in coin_text.iter_mut() {
        if text.sections[0].value != number.to_string() {
            text.sections[0].value = number.to_string();
        }
    }
}

pub(crate) fn update_rock_text(
    rock_text: &mut Query<
        &mut Text,
        (
            With<RockText>,
            Without<ClockTime>,
            Without<ClockIcon>,
            Without<EnergyBar>,
            Without<ScoreBar>,
            Without<WeatherIcon>,
            Without<FutureWeatherIcon>,
            Without<CoinText>,
            Without<TreeText>,
            Without<WaterText>,
            Without<GarbageText>,
            Without<FishText>,
        ),
    >,
    number: usize,
) {
    let mut text = rock_text.single_mut();
    if text.sections[0].value != number.to_string() {
        text.sections[0].value = number.to_string();
    }
}

pub(crate) fn update_water_text(
    water_text: &mut Query<
        &mut Text,
        (
            With<WaterText>,
            Without<ClockTime>,
            Without<ClockIcon>,
            Without<EnergyBar>,
            Without<ScoreBar>,
            Without<WeatherIcon>,
            Without<FutureWeatherIcon>,
            Without<CoinText>,
            Without<RockText>,
            Without<TreeText>,
            Without<GarbageText>,
            Without<FishText>,
        ),
    >,
    number: usize,
) {
    let mut text = water_text.single_mut();
    if text.sections[0].value != number.to_string() {
        text.sections[0].value = number.to_string();
    }
}

pub(crate) fn update_garbage_text(
    garbage_text: &mut Query<
        &mut Text,
        (
            With<GarbageText>,
            Without<ClockTime>,
            Without<ClockIcon>,
            Without<EnergyBar>,
            Without<ScoreBar>,
            Without<WeatherIcon>,
            Without<FutureWeatherIcon>,
            Without<CoinText>,
            Without<RockText>,
            Without<TreeText>,
            Without<WaterText>,
            Without<FishText>,
        ),
    >,
    number: usize,
) {
    let mut text = garbage_text.single_mut();
    if text.sections[0].value != number.to_string() {
        text.sections[0].value = number.to_string();
    }
}
pub(crate) fn update_fish_text(
    fish_text: &mut Query<
        &mut Text,
        (
            With<FishText>,
            Without<ClockTime>,
            Without<ClockIcon>,
            Without<EnergyBar>,
            Without<ScoreBar>,
            Without<WeatherIcon>,
            Without<FutureWeatherIcon>,
            Without<CoinText>,
            Without<RockText>,
            Without<TreeText>,
            Without<WaterText>,
            Without<GarbageText>,
        ),
    >,
    number: usize,
) {
    let mut text = fish_text.single_mut();
    if text.sections[0].value != number.to_string() {
        text.sections[0].value = number.to_string();
    }
}

pub(crate) fn update_tree_text(
    tree_text: &mut Query<
        &mut Text,
        (
            With<TreeText>,
            Without<ClockTime>,
            Without<ClockIcon>,
            Without<EnergyBar>,
            Without<ScoreBar>,
            Without<WeatherIcon>,
            Without<FutureWeatherIcon>,
            Without<CoinText>,
            Without<RockText>,
            Without<WaterText>,
            Without<GarbageText>,
            Without<FishText>,
        ),
    >,
    number: usize,
) {
    let mut text = tree_text.single_mut();
    if text.sections[0].value != number.to_string() {
        text.sections[0].value = number.to_string();
    }
}
