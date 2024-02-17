mod gui;
mod robot;

mod components;
mod utils;
mod utils_for_ai;
use crate::components::MAP_DIMENSION;
use crate::robot::MyRobot;
use crate::utils::make_map;
use crate::utils_for_ai::robot_audio;
use crate::utils_for_ai::ui_runner;
pub(crate) use bevy::prelude::*;
use oxagworldgenerator::world_generator::world_generator_builder::OxAgWorldGeneratorBuilder;
use oxagworldgenerator::world_generator::OxAgWorldGenerator;
use robotics_lib;
use robotics_lib::runner::{Robot, Runner};
use robotics_lib::world::world_generator::Generator;

fn main() {
    let my_banana;
    let robot;

    let mut generator = OxAgWorldGenerator::builder().build().unwrap();
    /*
        use robotics_lib::world::world_generator::Generator;
        use exclusion_zone::content::bank::BankSettings;
        use exclusion_zone::content::bin::BinSettings;
        use exclusion_zone::content::fire::FireSettings;
        use exclusion_zone::content::garbage::GarbageSettings;
        use exclusion_zone::content::tree::TreeSettings;
        use exclusion_zone::content::wood_crate::CrateSettings;
        use exclusion_zone::generator::{get_default_spawn_order, NoiseSettings, Thresholds, WorldGenerator};
        use exclusion_zone::tile_type::lava::LavaSettings;

        let world_size = 1000;

        let mut world_generator = WorldGenerator::new(
        world_size,
        get_default_spawn_order(),
        NoiseSettings::default(),
        Thresholds::def(),
        LavaSettings::default(world_size),
        BankSettings::default(world_size),
        BinSettings::default(world_size),
        CrateSettings::default(world_size),
        GarbageSettings::default(world_size),
        FireSettings::default(world_size),
        TreeSettings::default(world_size),
    );

        */

    {
        let mut map_dimension = MAP_DIMENSION.lock().unwrap();
        *map_dimension = generator.get_size();
        my_banana = MyRobot {
            robot: Robot::new(),
            audio: robot_audio(),
            weather_prediction: ohcrab_weather::weather_tool::WeatherPredictionTool::new(),
        };
        make_map(*map_dimension);
        robot = Runner::new(Box::new(my_banana), &mut generator);
    }

    match robot {
        Ok(runner) => {
            ui_runner(runner);
        }
        Err(x) => {
            println!("Something went wrong!");
            println!("{:?}", x);
        }
    }
}
