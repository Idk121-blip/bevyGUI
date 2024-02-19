mod alessio_gui;

mod components;

mod alessandro_gui;
mod robot;

use crate::alessandro_gui::main::run;
use crate::components::MAP_DIMENSION;
use crate::robot::Graphics;
use crate::robot::MyRobot;
use crate::utils::make_map;
use crate::utils::utils_for_ai::{robot_audio, ui_runner};
use alessio_gui::gui;
use alessio_gui::utils;
use exclusion_zone::content::bank::BankSettings;
use exclusion_zone::content::bin::BinSettings;
use exclusion_zone::content::coin::CoinSettings;
use exclusion_zone::content::fire::FireSettings;
use exclusion_zone::content::fish::FishSettings;
use exclusion_zone::content::garbage::GarbageSettings;
use exclusion_zone::content::market::MarketSettings;
use exclusion_zone::content::rock::RockSettings;
use exclusion_zone::content::tree::TreeSettings;
use exclusion_zone::content::wood_crate::CrateSettings;
use exclusion_zone::generator::{
    get_default_spawn_order, NoiseSettings, Thresholds, WorldGenerator,
};
use exclusion_zone::tile_type::lava::LavaSettings;
use robotics_lib;
use robotics_lib::runner::{Robot, Runner};
use std::io;

fn main() {
    let my_banana;
    let robot;

    let world_size = 101;
    let mut world_generator = WorldGenerator::new(
        world_size,
        get_default_spawn_order(),
        NoiseSettings::default(),
        Thresholds::default(),
        LavaSettings::default(world_size),
        BankSettings::default(world_size),
        BinSettings::default(world_size),
        CrateSettings::default(world_size),
        GarbageSettings::default(world_size),
        FireSettings::default(world_size),
        TreeSettings::default(world_size),
        CoinSettings::default(world_size),
        MarketSettings::default(world_size),
        FishSettings::default(world_size),
        RockSettings::default(world_size),
    );

    println!(
        "Select which graphic you would like to use:
    -1: Alessio's one
    -2: Alessandro's one"
    );

    let mut input_value = String::new();
    io::stdin()
        .read_line(&mut input_value)
        .expect("Failed to read line");

    let input_value: i32 = input_value.trim().parse().unwrap_or_else(|_| {
        println!("Invalid input. Defaulting to 1");
        1
    });
    let graphics;

    if input_value == 1 {
        graphics = Graphics::Alessio;
    } else {
        graphics = Graphics::Alessandro;
    }

    my_banana = MyRobot {
        robot: Robot::new(),
        audio: robot_audio(&graphics),
        weather_prediction: ohcrab_weather::weather_tool::WeatherPredictionTool::new(),
        route_planner: rastanidoumen_route_planner::tool::RoutePlanner::default(),
        graphics: graphics.clone(),
    };
    robot = Runner::new(Box::new(my_banana), &mut world_generator);

    match robot {
        Ok(runner) => match graphics {
            Graphics::Alessio => {
                alessio_graphics(world_size, runner);
            }
            Graphics::Alessandro => {
                alessandro_gui(runner);
            }
        },
        Err(x) => {
            println!("Something went wrong!");
            println!("{:?}", x);
        }
    }
}

fn alessio_graphics(map_dim: usize, robot: Runner) {
    {
        let mut map_dimension = MAP_DIMENSION.lock().unwrap();
        *map_dimension = map_dim;
        make_map(*map_dimension);
    }
    ui_runner(robot)
}

fn alessandro_gui(robot: Runner) {
    run(robot)
}
